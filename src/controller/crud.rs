use serde_json;

use super::schema::Schema;

use super::repository::{Error, Repository, Transaction};
use super::query::{GetQuery, CreateQuery, UpdateQuery, DeleteQuery, OrderType};

use super::types::DataPoint;
use super::rows::{Row, Rows};
use super::types::DataType::*;

use super::meta;



pub fn get_table_schema(repository: &Repository, table_id: &String) -> Result<Schema, String> {

    repository.transaction()
        .and_then::<Schema, _>(|tr| {

            let table_table = tr.get("table_history",
                                     &GetQuery::new()
                                         .column_equals("name", &DataPoint::String(table_id.to_owned()))
                                         .order_by("modified_at", &OrderType::Descending))?;
            //TODO: assert schema is correct

            let row = match table_table.x(0) {
                Some(row) => row,
                None => return Err(Error::TransactionError(tr, format!("could not find table `{}`", table_id.to_owned()))),
            };

            let column_index = meta::table_history().get_column_index("table_info").unwrap();
            let table_schema: Schema = match row.y(column_index).unwrap() {
                DataPoint::Json(json_value) => serde_json::from_value(json_value).unwrap(),
                _ => return Err(Error::TransactionError(tr, "Error capturing table_info".to_string())),
            };


            tr.commit()?;

            Ok(table_schema)
        }).or_else::<String, _>(|err| {
        match err {
            Error::TransactionError(transaction, msg) => {
                transaction.rollback();
                Err(msg)
            },
            Error::UsageError(msg) => Err(msg),
            Error::SystemError(msg) => Err(msg),
        }
    })
}

pub fn get_table_rows(repository: &Repository, table_id: &String, query: &GetQuery) -> Result<Rows, String> {

    repository.transaction()
        .and_then::<Rows, _>(|tr| {

            //TODO: assert table is in table_history and get the columns in that order
            let rows = tr.get(&table_id, &query)?;

            tr.commit()?;

            Ok(rows)
        }).or_else::<String, _>(|err| {
        match err {
            Error::TransactionError(transaction, msg) => {
                transaction.rollback();
                Err(msg)
            },
            Error::UsageError(msg) => Err(msg),
            Error::SystemError(msg) => Err(msg),
        }
    })
}