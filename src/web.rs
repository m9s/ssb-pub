use ::config::Config;
use actix_web::{server, App, HttpRequest, Responder};
use tokio::prelude::*;
use futures::sync::mpsc;
use slog::Logger;
use std::io;
use std::sync::Arc;

struct State {
    config: Arc<Config>,
    log: Logger,
}

fn index(req: HttpRequest<Arc<State>>) -> impl Responder {
    debug!(req.state().log, "index method called");
    format!("{}", req.state().config.web.motd)
}

pub fn start<S>(config: Arc<Config>, logger: &Logger) -> mpsc::Sender<S>
where S: AsyncRead + AsyncWrite + Send + 'static,
{
    let (tx, rx) = mpsc::channel(config.web.request_queue);
    let rx = rx.map_err(|_| io::ErrorKind::Other).from_err();
    let log = logger.new(o!("mod" => "web"));
    let state = Arc::new(State {
        config: config,
        log: log,
    });

    server::new(move || App::with_state(state.clone())
                .resource("/", |r| r.with(index)))
        .start_incoming(rx, false);

    tx
}
