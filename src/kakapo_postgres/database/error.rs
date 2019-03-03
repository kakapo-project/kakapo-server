
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum DbError {
    #[fail(display = "value already exists")]
    AlreadyExists,
    #[fail(display = "value does not exists")]
    NotFound,
    #[fail(display = "{}", 0)]
    QueryError(String),
    #[fail(display = "query is empty")]
    EmptyQuery,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

