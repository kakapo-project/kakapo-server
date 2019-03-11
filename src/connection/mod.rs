
pub mod executor;
pub mod domain;

use num_cpus;

use std::sync::Arc;
use std::fmt::Debug;
use std::collections::HashMap;

use actix::Addr;
use actix::sync::SyncArbiter;

use data::channels::Channels;

use plugins::v1::DomainBuilder;
use plugins::v1::Domain;

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
pub struct AppStateBuilder {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    pass: Option<String>,
    db: Option<String>,
    script_path: Option<String>,
    token_secret: Option<String>,
    password_secret: Option<String>,
    jwt_issuer: Option<String>,
    jwt_token_duration: i64,
    jwt_refresh_token_duration: i64,
    num_threads: usize,

    domain_builders: HashMap<String, Box<DomainBuilder>>,
}

/// Example Usage
///
impl AppStateBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            user: None,
            pass: None,
            db: None,
            script_path: None,
            token_secret: None,
            password_secret: None,
            jwt_issuer: None,
            jwt_token_duration: 600,
            jwt_refresh_token_duration: 60 * 60 * 24,
            num_threads: num_cpus::get(),

            domain_builders: HashMap::new(),
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn pass(mut self, pass: &str) -> Self {
        self.pass = Some(pass.to_string());
        self
    }

    pub fn db(mut self, db: &str) -> Self {
        self.db = Some(db.to_string());
        self
    }

    pub fn script_path(mut self, script_path: &str) -> Self {
        self.script_path = Some(script_path.to_string());
        self
    }

    pub fn token_secret(mut self, token_secret: &str) -> Self {
        self.token_secret = Some(token_secret.to_string());
        self
    }

    pub fn password_secret(mut self, password_secret: &str) -> Self {
        self.password_secret = Some(password_secret.to_string());
        self
    }

    pub fn issuer(mut self, issuer: &str) -> Self {
        self.jwt_issuer = Some(issuer.to_string());
        self
    }

    pub fn token_duration(mut self, token_duration: i64) -> Self {
        self.jwt_token_duration = token_duration;
        self
    }

    pub fn refresh_token_duration(mut self, refresh_token_duration: i64) -> Self {
        self.jwt_refresh_token_duration = refresh_token_duration;
        self
    }

    pub fn num_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads;
        self
    }

    pub fn add_plugin<HD>(mut self, name: &str, domain_builder: HD) -> Self
        where
            HD: DomainBuilder + 'static,
    {
        self.domain_builders.insert(name.to_string(), Box::new(domain_builder));
        self
    }

    pub fn done(self) -> AppState {
        let token_secret = self.token_secret.clone()
            .expect("Must specify a token secret");
        let password_secret = self.password_secret.clone()
            .expect("Must specify a password secret");
        let threads = self.num_threads;

        info!("Staring database connection");
        let connections = SyncArbiter::start(
            threads,
            move || executor::Executor::create(&self));



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
