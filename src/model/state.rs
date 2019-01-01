
use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use connection::py::PyRunner;

use connection::executor::Conn;

pub type State = PooledConnection<ConnectionManager<PgConnection>>; //TODO: should include user data

pub trait GetConnection {
    type Connection;
    fn get_conn<'a>(&'a self) -> &'a Self::Connection;
}

impl GetConnection for State {
    type Connection = Conn;
    fn get_conn<'a>(&'a self) -> &'a Conn {
        self
    }
}