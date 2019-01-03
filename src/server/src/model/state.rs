
use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use connection::py::PyRunner;

use connection::executor::Conn;

use model::actions::Action;

type DBConnector = PooledConnection<ConnectionManager<PgConnection>>;

pub enum Channels {
    AllTables,
    AllQueries,
    AllScripts,
    Table(String),
    Query(String),
    Script(String),
    TableData(String),
}

pub trait ChannelBroadcaster {
    fn on_broadcast<T>(&self, channel: Channels, msg: T);
}

pub struct State<B>
    where
        B: ChannelBroadcaster + Send + 'static,
{
    database: DBConnector, //TODO: this should be templated
    broadcaster: B
    //user
}

impl<B> State<B>
    where
        B: ChannelBroadcaster + Send + 'static,
{
    pub fn new(
        database: DBConnector,
        broadcaster: B,
    ) -> Self {
        Self {
            database,
            broadcaster,
        }
    }
}

pub trait GetConnection {
    type Connection;
    fn get_conn<'a>(&'a self) -> &'a Self::Connection;
}

impl<B> GetConnection for State<B>
    where
        B: ChannelBroadcaster + Send + 'static,
{
    type Connection = Conn;
    fn get_conn<'a>(&'a self) -> &'a Conn {
        &self.database
    }
}