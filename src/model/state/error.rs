
use argonautica;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum UserManagementError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "Unauthorized")]
    Unauthorized,
    #[fail(display = "{:?}", 0)]
    AuthenticationError(String),
    #[fail(display = "{:?}", 0)]
    HashError(String),
    #[fail(display = "Internal error")]
    InternalError(String), //returns back the DatabaseError variant of sql error
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
