#[derive(Debug, Deserialize)]
pub struct Config {
    pub web: Web,
}

#[derive(Debug, Deserialize)]
pub struct Web {
    pub motd: String,
    pub request_queue: usize,
}
