
use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use connection::AppStateBuilder;

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct Executor {
    pool: Pool<ConnectionManager<PgConnection>>,
    script_path: String,
    token_secret: String,
}

impl Executor {
    pub fn create(info: AppStateBuilder) -> Self {

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            info.user_name.unwrap_or_default(),
            info.pass_name.unwrap_or_default(),
            info.host_name.unwrap_or_default(),
            info.port_name.unwrap_or_default(),
            info.db_name.unwrap_or_default(),
        );
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        let script_path = info.script_path_dir.unwrap_or_default(); //TODO: should fallback to home
        let token_secret = info.secret.unwrap_or("HELLO_WORLD_HELLO_WORLD".to_string()); //TODO: should fallback to randomly generated

        Self {
            pool,
            script_path,
            token_secret,
        }
    }

    pub fn get_connection(&self) -> Conn {
        self.pool.get()
            .expect("Could not get connection")
    }

    pub fn get_scripts_path(&self) -> String {
        self.script_path.to_owned()
    }

    pub fn get_token_secret(&self) -> String {
        self.token_secret.to_owned()
    }
}

impl Actor for Executor {
    type Context = SyncContext<Self>;
}


