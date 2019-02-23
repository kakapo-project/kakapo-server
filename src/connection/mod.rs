
pub mod executor;
pub mod py;
pub mod publisher;

use num_cpus;

use std::sync::Arc;
use std::fmt::Debug;

use actix::Addr;
use actix::sync::SyncArbiter;

use data::channels::Channels;

pub trait GetSecrets {
    fn get_token_secret(&self) -> String;
    fn get_password_secret(&self) -> String;
}

pub trait AppStateLike: GetSecrets {
    fn connect(&self) -> &Addr<executor::Executor>;
}

#[derive(Debug, Clone)]
pub struct AppState {
    connections: Addr<executor::Executor>,
    token_secret: String, //This is duplicated here as well as inside the executor , because we need it both in the view (websocket) and in the model
    password_secret: String, // TODO: find a better way
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
    jwt_issuer: Option<String>,
    jwt_token_duration: i64,
    jwt_refresh_token_duration: i64,
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
            jwt_issuer: None,
            jwt_token_duration: 600,
            jwt_refresh_token_duration: 60 * 60 * 24,
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

    pub fn issuer(mut self, iss: &str) -> Self {
        self.jwt_issuer = Some(iss.to_string());
        self
    }

    pub fn token_duration(mut self, duration: i64) -> Self {
        self.jwt_token_duration = duration;
        self
    }

    pub fn refresh_token_duration(mut self, duration: i64) -> Self {
        self.jwt_refresh_token_duration = duration;
        self
    }

    pub fn num_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn done(self) -> AppState {
        let token_secret = self.token_secret_key.clone().unwrap_or_default();
        let password_secret = self.password_secret_key.clone().unwrap_or_default();

        let connections = SyncArbiter::start(
            self.threads,
            move || executor::Executor::create(self.clone()));



        AppState {
            connections,
            token_secret,
            password_secret,
        }
    }
}


impl AppStateLike for AppState {
    fn connect(&self) -> &Addr<executor::Executor> {
        &self.connections
    }
}

impl GetSecrets for AppState {
    fn get_token_secret(&self) -> String {
        self.token_secret.to_owned()
    }

    fn get_password_secret(&self) -> String {
        self.password_secret.to_owned()
    }
}
