
pub mod connector;
mod data;

#[derive(Clone)]
pub struct KakapoRedis {
    //pub pass: String, //TODO: keep pass, and probably need ssl certs too
    pub host: String,
    pub port: u16,
}

impl KakapoRedis {
    pub fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

}