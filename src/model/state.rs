
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
use connection::Broadcaster;
use std::sync::Arc;

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
    sub: i64, // == user_id
    iat: i64,
    exp: i64,
    username: String,
    is_admin: bool,
    role: Option<String>, //the default role that the user is interacting with
}

impl AuthClaims {
    pub fn get_user_id(&self) -> i64 {
        self.sub
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }

    pub fn is_user_admin(&self) -> bool {
        self.is_admin
    }
}

pub struct State {
    pub database: DBConnector, //TODO: this should be templated
    pub scripting: Scripting,
    pub claims: Option<AuthClaims>,
    pub broadcaster: Arc<Broadcaster>,
}

impl State {
    pub fn new(
        database: DBConnector,
        scripting: Scripting,
        claims: Option<AuthClaims>,
        broadcaster: Arc<Broadcaster>
    ) -> Self {
        Self {
            database,
            scripting,
            claims,
            broadcaster,
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
