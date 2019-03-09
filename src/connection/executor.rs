
use std::env;
use std::fmt;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::Arc;

use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use connection::AppStateBuilder;

use plugins::v1::Domain;
use plugins::v1::Datastore;
use plugins::v1::DataQuery;

#[derive(Debug, Fail, Serialize)]
pub enum DomainError {
    #[fail(display = "domain {} does not exist", 0)]
    DomainNotFound(String),
    #[fail(display = "domain does not support datastore operations")]
    DatastoreNotAvailable,
    #[fail(display = "domain does not support query operations")]
    QueryNotAvailable,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

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

    domains: HashMap<String, Box<Domain>>,

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

    pub fn get_datastore_conn(&self, domain_name: &str) -> Result<Box<Datastore>, DomainError> {
        let datastore = self.domains
            .get(domain_name)
            .ok_or_else(|| DomainError::DomainNotFound(domain_name.to_string()))?
            .connect_datastore()
            .ok_or_else(|| DomainError::DatastoreNotAvailable)?;

        Ok(datastore)
    }

    pub fn get_query_conn(&self, domain_name: &str) -> Result<Box<DataQuery>, DomainError> {
        let dataquery = self.domains
            .get(domain_name)
            .ok_or_else(|| DomainError::DomainNotFound(domain_name.to_string()))?
            .connect_query()
            .ok_or_else(|| DomainError::DatastoreNotAvailable)?;

        Ok(dataquery)
    }

    pub fn create(info: &AppStateBuilder) -> Self {

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            info.user.clone().unwrap_or_default(),
            info.pass.clone().unwrap_or_default(),
            info.host.clone().unwrap_or_default(),
            info.port.clone().unwrap_or_default(),
            info.db.clone().unwrap_or_default(),
        );
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        let script_path = match info.script_path.clone() {
            Some(dir) => PathBuf::from(dir),
            None => kakapo_script_home(),
        };

        let secrets = Secrets {
            token_secret: info.token_secret.clone().unwrap_or_default(),
            password_secret: info.password_secret.clone().unwrap_or_default(),
        };

        let mut domains = HashMap::new();
        for (key, value) in info.domain_builders.iter() {
            domains.insert(key.clone(), value.build());
        }

        Self {
            pool,
            script_path,
            secrets,

            domains,

            jwt_issuer: info.jwt_issuer.clone().unwrap_or_default(), //TODO: what is the default here?
            jwt_token_duration: info.jwt_token_duration.clone(),
            jwt_refresh_token_duration: info.jwt_refresh_token_duration.clone(),
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