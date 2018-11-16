
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

/*
use diesel_dynamic_schema::table as dynamic_table;
use diesel_dynamic_schema::Table as DynamicTable;
use diesel_dynamic_schema::Column as DynamicColumn;
use diesel_dynamic_schema::*;

struct TableMetaData {
    table: data::Table,
    schema_table: DynamicTable<String, String>,
    types: Vec<DynamicValueType>,
    column_names: Vec<String>,
    columns: Vec<DynamicColumn<DynamicTable<String, String>, String, Binary>>,
}

impl TableMetaData {
    pub fn get_column(&self, name: &str) -> Result<DynamicColumn<DynamicTable<String, String>, String, Binary>, diesel::result::Error> {
        //FIXME: O(n)
        let column_names = &self.column_names;
        let columns = &self.columns;
        for (column_name, column) in column_names.iter().zip(columns) {
            if column_name == name {
                return Ok(column.to_owned())
            }
        }

        Err(Error::SerializationError(Box::new(io::Error::new(io::ErrorKind::Other, "column not found")))) //TODO: clean this up
    }
}

fn get_table_meta_data(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String
) -> Result<TableMetaData, diesel::result::Error> {

    let table_schema: TableSchema = table_schema::table
        .filter(table_schema::name.eq(table_name.to_string()))
        .get_result::<TableSchema>(conn)?;
    println!("table schema: {:?}", table_schema);


    let detailed_table: data::DetailedTable = get_single_table(&conn, &table_schema)?;

    let table = unroll_table(detailed_table.to_owned())
        .or_else(|err| Err(Error::SerializationError(Box::new(err.compat()))))?;

    let schema_table = dynamic_table(table_name);

    // parse table
    let mut types: Vec<DynamicValueType> = vec![];
    let mut column_names: Vec<String> = vec![];
    let mut columns: Vec<DynamicColumn<_, _, Binary>> = vec![];

    for col in &table.schema.columns {
        columns.push(schema_table.column::<Blob, _>(col.name.to_owned()));
        column_names.push(col.name.to_owned());
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

    let table_meta_data = TableMetaData {
        table: table,
        schema_table: schema_table,
        types: types,
        column_names: column_names,
        columns: columns,
    };

    Ok(table_meta_data)
}

pub fn get_table_data(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    //TODO: Better SQL query functionality, i.e. filter, ...
    //TODO: Add output format: indexed, rows (default), columns, schema
) -> Result<api::GetTableDataResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let meta = get_table_meta_data(&conn, table_name)?;

        let query = meta.schema_table.select(VecColumn::new(meta.columns));
        let column_names = meta.column_names;
        let types = meta.types;

        println!("DEBUG QUERY: {:?}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
        let raw_rows = query.load::<ValueList<Vec<u8>>>(&conn)?;
        let rows: Vec<data::RowData> = raw_rows.iter()
            .map(|row| {
                let values: Vec<DynamicValue> = row.decode(&types)?;
                let mut raw_row_data: BTreeMap<String, data::Value> = BTreeMap::new();
                for (key, raw_value) in column_names.iter().zip(values) {
                    let value = match raw_value {
                        DynamicValue::Text(x) => data::Value::String(x),
                        DynamicValue::Integer(x) => data::Value::Integer(x as i64),
                        DynamicValue::Json(x) => data::Value::Json(x),
                    };
                    raw_row_data.insert(key.to_owned(), value);
                }
                let row_data = data::RowData(raw_row_data);
                Ok(row_data)
            })
            .collect::<Result<Vec<data::RowData>, _>>()
            .or_else(|err: Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>|
                Err(Error::SerializationError(Box::new(io::Error::new(io::ErrorKind::Other, "could not decode")))) //TODO: clean this up
            )?;

        let data = data::TableData::RowsData(rows);

        let table_with_data = data::TableWithData {
            table: meta.table,
            data: data,
        };

        Ok(table_with_data)

    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::GetTableDataResult(result))
}


pub fn insert_table_data(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    table_data: api::TableData,
    //TODO: Add output format: indexed, rows (default), columns, schema
) -> Result<api::InsertTableDataResult, api::Error> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let meta = get_table_meta_data(&conn, table_name)?;

        let db_connection: &PgConnection = conn.deref();

        /*
        let query = diesel::insert_into(meta.schema_table)
            .values((
                meta.get_column("name")?.eq("a".to_string()),
                meta.get_column("likes")?.eq("b".to_string()),
                meta.get_column("age")?.eq(42)
            )).execute(&conn);
        */
        let column_names = meta.column_names;
        let types = meta.types;



        Ok(data::TableData::RowsData(vec![]))
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::InsertTableDataResult(result))
}
*/

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

        let table_with_data = data::TableWithData {
            table: table,
            data: data.into_rows_data(),
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

        let db_connection: &PgConnection = conn.deref();

        Ok(data::TableData::RowsData(vec![]))
    }).or_else(|err| Err(api::Error::DatabaseError(err)))?;


    Ok(api::InsertTableDataResult(result))
}