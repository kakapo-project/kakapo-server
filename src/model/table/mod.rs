use connection::executor::Conn;
use data;

pub mod error;

use model::table::error::TableQueryError;


pub struct TableAction;
pub trait TableActionFunctions {
    fn query(conn: &Conn, table: data::Table) -> Result<data::TableData, TableQueryError>;

    fn insert_row(conn: &Conn, table: data::Table) -> ();

    fn upsert_row(conn: &Conn, table: data::Table) -> ();

    fn update_row(conn: &Conn, table: data::Table) -> ();

    fn delete_row(conn: &Conn, table: data::Table) -> ();
}

impl TableActionFunctions for TableAction {
    fn query(conn: &Conn, table: data::Table) -> Result<data::TableData, TableQueryError> {
        unimplemented!()
    }

    fn insert_row(conn: &Conn, table: data::Table) -> () {
        unimplemented!()
    }

    fn upsert_row(conn: &Conn, table: data::Table) -> () {
        unimplemented!()
    }

    fn update_row(conn: &Conn, table: data::Table) -> () {
        unimplemented!()
    }

    fn delete_row(conn: &Conn, table: data::Table) -> () {
        unimplemented!()
    }
}