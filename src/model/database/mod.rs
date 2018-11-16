
extern crate pq_sys;

use std::ops::Deref;
use std::os::raw;
use std::ffi::CString;
use std::ptr::NonNull;
use std::mem::transmute_copy;
use std::{slice, str};
use std::io;
use std::str::from_utf8;
use std::ptr;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use serde_json;

use super::data::{RowData, Table, Value, DataType};
use diesel::pg::Pg;
use diesel::sql_types;
use diesel::deserialize::FromSql;

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

impl Drop for ConnWrapper {
    fn drop(&mut self) {
        unsafe { pq_sys::PQfinish(self.p()) };
    }
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
            _ => Err(
                Error::SerializationError(
                    Box::new(
                        io::Error::new(
                            io::ErrorKind::Other, format!("could not understand oid : `0x{:X?}`", type_oid)
                        )
                    )
                )
            )
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

fn exec(conn: ConnWrapper, query: &str) -> Result<ResultWrapper, Error> {

    let query_cstring = CString::new(query)?;

    let param_data: Vec<Option<Vec<u8>>> = vec![];

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
) -> Result<Vec<RowData>, Error> {

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


    let result = exec(internal_conn, &query)?;
    println!("Number of results: {}", result.num_rows());

    let res: Result<Vec<Vec<Value>>, Error> =
    (0..result.num_rows()).map(|row_idx| {
        (0..result.num_cols()).map(|col_idx| {
            result.get(row_idx, col_idx)
        }).collect()
    }).collect();

    println!("res: {:?}", res?);

    Ok(vec![])
}
