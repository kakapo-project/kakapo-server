
use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use actix::sync::SyncArbiter;
use num_cpus;

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DatabaseExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
    script_path: String,
}

impl DatabaseExecutor {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, script_path: String) -> Self {
        Self {
            pool,
            script_path
        }
    }

    pub fn get_connection(&self) -> Conn {
        self.pool.get()
            .expect("Could not get connection")
    }

    pub fn get_scripts_path(&self) -> String {
        self.script_path.to_owned()
    }
}

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

pub struct Connector {
    host_name: Option<String>,
    port_name: Option<u16>,
    user_name: Option<String>,
    pass_name: Option<String>,
    db_name: Option<String>,
    script_path_dir: Option<String>,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            host_name: None,
            port_name: None,
            user_name: None,
            pass_name: None,
            db_name: None,
            script_path_dir: None,
        }
    }

    pub fn host(mut self, param: String) -> Self {
        self.host_name = Some(param);
        self
    }

    pub fn port(mut self, param: u16) -> Self {
        self.port_name = Some(param);
        self
    }

    pub fn user(mut self, param: String) -> Self {
        self.user_name = Some(param);
        self
    }

    pub fn pass(mut self, param: String) -> Self {
        self.pass_name = Some(param);
        self
    }

    pub fn db(mut self, param: String) -> Self {
        self.db_name = Some(param);
        self
    }

    pub fn script_path(mut self, script_path_dir: String) -> Self {
        self.script_path_dir = Some(script_path_dir);
        self
    }

    pub fn done(self) -> Addr<DatabaseExecutor> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user_name.unwrap_or_default(),
            self.pass_name.unwrap_or_default(),
            self.host_name.unwrap_or_default(),
            self.port_name.unwrap_or_default(),
            self.db_name.unwrap_or_default(),
        );
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        let script_path = self.script_path_dir.unwrap_or_default(); //TODO: should fallback to home
        SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor::new(pool.clone(), script_path.clone()))
    }
}
