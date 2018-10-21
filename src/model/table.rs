
use actix::prelude::*;
use diesel;
use diesel::result::Error;
use diesel::{
    prelude::*,
    sql_types,
    insert_into,
    delete,
    update,
};
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::sql_types::*;

use failure::Fail;
use std::io::Write;
use std::io;

use super::data;
use super::data::DataType;
use super::api;

use super::dbdata::*;
use super::schema::{entity, table_schema, table_schema_history};
use super::manage::{get_single_table, unroll_table};

use diesel_dynamic_schema::table as dynamic_table;
use diesel_dynamic_schema::Column as DynamicColumn;
use diesel_dynamic_schema::*;

struct MyColumns(String);

pub fn get_table_data(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    //TODO: Query this
) -> Result<api::GetTableDataResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {
        let table_schema: TableSchema = table_schema::table
            .filter(table_schema::name.eq(table_name.to_string()))
            .get_result::<TableSchema>(&conn)?;
        println!("table schema: {:?}", table_schema);


        let detailed_table: data::DetailedTable = get_single_table(&conn, &table_schema)?;

        let table = unroll_table(detailed_table.to_owned())
            .or_else(|err| Err(Error::SerializationError(Box::new(err.compat()))))?;

        let schema_table = dynamic_table(table_name);

        // parse table
        let mut types: Vec<DynamicValueType> = vec![];
        let mut columns: Vec<DynamicColumn<_, _, Binary>> = vec![];

        for col in &table.schema.columns {
            columns.push(schema_table.column::<Blob, _>(col.name.to_owned()));
            match col.data_type {
                DataType::Integer => {
                    types.push(DynamicValueType::Integer);
                },
                DataType::String => {
                    types.push(DynamicValueType::Text);
                },
                DataType::Json => {
                    types.push(DynamicValueType::Json);
                }
            };
        }

        let query = schema_table.select(VecColumn::new(columns));
        println!("DEBUG QUERY: {:?}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
        let raw_rows = query.load::<ValueList<Vec<u8>>>(&conn)?;
        let rows: Vec<Vec<DynamicValue>> = raw_rows.iter()
            .map(|x| x.decode(&types))
            .collect::<Result<Vec<Vec<DynamicValue>>, _>>()
            .or_else(|err|
                Err(Error::SerializationError(Box::new(io::Error::new(io::ErrorKind::Other, "could not decode")))) //TODO: clean this up
            )?;
        println!("FINAL ROWS: {:?}", rows);

        Ok(table)

    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    let rows = data::RowsData(vec![]);
    Ok(api::GetTableDataResult::Rows(rows))
}
