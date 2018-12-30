

use data;

use diesel;
use std::fmt;
use std;

#[derive(Debug)]
pub enum Error {
    ScriptError(String),
    InvalidStateError,
    TableNotFound,
    QueryNotFound,
    ScriptNotFound,
    TooManyConnections,
    SerializationError,
    UnknownError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::ScriptError(x) => &x[..],
            Error::InvalidStateError => "The state of the data is broken",
            Error::TableNotFound => "Table could not be found",
            Error::QueryNotFound => "Query could not be found",
            Error::ScriptNotFound => "Script could not be found",
            Error::TooManyConnections => "Too many connections, or too many requests",
            Error::SerializationError => "Could not serialize data",
            Error::UnknownError => "Unknown error",
        }
    }
}
