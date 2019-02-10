
use std::os::raw;
use std::ffi::CString;
use std::ffi::CStr;
use std::ptr::NonNull;
use std::mem::transmute_copy;
use std::mem;
use std::slice;
use std::str;
use std::io;
use std::ptr;
use diesel::result::Error;

use diesel::prelude::*;

use connection::executor::Conn;

use model::table::DatabaseFunctions;
use data::Value;
use data::DataType;
use data;

use database::error::DbError;
use diesel::pg::Pg;
use diesel::sql_types;
use diesel::deserialize::FromSql;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::serialize::IsNull;


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
    fn new(conn: &PgConnection) -> Self {
        let internal_db = unsafe {
            //WARNING: this assumes that the `PgConnection` and `InternalPgConnection` are the same underlying data format
            //rust compiler could switch around the data layout apparently, which could mess up the transmute
            transmute_copy::<PgConnection, InternalPgConnection>(conn)
        };
        let raw_connection = internal_db.raw_connection;

        let conn = raw_connection.internal_connection.as_ptr();

        ConnWrapper(conn)
    }

    fn p(&self) -> *mut pq_sys::PGconn {
        self.0
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

                DataType::Byte => Value::Binary(parse(<Vec<u8> as FromSql<sql_types::Binary, Pg>>::from_sql(bytes))?),

                DataType::Timestamp { with_tz } => Value::DateTime(parse(<chrono::NaiveDateTime as FromSql<sql_types::Timestamp, Pg>>::from_sql(bytes))?),
                DataType::Date => Value::Date(parse(<chrono::NaiveDate as FromSql<sql_types::Date, Pg>>::from_sql(bytes))?),
                DataType::Time { with_tz } => Value::DateTime(parse(<chrono::NaiveDateTime as FromSql<sql_types::Timestamp, Pg>>::from_sql(bytes))?),

                DataType::Boolean => Value::Boolean(parse(<bool as FromSql<sql_types::Bool, Pg>>::from_sql(bytes))?),
                DataType::Json => Value::Json(parse(<serde_json::Value as FromSql<sql_types::Json, Pg>>::from_sql(bytes))?),
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


impl Drop for ResultWrapper {
    fn drop(&mut self) {
        //drop it like it's hot
        unsafe { pq_sys::PQclear(self.p()) }
    }
}


fn final_execute(conn: &Conn, query: &str, params: Vec<data::Value>) -> Result<ResultWrapper, Error> {
    let conn_wrapper = ConnWrapper::new(&conn);


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
            Value::DateTime(x) => {
                let value = x;
                <chrono::NaiveDateTime as ToSql<sql_types::Timestamp, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Date(x) => {
                let value = x;
                <chrono::NaiveDate as ToSql<sql_types::Date, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::String(x) => {
                let value = x;
                <String as ToSql<sql_types::Text, Pg>>::to_sql(&value, &mut bytes)
            },
            Value::Binary(x) => {
                let value = x;
                <Vec<u8> as ToSql<sql_types::Binary, Pg>>::to_sql(&value, &mut bytes)
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

    let param_types: Vec<u32> = params.iter().map(|x| {
        match x {
            Value::Null => 0x0, //TODO: is this right?
            Value::Integer(_) => 0x17,
            Value::String(_) => 0x19,
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

    let internal_ptr = conn_wrapper.p();
    let result = unsafe {
        //TODO: you can cache this with `PQprepare`
        pq_sys::PQexecParams(
            internal_ptr,
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

impl DatabaseFunctions for Conn {
    fn exec(&self, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError> {
        let result = final_execute(self, query, params)
            .map_err(|err| {
                println!("Encountered error: {:?}", &err);
                DbError::Unknown
            })?;

        let data = result.get_rows_data()
            .map_err(|err| {
                println!("Encountered error: {:?}", &err);
                DbError::Unknown
            })?;

        let columns = result.get_column_names()
            .map_err(|err| {
                println!("Encountered error: {:?}", &err);
                DbError::Unknown
            })?;

        result.print_error();

        let table_data = data::RawTableData::new_and_fill(columns, data);

        println!("result: {:?}", &table_data);

        Ok(table_data)
    }
}