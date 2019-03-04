
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum DatastoreError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Internal error")]
    InternalError, //returns back the DatabaseError variant of sql error
    #[fail(display = "Failed to deserialize")]
    DeserializationError,
    #[fail(display = "Failed to serialize")]
    SerializationError,
    #[fail(display = "domain {} does not exist", 0)]
    DomainNotFound(String),
    #[fail(display = "domain not supported")]
    NotSupported,
    #[fail(display = "File System Error {:?}", 0)]
    FileSystemError(String),
    #[fail(display = "Invalid state, something is really weird with the database")]
    InvalidState,
    #[fail(display = "No Columns found, every table must have at least one column")]
    NoColumns,
    #[fail(display = "{}", 0)]
    DbError(String),
    #[fail(display = "An unknown error occurred")]
    Unknown,
}