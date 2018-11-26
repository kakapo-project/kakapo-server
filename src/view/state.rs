

use actix::prelude::*;


use model::{api, connection, connection::DatabaseExecutor};

#[derive(Clone)]
pub struct AppState {
    db_connections: Vec<Addr<DatabaseExecutor>>,
    pub app_name: String,
}

impl AppState {
    pub fn new(connections: Vec<Addr<DatabaseExecutor>>, app_name: &str) -> Self {
        AppState {
            db_connections: connections,
            app_name: app_name.to_string(),
        }
    }

    pub fn connect<'a>(&'a self, idx: usize) -> &'a Addr<DatabaseExecutor> {
        &self.db_connections[idx]
    }
}
