
extern crate pq_sys;

use std::ops::Deref;
use std::os::raw;
use std::ffi::{CString, CStr};
use std::ptr::NonNull;
use std::mem::transmute_copy;
use std::mem;
use std::{slice, str};
use std::io;
use std::ptr;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use serde_json;

use data;
use data::{RowData, Table, Value, DataType, TableData};
use diesel::pg::Pg;
use diesel::sql_types;
use diesel::deserialize::FromSql;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::serialize::IsNull;

use base64;

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

fn generate_error(fmt: &str) -> Error {
    Error::SerializationError(
        Box::new(
            io::Error::new(
                io::ErrorKind::Other, fmt
            )
        )
    )
}

type FromError = std::boxed::Box<(dyn std::error::Error + std::marker::Sync + std::marker::Send + 'static)>;
fn parse<T>(data: Result<T, FromError>) -> Result<T, Error> {
    data.or_else(|err| Err(Error::SerializationError(err)))
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
                DataType::SmallInteger => Value::Integer(parse(<i16 as FromSql<sql_types::SmallInt, Pg>>::from_sql(bytes))? as i64),
                DataType::Integer => Value::Integer(parse(<i32 as FromSql<sql_types::Integer, Pg>>::from_sql(bytes))? as i64),
                DataType::BigInteger =>  Value::Integer(parse(<i64 as FromSql<sql_types::BigInt, Pg>>::from_sql(bytes))?),
                DataType::Float => Value::Float(parse(<f32 as FromSql<sql_types::Float, Pg>>::from_sql(bytes))? as f64),
                DataType::DoubleFloat => Value::Float(parse(<f64 as FromSql<sql_types::Double, Pg>>::from_sql(bytes))?),

                DataType::String => Value::String(parse(<String as FromSql<sql_types::Text, Pg>>::from_sql(bytes))?),
                DataType::VarChar { length } => Value::String(parse(<String as FromSql<sql_types::VarChar, Pg>>::from_sql(bytes))?),

                DataType::Byte => Value::Binary {
                    b64: base64::encode(&parse(<Vec<u8> as FromSql<sql_types::Binary, Pg>>::from_sql(bytes))?)
                },

                DataType::Timestamp { with_tz } => Value::DateTime { datetime: parse(<chrono::NaiveDateTime as FromSql<sql_types::Timestamp, Pg>>::from_sql(bytes))? },
                DataType::Date => Value::Date { date: parse(<chrono::NaiveDate as FromSql<sql_types::Date, Pg>>::from_sql(bytes))? },
                DataType::Time { with_tz } => Value::DateTime { datetime: parse(<chrono::NaiveDateTime as FromSql<sql_types::Timestamp, Pg>>::from_sql(bytes))? },

                DataType::Boolean => Value::Boolean(parse(<bool as FromSql<sql_types::Bool, Pg>>::from_sql(bytes))?),
                DataType::Json => Value::Json { json: parse(<serde_json::Value as FromSql<sql_types::Json, Pg>>::from_sql(bytes))? },
            }
        };


        Ok(result)
    }

    pub fn get_column_names(&self) -> Result<Vec<String>, Error> {
        let num_cols = self.num_cols();

        let res: Result<Vec<String>, Error> =
            (0..num_cols).map(|col_idx| {
                let name_raw = unsafe {
                    let name_ptr = pq_sys::PQfname(self.p(), col_idx as i32);
                    CStr::from_ptr(name_ptr)
                };
                name_raw.to_str()
                    .or_else(|err| Err(generate_error("error parsing column name")))
                    .and_then(|val| Ok(val.to_owned()))
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

    pub fn print_error(&self) -> () {
        unsafe {
            let error_enum = pq_sys::PQresultStatus(self.p());
            println!("status: {:?}", error_enum);
            if format!("{:?}", error_enum) == "PGRES_FATAL_ERROR" {
                let r = pq_sys::PQresultErrorMessage(self.p());
                println!("error: {:?}", CString::from_raw(r));
            }
        }
    }
}

impl Drop for ResultWrapper {
    fn drop(&mut self) {
        //drop it like it's hot
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
                let value = *x as i32; //TODO: why not i64?
                <i32 as ToSql<sql_types::Integer, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Float(x) => {
                let value = x;
                <f64 as ToSql<sql_types::Double, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Boolean(x) => {
                let value = x;
                <bool as ToSql<sql_types::Bool, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::DateTime { datetime } => {
                let value = datetime;
                <chrono::NaiveDateTime as ToSql<sql_types::Timestamp, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Date { date } => {
                let value = date;
                <chrono::NaiveDate as ToSql<sql_types::Date, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::String(x) => {
                let value = x;
                <String as ToSql<sql_types::Text, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Binary { b64 } => {
                let value =  base64::decode(b64).or_else(|err| Err(generate_error("error decoding base 64 data")))?;
                <Vec<u8> as ToSql<sql_types::Binary, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Json { json } => {
                let value = json;
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

    let param_types: Vec<u32> = params.iter().map(|x| {
         match x {
             Value::Null => 0x0, //TODO: is this right?
             Value::Integer(x) => 0x17,
             Value::String(x) => 0x19,
             _ => 0x0, //TODO: fix
         }
    }).collect();

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
            param_types.as_ptr(),
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

    let query_str = format!("{}", &query.statement);
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

