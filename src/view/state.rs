

use actix::prelude::*;

use connection::executor::DatabaseExecutor;
use connection::py::PyRunner;

#[derive(Clone)]
pub struct AppState {
    db_connections: Addr<DatabaseExecutor>,
    pub app_name: String,
}

impl AppState {
    pub fn new(connections: Addr<DatabaseExecutor>, app_name: &str) -> Self {
        AppState {
            db_connections: connections,
            app_name: app_name.to_string(),
        }
    }

    pub fn connect(&self) -> &Addr<DatabaseExecutor> {
        &self.db_connections
    }
}
