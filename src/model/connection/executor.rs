
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use actix::sync::SyncArbiter;
use num_cpus;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl DatabaseExecutor {
    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.0.get()
            .expect("Could not get connection")
    }
}

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}


pub fn create(database_url: &str) -> Addr<DatabaseExecutor> {

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)
        .expect("Could not start connection");

    SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()))
}


//TODO: do I need to impl Drop?