
extern crate pq_sys;

use std::ops::Deref;
use std::os::raw;
use std::ffi::CString;
use std::ptr::NonNull;
use std::mem::transmute_copy;
use std::mem;
use std::{slice, str};
use std::io;
use std::str::from_utf8;
use std::ptr;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use serde_json;

use super::data;
use super::data::{RowData, Table, Value, DataType, TableData};
use diesel::pg::Pg;
use diesel::sql_types;
use diesel::deserialize::FromSql;
use diesel::serialize::Output;
use diesel::pg::PgMetadataLookup;
use diesel::serialize::ToSql;
use diesel::serialize::IsNull;
use diesel::serialize;

struct InternalRawConnection {
    pub internal_connection: NonNull<pq_sys::PGconn>,
}

struct InternalPgConnection {
    pub raw_connection: InternalRawConnection,
    //Other stuff here
}

struct ConnWrapper(*mut pq_sys::PGconn);
struct ResultWrapper(*mut pq_sys::PGresult);

impl ConnWrapper {
    fn p(&self) -> *mut pq_sys::PGconn {
        self.0
    }
}
/*
impl Drop for ConnWrapper {
    //TODO: is this handled or not handled (MEMORY LEAK!!!!!!!!!!!!!)
    fn drop(&mut self) {
        unsafe { pq_sys::PQfinish(self.p()) };
    }
}
*/

fn generate_error(fmt: &str) -> Error {
    Error::SerializationError(
        Box::new(
            io::Error::new(
                io::ErrorKind::Other, fmt
            )
        )
    )
}

impl ResultWrapper {
    fn p(&self) -> *mut pq_sys::PGresult {
        self.0
    }

    pub fn num_rows(&self) -> usize {
        unsafe { pq_sys::PQntuples(self.p()) as usize }
    }

    pub fn num_cols(&self) -> usize {
        unsafe  { pq_sys::PQnfields(self.p()) as usize }
    }

    pub fn get_binary(&self, row_idx: usize, col_idx: usize) -> Option<&[u8]> {
        if self.is_null(row_idx, col_idx) {
            None
        } else {
            let row_idx = row_idx as raw::c_int;
            let col_idx = col_idx as raw::c_int;
            unsafe {
                let value_ptr =
                    pq_sys::PQgetvalue(self.p(), row_idx, col_idx) as *const u8;
                let num_bytes = pq_sys::PQgetlength(self.p(), row_idx, col_idx);

                Some(slice::from_raw_parts(value_ptr, num_bytes as usize))
            }
        }
    }


    //FIXME: don't use this all the time, it's gonna be slow
    pub fn get(&self, row_idx: usize, col_idx: usize) -> Result<Value, Error> {


        let type_oid = unsafe { pq_sys::PQftype(self.p(), col_idx as i32) };
        let data_type = match type_oid {
            0x17 => Ok(DataType::Integer),
            0x19 => Ok(DataType::String),
            _ => Err(generate_error(&format!("could not understand oid : `0x{:X?}`", type_oid))),
        }?;

        self.get_with_hint(data_type, row_idx, col_idx)
    }

    pub fn get_with_hint(&self, data_type: DataType, row_idx: usize, col_idx: usize) -> Result<Value, Error> {
        let bytes = self.get_binary(row_idx, col_idx);
        let result = if bytes.is_none() {
            Value::Null
        } else {
             match data_type {
                DataType::Integer => Value::Integer({
                    <i32 as FromSql<sql_types::Integer, Pg>>::from_sql(bytes)
                        .or_else(|err| Err(Error::SerializationError(err)))
                        .and_then(|r| Ok(r as i64))?
                }),
                DataType::String => Value::String(
                    <String as FromSql<sql_types::Text, Pg>>::from_sql(bytes)
                        .or_else(|err| Err(Error::SerializationError(err)))?
                ),
                DataType::Json => Value::Json(
                    <serde_json::Value as FromSql<sql_types::Json, Pg>>::from_sql(bytes)
                        .or_else(|err| Err(Error::SerializationError(err)))?
                ),
            }
        };


        Ok(result)
    }

    pub fn get_column_names(&self) -> Result<Vec<String>, Error> {
        let num_cols = self.num_cols();

        let res: Result<Vec<String>, Error> =
            (0..num_cols).map(|col_idx| {
                let name_str = unsafe {
                    let name_ptr = unsafe { pq_sys::PQfname(self.p(), col_idx as i32) };
                    CString::from_raw(name_ptr)
                };
                name_str.into_string().or_else(|err| Err(generate_error("error parsing column name")))
            }).collect::<Result<Vec<String>, Error>>();

        res
    }

    pub fn get_rows_data(&self) -> Result<Vec<Vec<Value>>, Error> {
        let num_cols = self.num_cols();
        let num_rows = self.num_rows();

        let res: Result<Vec<Vec<Value>>, Error> =
            (0..num_rows).map(|row_idx| {
                (0..num_cols).map(|col_idx| {
                    self.get(row_idx, col_idx)
                }).collect()
            }).collect();

        res
    }

    pub fn is_null(&self, row_idx: usize, col_idx: usize) -> bool {
        unsafe {
            0 != pq_sys::PQgetisnull(
                self.p(),
                row_idx as raw::c_int,
                col_idx as raw::c_int,
            )
        }
    }
}

impl Drop for ResultWrapper {
    fn drop(&mut self) {
        unsafe { pq_sys::PQclear(self.p()) }
    }
}

fn conn_ptr(db: &PgConnection) -> ConnWrapper {
    let internal_db = unsafe {
        //WARNING: this assumes that the `PgConnection` and `InternalPgConnection` are the same underlying data format
        //rust compiler could switch around the data layout apparently, which could mess up the transmute
        transmute_copy::<PgConnection, InternalPgConnection>(db)
    };
    let raw_connection = internal_db.raw_connection;

    let conn = raw_connection.internal_connection.as_ptr();

    ConnWrapper(conn)
}

fn exec(conn: &ConnWrapper, query: &str, params: Vec<Value>) -> Result<ResultWrapper, Error> {

    let query_cstring = CString::new(query)?;

    let param_data: Vec<Option<Vec<u8>>> = params.iter().map(|x| {

        let mut bytes = Output::new(Vec::new(), unsafe { mem::uninitialized() }); //This is probably fine
        let result = match x {
            Value::Null => Ok(IsNull::Yes),
            Value::Integer(x) => {
                let value = *x as i32;
                <i32 as ToSql<sql_types::Integer, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::String(x) => {
                let value = x;
                <String as ToSql<sql_types::Text, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Json(x) => {
                let value = x;
                <serde_json::Value as ToSql<sql_types::Json, Pg>>::to_sql(&value, &mut bytes)
            },
        };

        result
            .and_then(|is_null| {
                match is_null {
                    IsNull::No => Ok(Some(bytes.into_inner())),
                    IsNull::Yes => Ok(None),
                }
            })
            .or_else(|err| Err(Error::SerializationError(err)))
    }).collect::<Result<_, Error>>()?;

    let params_pointer = param_data
        .iter()
        .map(|data| {
            data.as_ref()
                .map(|d| d.as_ptr() as *const raw::c_char)
                .unwrap_or(ptr::null())
        })
        .collect::<Vec<_>>();
    let param_lengths = param_data
        .iter()
        .map(|data| data.as_ref().map(|d| d.len() as raw::c_int).unwrap_or(0))
        .collect::<Vec<_>>();

    let result = unsafe {
        //TODO: you can cache this with `PQprepare`
        pq_sys::PQexecParams(
            conn.p(),
            query_cstring.as_ptr(),
            params_pointer.len() as raw::c_int,
            ptr::null() as *const u32, //FIXME: what are the oids?
            params_pointer.as_ptr(),
            param_lengths.as_ptr(),
            vec![1 as raw::c_int; params_pointer.len()].as_ptr(),
            1 as raw::c_int
        )
    };

    Ok(ResultWrapper(result))
}

pub fn get_all_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\".\"{}\"", table_name, x.name))
        .collect::<Vec<String>>();

    //SELECT "table_name"."col1", "table_name"."col2" FROM "table_name";
    let query = format!("SELECT {} FROM \"{}\";", column_names.join(", "), table_name);
    println!("final query: {:?}", query);


    let result = exec(&internal_conn, &query, vec![])?;
    println!("Number of results: {}", result.num_rows());

    let rows = result.get_rows_data()?;

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}


pub fn execute_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    query: &data::Query,
    //TODO: params
    //params: &data::QueryParams,
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let query_str = &query.statement;
    println!("final query: {:?}", query);


    let result = exec(&internal_conn, &query_str, vec![])?;
    println!("Number of results: {}", result.num_rows());

    let rows = result.get_rows_data()?;

    let table_data = TableData::RowsFlatData {
        columns: result.get_column_names()?,
        data: rows,
    };

    Ok(table_data)
}

pub fn upsert_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //INSERT INTO "table_name" ("col_1_pk", "col_2", "col_3")
        //    VALUES (1, 2, 3)
        //    ON CONFLICT (col_1_pk) DO UPDATE SET col_2 = EXCLUDED.col_2, col_3 = EXCLUDED.col_3;
        //TODO: insert multiple values at once
        let row_column_names: Vec<String> = item.keys().cloned().map(|x| format!("\"{}\"", x)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_insert_into = format!(
            "INSERT INTO \"{}\" ({})",
            table_name, row_column_names.join(", "));
        let query_values = format!(
            "VALUES ({})",
            (0..row_column_names.len())
                .map(|x| format!("${}", x+1))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let query_on_conflict = if column_names_without_key.len() > 0 {
            format!(
                "ON CONFLICT ({}) DO UPDATE SET {}",
                key_column_name,
                column_names_without_key.iter().map(|x| format!("{} = EXCLUDED.{}", x, x))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            "".to_string()
        };
        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{}\n{};", query_insert_into, query_values, query_on_conflict, query_returning);
        println!("query: {}", query);

        let result = exec(&internal_conn, &query, values)?;
        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}

pub fn insert_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData,
    ignore_on_failure: bool,
) -> Result<TableData, Error> {
    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //INSERT INTO "table_name" ("col_1_pk", "col_2", "col_3")
        //    VALUES (1, 2, 3)
        //    ON CONFLICT (col_1_pk) DO UPDATE SET col_2 = EXCLUDED.col_2, col_3 = EXCLUDED.col_3;
        //TODO: insert multiple values at once
        let row_column_names: Vec<String> = item.keys().cloned().map(|x| format!("\"{}\"", x)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_insert_into = format!(
            "INSERT INTO \"{}\" ({})",
            table_name, row_column_names.join(", "));
        let query_values = format!(
            "VALUES ({})",
            (0..row_column_names.len())
                .map(|x| format!("${}", x+1))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let query_on_conflict = if ignore_on_failure {
            format!(
                "ON CONFLICT ({}) DO NOTHING",
                key_column_name,
            )
        } else {
            "".to_string()
        };
        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{}\n{};", query_insert_into, query_values, query_on_conflict, query_returning);
        println!("query: {}", query);

        let result = exec(&internal_conn, &query, values)?;
        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}

pub fn update_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData,
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //UPDATE "table_name" SET "col_2" = 2, "col_3" = 3
        //    WHERE "col_1_pk" = 1;
        //TODO: insert multiple values at once
        let row_column_names: Vec<(String, usize)> = item.keys().cloned().enumerate()
            .map(|(i, x)| (format!("\"{}\"", x), i+1)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<(String, usize)> = row_column_names.iter()
            .filter(|&x| x.0.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<(String, usize)> = row_column_names.iter()
            .filter(|&x| x.0.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_update_value = format!(
            "UPDATE \"{}\" SET {}",
            table_name,
            column_names_without_key.iter()
                .map(|(x, i)| format!("{} = ${}", x, i))
                .collect::<Vec<String>>()
                .join(", "));
        let where_clause = format!(
            "WHERE {}",
            column_names_only_key.iter()
                .map(|(x, i)| format!("{} = ${}", x, i))
                .collect::<Vec<String>>()
                .join(" AND ")
        );

        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{};", query_update_value, where_clause, query_returning);
        println!("query: {}", query);

        let result = exec(&internal_conn, &query, values)?;
        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}


pub fn update_row_with_key(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    row_data: RowData,
    key_value: String,
) -> Result<RowData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);

    //UPDATE "table_name" SET "col_2" = 2, "col_3" = 3
    //    WHERE "col_1_pk" = 1;
    let item = row_data.into_row_data_vec();
    let row_column_names: Vec<(String, usize)> = item.keys().enumerate()
        .map(|(i, x)| (format!("\"{}\"", x), i+1)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
    let mut values: Vec<Value> = item.values().cloned().collect();

    let query_update_value = format!(
        "UPDATE \"{}\" SET {}",
        table_name,
        row_column_names.iter()
            .map(|(x, i)| format!("{} = ${}", x, i))
            .collect::<Vec<String>>()
            .join(", "));
    let where_clause = format!(
        "WHERE {} = ${}",
        key_column_name,
        row_column_names.len() + 1
    );

    let query_returning = format!(
        "RETURNING {}",
        column_names.join(", ")
    );
    let query = format!("{}\n{}\n{};", query_update_value, where_clause, query_returning);
    println!("query: {}", query);

    values.push(Value::String(key_value));
    println!("value: {:?}", values);
    let result = exec(&internal_conn, &query, values)?;
    let rows = result.get_rows_data()?;
    let row: Vec<Value> = match rows.first() {
        None => Err(generate_error("no row")),
        Some(x) => Ok(x.to_owned()),
    }?;

    let row_data = RowData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: row,
    };

    Ok(row_data)
}

pub fn delete_row_with_key(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    key_value: String, //TODO: this should be value based on type
) -> Result<RowData, Error> {


    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);

    //DELETE FROM "table_name" WHERE "col_1_pk" = 1;

    let delete_from = format!(
        "DELETE FROM \"{}\" ",
        table_name
    );
    let where_clause = format!(
        "WHERE {} = $1",
        key_column_name
    );

    let query_returning = format!(
        "RETURNING {}",
        column_names.join(", ")
    );
    let query = format!("{}\n{}\n{};", delete_from, where_clause, query_returning);
    println!("query: {}", query);

    let values = vec![Value::String(key_value)];
    let result = exec(&internal_conn, &query, values)?;
    let rows = result.get_rows_data()?;
    let row: Vec<Value> = match rows.first() {
        None => Err(generate_error("no row")),
        Some(x) => Ok(x.to_owned()),
    }?;

    let row_data = RowData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: row,
    };

    Ok(row_data)
}