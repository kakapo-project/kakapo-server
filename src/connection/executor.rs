
use std::env;
use std::fmt;
use std::path::PathBuf;

use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use connection::AppStateBuilder;

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Secrets {
    pub token_secret: String,
    pub password_secret: String,
}

pub struct Executor {
    pool: Pool<ConnectionManager<PgConnection>>,
    script_path: PathBuf,
    secrets: Secrets,

    pub jwt_issuer: String,
    pub jwt_token_duration: i64,
    pub jwt_refresh_token_duration: i64,
}

impl fmt::Debug for Executor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Executor")
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

        let script_path = match info.script_path_dir {
            Some(dir) => PathBuf::from(dir),
            None => kakapo_script_home(),
        };

        let secrets = Secrets {
            token_secret: info.token_secret_key.unwrap_or_default(),
            password_secret: info.password_secret_key.unwrap_or_default(),
        };

        Self {
            pool,
            script_path,
            secrets,

            jwt_issuer: info.jwt_issuer.unwrap_or_default(), //TODO: what is the default here?
            jwt_token_duration: info.jwt_token_duration,
            jwt_refresh_token_duration: info.jwt_refresh_token_duration,
        }
    }

    pub fn get_scripts_path(&self) -> PathBuf {
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


fn kakapo_home() -> PathBuf {
    let mut home = dirs::home_dir().unwrap_or(PathBuf::from("/var/kakapo/"));
    home.push(".kakapo");
    home
}

fn kakapo_script_home() -> PathBuf {
    let mut kakapo_home = kakapo_home();
    kakapo_home.push("scripts");
    kakapo_home
}