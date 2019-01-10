

use data;

use diesel;
use std::fmt;
use std;

use model::actions::error::Error as ActionError;
use actix_web::ResponseError;
use actix_web::HttpResponse;

//TODO: is this being used right now???

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(display = "Too many connections, or too many requests")]
    TooManyConnections,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
