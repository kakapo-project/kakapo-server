
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum EntityError {
    #[fail(display = "Internal error")]
    InternalError(String), //returns back the DatabaseError variant of diesel::result::Error
    #[fail(display = "Failed to deserialize")]
    DeserializationError,
    #[fail(display = "Failed to serialize")]
    SerializationError,
    #[fail(display = "Invalid state, something is really weird with the database")]
    InvalidState,
    #[fail(display = "No Columns found, every table must have at least one column")]
    NoColumns,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
