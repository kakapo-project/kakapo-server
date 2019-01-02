use connection::executor::Conn;
use data;

pub mod error;

use model::table::error::TableQueryError;
use model::state::State;


pub struct TableAction;
pub trait TableActionFunctions<S> {
    fn query(conn: &S, table: data::Table) -> Result<data::TableData, TableQueryError>;

    fn insert_row(conn: &S, table: data::Table) -> ();

    fn upsert_row(conn: &S, table: data::Table) -> ();

    fn update_row(conn: &S, table: data::Table) -> ();

    fn delete_row(conn: &S, table: data::Table) -> ();
}

impl TableActionFunctions<State> for TableAction {
    fn query(conn: &State, table: data::Table) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn insert_row(conn: &State, table: data::Table) -> () {
        unimplemented!()
    }

    fn upsert_row(conn: &State, table: data::Table) -> () {
        unimplemented!()
    }

    fn update_row(conn: &State, table: data::Table) -> () {
        unimplemented!()
    }

    fn delete_row(conn: &State, table: data::Table) -> () {
        unimplemented!()
    }
}