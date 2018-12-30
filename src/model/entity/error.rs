
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum DBError {
    InternalError, //returns back the DatabaseError variant of diesel::result::Error
    DeserializationError,
    SerializationError,
    InvalidState,
    Unknown,
}
