

use data;

use diesel;
use std::fmt;
use std;

use model::actions::error::Error as ActionError;
use actix_web::ResponseError;
use actix_web::HttpResponse;

#[derive(Debug, Fail, Serialize)]
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

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        unimplemented!();
        /*
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": self.to_string() })).unwrap())
           */
    }
}
