
use diesel;
use diesel::result::Error;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use failure::Fail;

use data;
use data::api;

use super::dbdata::*;
use super::schema::table_schema;
use super::manage::utils::{get_single_table, unroll_table};
use super::database;

type DataQuery = RawQuery;
type DataQueryHistory = RawQueryHistory;
type TableSchema = RawTable;
type TableSchemaHistory = RawTableHistory;
type DataScript = RawScript;
type DataScriptHistory = RawScriptHistory;

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
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    format: api::TableDataFormat,
    //TODO: Better SQL query functionality, i.e. filter, ...
) -> Result<api::GetTableDataResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(conn, table_name)?;
        let data = database::get_all_rows(conn, &table)?;
        let formatted_data = match format {
            data::TableDataFormat::Rows => data.into_rows_data(),
            data::TableDataFormat::FlatRows => data.into_rows_flat_data(),
        };

        let table_with_data = data::TableWithData {
            table: table,
            data: formatted_data,
        };

        Ok(table_with_data)

    }).or_else(|err| match err {
        diesel::result::Error::NotFound => Err(api::Error::TableNotFound),
        _ => Err(api::Error::DatabaseError(err)),
    })?;

    println!("final result: {:?}", result);

    Ok(api::GetTableDataResult(result.data))
}


pub fn insert_table_data(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    table_data: api::TableData,
    format: api::TableDataFormat,
    method: api::CreationMethod,
) -> Result<api::InsertTableDataResult, api::Error> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(conn, table_name)?;
        let data = match method {
            data::CreationMethod::Update => database::upsert_rows(conn, &table, table_data)?,
            data::CreationMethod::IgnoreIfExists => database::insert_rows(conn, &table, table_data, true)?,
            data::CreationMethod::FailIfExists => database::insert_rows(conn, &table, table_data, false)?,
            data::CreationMethod::FailIfNotExists => database::update_rows(conn, &table, table_data)?,
        };

        let formatted_data = match format {
            data::TableDataFormat::Rows => data.into_rows_data(),
            data::TableDataFormat::FlatRows => data.into_rows_flat_data(),
        };

        Ok(formatted_data)
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::InsertTableDataResult(result))
}


pub fn update_table_data(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    key: String,
    row_data: api::RowData,
    format: api::TableDataFormat,
) -> Result<api::UpdateTableDataResult, api::Error> {
    //NOTE: key should be parsed here to support multiple/spacial primary keys
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(conn, table_name)?;
        let data = database::update_row_with_key(conn, &table, row_data, key)?;
        let formatted_data = match format {
            data::TableDataFormat::Rows => data.into_row_data(),
            data::TableDataFormat::FlatRows => data.into_row_flat_data(),
        };

        Ok(formatted_data)
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::UpdateTableDataResult(result))
}


pub fn delete_table_data(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    key: String,
) -> Result<api::DeleteTableDataResult, api::Error> {
    //NOTE: key should be parsed here to support multiple/spacial primary keys
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let table = get_table(conn, table_name)?;
        let data = database::delete_row_with_key(conn, &table, key)?;

        Ok(data)
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::DeleteTableDataResult(result))
}