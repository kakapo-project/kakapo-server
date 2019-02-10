use database::error::DbError;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum QueryError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Internal error")]
    InternalError, //returns back the DatabaseError variant of sql error
    #[fail(display = "Failed to deserialize")]
    DeserializationError,
    #[fail(display = "Failed to serialize")]
    SerializationError,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

impl QueryError {
    pub fn db_error(err: DbError) -> Self {
        unimplemented!()
    }
}