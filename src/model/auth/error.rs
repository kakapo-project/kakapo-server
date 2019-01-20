
#[derive(Debug, Fail)]
pub enum UserManagementError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
