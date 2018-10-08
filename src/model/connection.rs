
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use actix::prelude::*;
use diesel::{prelude::*, r2d2::ConnectionManager};
use diesel::r2d2::Pool;
use actix::sync::SyncArbiter;
use num_cpus;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}


pub fn create() -> Addr<DatabaseExecutor> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)
        .expect("Could not start connection");

    SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()))
}
