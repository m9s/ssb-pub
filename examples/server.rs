extern crate actix;
extern crate actix_web;
extern crate config;
extern crate listenfd;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
#[macro_use]
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
extern crate ssb_pub;
extern crate tokio;

use listenfd::ListenFd;
use config::{ConfigError, Config, File, FileFormat, Environment};
use slog::{Drain, Logger};
use std::net;
use std::str::FromStr;
use tokio::net::{Incoming, TcpListener};
use tokio::reactor::Handle;

#[derive(Debug, Deserialize)]
struct Server {
    bind: String,
    log_level: String,
}

fn config() -> Result<(Server, ssb_pub::Config), ConfigError> {
    let mut s = Config::new();

    let default = include_str!("default.toml");
    s.merge(File::from_str(default, FileFormat::Toml))?;

    s.merge(File::with_name("config.toml").required(false))?;

    s.merge(Environment::with_prefix("app").separator("__"))?;

    let server: Server = s.get("server")?;

    s.try_into().map(|x| (server, x))
}

fn logging(level: &str) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::LogBuilder::new(drain).parse(level).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, slog_o!("version" => env!("CARGO_PKG_VERSION")))
}

fn listen(bind: &str) -> Incoming {
    let mut listenfd = ListenFd::from_env();

    if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        info!("Listening on environment socket");
        TcpListener::from_std(l, &Handle::current()).unwrap().incoming()
    } else {
        info!("Listening on {}", bind);
        let addr = net::SocketAddr::from_str(bind).unwrap();
        TcpListener::bind(&addr).unwrap().incoming()
    }
}

fn run() -> i32 {
    let sys = actix::System::new("ssb-pub");

    let (server, config) = config().expect("Unable to parse config file");

    let logger = logging(&server.log_level);
    let _scope_guard = slog_scope::set_global_logger(logger);
    let _log_guard = slog_stdlog::init().unwrap();

    let connection_stream = listen(&server.bind);
    ssb_pub::start(config, &slog_scope::logger(), connection_stream);

    sys.run()
}

fn main() {
    let code = run();
    std::process::exit(code);
}
