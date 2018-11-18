
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
use std::collections::BTreeMap;
use std;
use std::ops::Deref;

use super::data;
use super::data::DataType;
use super::api;

use super::dbdata::*;
use super::schema::{entity, table_schema, table_schema_history};
use super::manage::{get_single_table, unroll_table};
use super::database;

fn get_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String
) -> Result<data::Table, diesel::result::Error> {


    let table_schema: TableSchema = table_schema::table
        .filter(table_schema::name.eq(table_name.to_string()))
        .get_result::<TableSchema>(conn)?;
    println!("table schema: {:?}", table_schema);


    let detailed_table: data::DetailedTable = get_single_table(&conn, &table_schema)?;

    let table = unroll_table(detailed_table.to_owned())
        .or_else(|err| Err(Error::SerializationError(Box::new(err.compat()))))?;

    Ok(table)
}

pub fn get_table_data(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    //TODO: Better SQL query functionality, i.e. filter, ...
    //TODO: Add output format: indexed, rows (default), columns, schema
) -> Result<api::GetTableDataResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(&conn, table_name)?;
        let data = database::get_all_rows(&conn, &table)?;
        let formatted_data = data.into_rows_data(); //TODO: this is where the magic happens to transform into different output format types

        let table_with_data = data::TableWithData {
            table: table,
            data: formatted_data,
        };

        Ok(table_with_data)

    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;

    println!("final result: {:?}", result);

    Ok(api::GetTableDataResult(result))
}


pub fn insert_table_data(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    table_data: api::TableData,
    //TODO: Add output format: indexed, rows (default), columns, schema
) -> Result<api::InsertTableDataResult, api::Error> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(&conn, table_name)?;
        let data = database::upsert_rows(&conn, &table, table_data)?;
        let formatted_data = data.into_rows_data(); //TODO: this is where the magic happens to transform into different output format types

        Ok(formatted_data)
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::InsertTableDataResult(result))
}