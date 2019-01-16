

use actix::prelude::*;

use server::action_wrapper::DatabaseExecutor;

#[derive(Clone)]
pub struct AppState {
    db_connections: Addr<DatabaseExecutor>,
    pub app_name: String,
}

impl AppState {
    pub fn new(connections: Addr<DatabaseExecutor>, script_path: &str, app_name: &str) -> Self {
        AppState {
            db_connections: connections,
            app_name: app_name.to_string(),
        }
    }

    pub fn connect<'a>(&'a self) -> &'a Addr<DatabaseExecutor> {
        &self.db_connections
    }

}
