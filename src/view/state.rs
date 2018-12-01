

use actix::prelude::*;

use model::{api, connection, connection::executor::DatabaseExecutor, connection::py::PyRunner};
use cpython::{Python, PyDict, PyErr, PyResult, NoArgs};


#[derive(Clone)]
pub struct AppState {
    db_connections: Vec<Addr<DatabaseExecutor>>,
    py_runner: PyRunner,
    pub app_name: String,
}

impl AppState {
    pub fn new(connections: Vec<Addr<DatabaseExecutor>>, script_path: &str, app_name: &str) -> Self {
        AppState {
            db_connections: connections,
            py_runner: PyRunner::new(script_path.to_string()),
            app_name: app_name.to_string(),
        }
    }

    pub fn connect<'a>(&'a self, idx: usize) -> &'a Addr<DatabaseExecutor> {
        &self.db_connections[idx]
    }

    pub fn get_py_runner(&self) -> PyRunner {
        self.py_runner.to_owned()
    }
}
