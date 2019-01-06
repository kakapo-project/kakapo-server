use connection::executor::Conn;
use data;

pub mod error;

use model::table::error::TableQueryError;
use model::state::State;
use model::state::ChannelBroadcaster;
use data::TableData;
use data::KeyData;


pub struct TableAction;
pub trait TableActionFunctions<S> {
    fn query(conn: &S, table: &data::Table) -> Result<data::TableData, TableQueryError>;

    fn insert_row(conn: &S, table: &data::Table, data: &TableData, fail_on_duplicate: bool) -> Result<data::TableData, TableQueryError>;

    fn upsert_row(conn: &S, table: &data::Table, data: &TableData) -> Result<data::TableData, TableQueryError>;

    fn update_row(conn: &S, table: &data::Table, keys: &KeyData, data: &TableData, fail_on_exists: bool) -> Result<data::TableData, TableQueryError>;

    fn delete_row(conn: &S, table: &data::Table, keys: &KeyData, fail_on_exists: bool) -> Result<data::TableData, TableQueryError>;
}

impl<B> TableActionFunctions<State<B>> for TableAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn query(conn: &State<B>, table: &data::Table) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn insert_row(conn: &State<B>, table: &data::Table, data: &TableData, fail_on_duplicate: bool) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn upsert_row(conn: &State<B>, table: &data::Table, data: &TableData) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn update_row(conn: &State<B>, table: &data::Table, keys: &KeyData, data: &TableData, fail_on_exists: bool) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn delete_row(conn: &State<B>, table: &data::Table, keys: &KeyData, fail_on_exists: bool) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }
}