
pub mod executor;
pub mod py;
pub mod publisher;

use num_cpus;

use std::sync::Arc;
use std::fmt::Debug;

use actix::Addr;
use actix::sync::SyncArbiter;

use data::channels::Channels;


pub trait AppStateLike {
    fn connect(&self) -> &Addr<executor::Executor>;
}

#[derive(Debug, Clone)]
pub struct AppState {
    connections: Addr<executor::Executor>,
}

/// Builder for the AppState
#[derive(Clone)]
pub struct AppStateBuilder {
    host_name: Option<String>,
    port_name: Option<u16>,
    user_name: Option<String>,
    pass_name: Option<String>,
    db_name: Option<String>,
    script_path_dir: Option<String>,
    token_secret_key: Option<String>,
    password_secret_key: Option<String>,
    threads: usize,
}

/// Example Usage
///
impl AppStateBuilder {
    pub fn new() -> Self {
        Self {
            host_name: None,
            port_name: None,
            user_name: None,
            pass_name: None,
            db_name: None,
            script_path_dir: None,
            token_secret_key: None,
            password_secret_key: None,
            threads: num_cpus::get(),
        }
    }

    pub fn host(mut self, param: &str) -> Self {
        self.host_name = Some(param.to_string());
        self
    }

    pub fn port(mut self, param: u16) -> Self {
        self.port_name = Some(param);
        self
    }

    pub fn user(mut self, param: &str) -> Self {
        self.user_name = Some(param.to_string());
        self
    }

    pub fn pass(mut self, param: &str) -> Self {
        self.pass_name = Some(param.to_string());
        self
    }

    pub fn db(mut self, param: &str) -> Self {
        self.db_name = Some(param.to_string());
        self
    }

    pub fn script_path(mut self, script_path_dir: &str) -> Self {
        self.script_path_dir = Some(script_path_dir.to_string());
        self
    }

    pub fn token_secret(mut self, secret: &str) -> Self {
        self.token_secret_key = Some(secret.to_string());
        self
    }

    pub fn password_secret(mut self, secret: &str) -> Self {
        self.password_secret_key = Some(secret.to_string());
        self
    }

    pub fn num_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn done(self) -> AppState {

        let connections = SyncArbiter::start(
            self.threads,
            move || executor::Executor::create(self.clone()));


        AppState { connections }
    }
}


impl AppStateLike for AppState {
    fn connect(&self) -> &Addr<executor::Executor> {
        &self.connections
    }
}

