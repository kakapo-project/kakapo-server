use actix::Addr;

pub mod executor;
pub mod py;

use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::Pool;
use actix::sync::SyncArbiter;
use num_cpus;

///Put this somewhere in your State
#[derive(Clone)]
pub struct AppState {
    connections: Addr<executor::Executor>,
}

///Implement this for your state for broadcasting info
pub trait Broadcaster {
    fn publish(&self, channels: Vec<String>);
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
    secret: Option<String>,
}

/// Implement this for your state
pub trait GetAppState<B>
    where
        B: Broadcaster,
{
    fn get_app_state(&self) -> &AppState;

    fn get_broadcaster(&self) -> B;
}

impl AppStateBuilder {
    pub fn new() -> Self {
        Self {
            host_name: None,
            port_name: None,
            user_name: None,
            pass_name: None,
            db_name: None,
            script_path_dir: None,
            secret: None,
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
        self.secret = Some(secret.to_string());
        self
    }

    pub fn done(self) -> AppState {

        let connections = SyncArbiter::start(
            num_cpus::get(),
            move || executor::Executor::create(self.clone()));


        AppState { connections }
    }
}


impl AppState {
    pub fn connect(&self) -> &Addr<executor::Executor> {
        &self.connections
    }
}

