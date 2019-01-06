use connection::executor::Conn;
use data;

pub mod error;

use model::table::error::TableError;

use model::state::State;
use model::state::ChannelBroadcaster;
use data::TableData;
use data::KeyData;


pub struct TableAction;
pub trait TableActionFunctions<S> {
    fn query(conn: &S, table: &data::Table) -> Result<data::TableData, TableError>;

    fn insert_row(conn: &S, table: &data::Table, data: &TableData, fail_on_duplicate: bool) -> Result<data::TableData, TableError>;

    fn upsert_row(conn: &S, table: &data::Table, data: &TableData) -> Result<data::TableData, TableError>;

    fn update_row(conn: &S, table: &data::Table, keys: &KeyData, data: &TableData, fail_on_exists: bool) -> Result<data::TableData, TableError>;

    fn delete_row(conn: &S, table: &data::Table, keys: &KeyData, fail_on_exists: bool) -> Result<data::TableData, TableError>;
}

impl<B> TableActionFunctions<State<B>> for TableAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn query(conn: &State<B>, table: &data::Table) -> Result<data::TableData, TableError> {
        unimplemented!()
    }

    fn insert_row(conn: &State<B>, table: &data::Table, data: &TableData, fail_on_duplicate: bool) -> Result<data::TableData, TableError> {
        unimplemented!()
    }

    fn upsert_row(conn: &State<B>, table: &data::Table, data: &TableData) -> Result<data::TableData, TableError> {
        unimplemented!()
    }

    fn update_row(conn: &State<B>, table: &data::Table, keys: &KeyData, data: &TableData, fail_on_exists: bool) -> Result<data::TableData, TableError> {
        unimplemented!()
    }

    fn delete_row(conn: &State<B>, table: &data::Table, keys: &KeyData, fail_on_exists: bool) -> Result<data::TableData, TableError> {
        unimplemented!()
    }
}