
use argonautica;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum UserManagementError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Not found")]
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

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum BroadcastError {
    #[fail(display = "Internal error")]
    InternalError(String), //returns back the DatabaseError variant of sql error
    #[fail(display = "Already subscribed")]
    AlreadySubscribed,
    #[fail(display = "Not yet subscribed")]
    NotSubscribed,
    #[fail(display = "User not found")]
    UserNotFound,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
