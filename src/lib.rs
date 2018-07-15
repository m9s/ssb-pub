extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate tokio;

mod config;
mod web;

pub use config::Config;

use actix::Arbiter;
use futures::prelude::*;
use slog::Logger;
use std::io;
use std::sync::Arc;
use tokio::net::TcpStream;

pub fn start<S>(config: Config, logger: &Logger, stream: S)
where S: Stream<Item = TcpStream, Error = io::Error> + Sized + 'static
{
    let config = Arc::new(config);

    let web_app = web::start(config.clone(), logger)
        .sink_map_err(|_| ());

    let pipe = stream
        .map_err(|_| ())
        .forward(web_app)
        .map(|_| ());
    Arbiter::spawn(pipe);

    info!(logger, "Application started");
    info!(logger, "MOTD is {}", config.web.motd);
}
