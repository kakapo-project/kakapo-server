use connection::executor::Conn;
use data;

pub mod error;

use model::table::error::TableError;

use model::state::State;
use model::state::ChannelBroadcaster;
use model::entity::error::EntityError;

use database::Database;
use database::DatabaseFunctions;
use model::state::GetConnection;


pub struct TableAction;
pub trait TableActionFunctions<S> {
    fn query(conn: &S, table: &data::Table) -> Result<data::RawTableData, TableError>;

    fn insert_row(conn: &S, table: &data::Table, data: &data::ObjectValues, fail_on_duplicate: bool) -> Result<data::RawTableData, TableError>;

    fn upsert_row(conn: &S, table: &data::Table, data: &data::ObjectValues) -> Result<data::RawTableData, TableError>;

    fn update_row(conn: &S, table: &data::Table, keys: &data::ObjectKeys, data: &data::ObjectValues, fail_on_exists: bool) -> Result<data::RawTableData, TableError>;

    fn delete_row(conn: &S, table: &data::Table, keys: &data::ObjectKeys, fail_on_exists: bool) -> Result<data::RawTableData, TableError>;
}

impl<B> TableActionFunctions<State<B>> for TableAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn query(conn: &State<B>, table: &data::Table) -> Result<data::RawTableData, TableError> {

        let query = format!("SELECT * FROM {}", &table.name);
        let result = Database::exec(conn.get_conn(), &query/*, vec![]*/);
        unimplemented!()
    }

    fn insert_row(conn: &State<B>, table: &data::Table, data: &data::ObjectValues, fail_on_duplicate: bool) -> Result<data::RawTableData, TableError> {

        let query = format!(
            "INSERT INTO {name} {columns} VALUES {values}",
            name=table.name,
            columns=vec!["TEST"].join(","),
            values=vec!["TEST"].join(","),
        );
        Database::exec(conn.get_conn(), &query/*, values*/);
        unimplemented!()
    }

    fn upsert_row(conn: &State<B>, table: &data::Table, data: &data::ObjectValues) -> Result<data::RawTableData, TableError> {
        //TODO: doing this because I want to know whether it was an insert or update so that I can put in the correct data in the transactions table
        // otherise, maybe ON CONFLICT with triggers would have been the proper choice
        Database::exec(conn.get_conn(), "SELECT id FROM table WHERE id = my_id"/*, params*/);
        Database::exec(conn.get_conn(), "INSERT INTO table (value1, value2, value3) VALUES (1, 2, 3)"/*, params*/);
        Database::exec(conn.get_conn(), "UPDATE table SET value1 = 1, value2 = 2 WHERE id = my_id"/*, params*/);
        unimplemented!()
    }

    fn update_row(conn: &State<B>, table: &data::Table, keys: &data::ObjectKeys, data: &data::ObjectValues, fail_on_exists: bool) -> Result<data::RawTableData, TableError> {
        let query = format!(
            "UPDATE {name} SET {sets} WHERE {id}", //"UPDATE table SET value1 = 1, value2 = 2 WHERE id = my_id"
            name=table.name,
            sets=vec!["TEST"].join(","),
            id=vec!["TEST"].join(","),
        );
        Database::exec(conn.get_conn(), &query/*, values*/);
        unimplemented!()
    }

    fn delete_row(conn: &State<B>, table: &data::Table, keys: &data::ObjectKeys, fail_on_exists: bool) -> Result<data::RawTableData, TableError> {
        let query = format!(
            "DELETE {name} WHERE {id}", //"DELETE table WHERE id = my_id"
            name=table.name,
            id=vec!["TEST"].join(","),
        );
        Database::exec(conn.get_conn(), &query/*, values*/);
        unimplemented!()
    }
}