use actix::Addr;

pub mod executor;
pub mod py;
use actix::sync::SyncArbiter;
use num_cpus;
use std::sync::Arc;
pub use model::state::Channels;
use std::fmt::Debug;

///Put this somewhere in your State
#[derive(Debug, Clone)]
pub struct AppState {
    connections: Addr<executor::Executor>,
}

#[derive(Debug, Fail)]
pub enum BroadcasterError {
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

///Implement this for your state for broadcasting info
pub trait Broadcaster
    where Self: Send + Debug + Sync + 'static
{
    fn publish(&self, channels: Vec<Channels>, action_name: String, action_result: serde_json::Value) -> Result<(), BroadcasterError>;
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
    threads: usize,
}

/// Implement this for your state
pub trait GetAppState<B>
    where
        B: Broadcaster,
{
    fn get_app_state(&self) -> &AppState;

    fn get_broadcaster(&self) -> Arc<Broadcaster>;
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
            secret: None,
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
        self.secret = Some(secret.to_string());
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


impl AppState {
    pub fn connect(&self) -> &Addr<executor::Executor> {
        &self.connections
    }
}

