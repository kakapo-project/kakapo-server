
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