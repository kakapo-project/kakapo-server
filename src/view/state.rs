

use actix::prelude::*;


use model::{api, connection, connection::DatabaseExecutor};


pub struct AppState {
    pub db_connection: Addr<DatabaseExecutor>,
    pub app_name: String,
}

impl AppState {
    pub fn new(connection: Addr<DatabaseExecutor>, app_name: &str) -> Self {
        AppState {
            db_connection: connection,
            app_name: app_name.to_string(),
        }
    }
}
