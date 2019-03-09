
pub mod connector;
pub mod utils;
mod methods;
mod table;
mod query;
mod database;
mod data;
mod update_state;


#[derive(Clone)]
pub struct KakapoPostgres {
    pub user: String,
    pub pass: String,
    pub host: String,
    pub port: u16,
    pub db: String,
}

impl KakapoPostgres {
    pub fn new() -> Self {
        Self {
            user: "postgres".to_string(),
            pass: "".to_string(),
            host: "127.0.0.1".to_string(),
            port: 5432,
            db: "postgres".to_string(),
        }
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = user.to_string();
        self
    }

    pub fn pass(mut self, pass: &str) -> Self {
        self.pass = pass.to_string();
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn db(mut self, db: &str) -> Self {
        self.db = db.to_string();
        self
    }
}