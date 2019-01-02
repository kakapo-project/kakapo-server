

use data;

use diesel;
use std::fmt;
use std;

use model::actions::error::Error as ActionError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Too many connections, or too many requests")]
    TooManyConnections,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}


impl From<ActionError> for Error {
    fn from(err: ActionError) -> Self {
        Error::Unknown
    }
}