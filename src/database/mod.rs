use connection::executor::Conn;
use data;
use std::fmt::Debug;

#[derive(Debug, Fail)]
pub enum DbError {
    #[fail(display = "value already exists")]
    AlreadyExists,
    #[fail(display = "value does not exists")]
    NotFound,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

pub trait DatabaseFunctions {
    fn exec(&self, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError>;
}

impl DatabaseFunctions for Conn {
    fn exec(&self, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError> {
        unimplemented!()
    }
}