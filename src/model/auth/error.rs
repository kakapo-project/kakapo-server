
#[derive(Debug, Fail)]
pub enum UserManagementError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Internal error")]
    InternalError(String), //returns back the DatabaseError variant of diesel::result::Error
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
