

use serde_json;

use super::super::schema::Schema;
use super::super::types::Identifier;

use super::super::repository::{Error, Repository, Transaction};
use super::super::query::{GetQuery, CreateQuery, UpdateQuery, DeleteQuery, OrderType};

use super::super::types::DataPoint;
use super::super::rows::{Row, Rows};


pub fn get_table_schema(repository: &Repository, table_id: &Identifier) -> Result<Schema, String> {

    let table_name = table_id.get_name();
    
    repository.transaction()
        .and_then::<Schema, _>(|tr| {

            let table_table = tr.get(&Identifier::new("table_history"),
                   &GetQuery::new()
                       .column_equals("name", &DataPoint::String(table_name.to_owned()))
                       .order_by("modified_at", &OrderType::Descending))?;
            //TODO: assert schema is correct

            let row = match table_table.get_rows().x(0) {
                Some(row) => row,
                None => return Err(Error::TransactionError(tr, format!("could not find table `{}`", table_name.to_owned()))),
            };

            let column_index = table_table.get_column_index("table_info").unwrap();
            let table_info: Schema = match row.y(column_index).unwrap() {
                DataPoint::Json(json_value) => serde_json::from_value(json_value).unwrap(),
                _ => return Err(Error::TransactionError(tr, "Error capturing table_info".to_string())),
            };


            tr.commit()?;

            Ok(table_info)
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