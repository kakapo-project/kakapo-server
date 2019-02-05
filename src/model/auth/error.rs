
use argonautica;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum UserManagementError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "{:?}", 0)]
    HashError(argonautica::Error),
    #[fail(display = "Internal error")]
    InternalError(String), //returns back the DatabaseError variant of diesel::result::Error
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
