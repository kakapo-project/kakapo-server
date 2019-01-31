
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


