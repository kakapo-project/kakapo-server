
use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use connection::AppStateBuilder;
use std::fmt;

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Secrets {
    pub token_secret: String,
    pub password_secret: String,
}

pub struct Executor {
    pool: Pool<ConnectionManager<PgConnection>>,
    script_path: String,
    secrets: Secrets,
}

impl fmt::Debug for Executor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Executor ...")
    }
}

impl Executor {
    pub fn get_connection(&self) -> Conn {
        self.pool.get()
            .expect("Could not get connection")
    }

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
        let secrets = Secrets {
            token_secret: info.token_secret_key.unwrap_or_default(),
            password_secret: info.password_secret_key.unwrap_or_default(),
        };

        Self {
            pool,
            script_path,
            secrets,
        }
    }

    pub fn get_scripts_path(&self) -> String {
        self.script_path.to_owned()
    }

    pub fn get_token_secret(&self) -> String {
        self.secrets.token_secret.to_owned()
    }

    pub fn get_secrets(&self) -> Secrets {
        self.secrets.to_owned()
    }
}

impl Actor for Executor {
    type Context = SyncContext<Self>;
}


