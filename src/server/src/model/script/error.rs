
#[derive(Debug, Fail)]
pub enum ScriptError {
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "Internal error")]
    InternalError, //returns back the DatabaseError variant of diesel::result::Error
    #[fail(display = "Failed to deserialize")]
    DeserializationError,
    #[fail(display = "Failed to serialize")]
    SerializationError,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
