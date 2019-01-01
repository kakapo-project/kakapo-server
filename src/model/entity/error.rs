
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum EntityError {
    InternalError, //returns back the DatabaseError variant of diesel::result::Error
    DeserializationError,
    SerializationError,
    InvalidState,
    Unknown,
}
