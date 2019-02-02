
use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use connection::py::PyRunner;

use connection::executor::Conn;

use model::actions::Action;
use diesel::Connection;
use data::dbdata::RawEntityTypes;
use scripting::Scripting;
use std::collections::HashSet;
use std::iter::FromIterator;

type DBConnector = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone, Serialize)]
pub enum Channels {
    AllTables,
    AllQueries,
    AllScripts,
    Table(String),
    Query(String),
    Script(String),
    TableData(String),
}

impl Channels {
    pub fn all_entities<T>() -> Self
        where T: RawEntityTypes,
    {
        Channels::AllTables
    }

    pub fn entity<T>(name: &str) -> Self
        where T: RawEntityTypes,
    {
        Channels::Table(name.to_string())
    }

    pub fn table(table_name: &str) -> Self {
        Channels::TableData(table_name.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthClaims {
    iss: String,
    sub: i64,
    iat: i64,
    exp: i64,
    username: String,
    is_admin: bool,
    roles: Vec<String>,
}

impl AuthClaims {
    pub fn get_user_id(&self) -> i64 {
        self.sub
    }

    pub fn is_user_admin(&self) -> bool {
        self.is_admin
    }

    pub fn get_roles(&self) -> HashSet<String> {
        HashSet::from_iter(self.roles.iter().cloned())
    }
}

pub struct State {
    database: DBConnector, //TODO: this should be templated
    scripting: Scripting,
    claims: Option<AuthClaims>,
}

impl State {
    pub fn new(
        database: DBConnector,
        scripting: Scripting,
        claims: Option<AuthClaims>,
    ) -> Self {
        Self {
            database,
            scripting,
            claims,
        }
    }
}

pub trait GetConnection
    where Self: Send
{
    type Connection;
    fn get_conn(&self) -> &Self::Connection;

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where
            F: FnOnce() -> Result<G, E>,
            E: From<diesel::result::Error>;
}

impl GetConnection for State {
    type Connection = Conn;
    fn get_conn(&self) -> &Conn {
        &self.database
    }

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where
            F: FnOnce() -> Result<G, E>,
            E: From<diesel::result::Error>,
    {
        let conn = self.get_conn();
        conn.transaction::<G, E, _>(f)
    }
}

pub trait GetScripting
    where Self: Send
{
    fn get_scripting(&self) -> &Scripting;
}

impl GetScripting for State {
    fn get_scripting(&self) -> &Scripting {
        &self.scripting
    }
}

pub trait GetUserInfo
    where Self: Send
{
    const ADMIN_USER_ID: i64;

    fn get_user_id(&self) -> Option<i64>;

    fn is_user_admin(&self) -> bool;

    fn get_user_roles(&self) -> Option<HashSet<String>>;

    fn get_db_user(&self) -> String;

}

impl GetUserInfo for State {
    const ADMIN_USER_ID: i64 = 1;

    fn get_user_id(&self) -> Option<i64> {
        self.claims.to_owned().map(|x| x.get_user_id())
    }

    fn is_user_admin(&self) -> bool {
        self.claims.to_owned().map(|x| x.is_user_admin()).unwrap_or(false)
    }

    fn get_user_roles(&self) -> Option<HashSet<String>> {
        self.claims.to_owned().map(|x| x.get_roles())
    }

    fn get_db_user(&self) -> String {
        "my_user".to_string()
    }
}