
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

pub struct State {
    database: DBConnector, //TODO: this should be templated
    //user
}

impl State {
    pub fn new(
        database: DBConnector,
    ) -> Self {
        Self {
            database,
        }
    }
}

pub trait GetConnection
    where Self: Send
{
    type Connection;
    fn get_conn<'a>(&'a self) -> &'a Self::Connection;

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where
            F: FnOnce() -> Result<G, E>,
            E: From<diesel::result::Error>;
}

impl GetConnection for State {
    type Connection = Conn;
    fn get_conn<'a>(&'a self) -> &'a Conn {
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

pub trait GetUserInfo
    where Self: Send
{
    fn get_user_id(&self) -> i64;
}

impl GetUserInfo for State {
    fn get_user_id(&self) -> i64 { 1 }
}