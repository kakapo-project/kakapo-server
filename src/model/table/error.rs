

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum TableQueryError {
    InternalError, //returns back the DatabaseError variant of diesel::result::Error
    DeserializationError,
    SerializationError,
    Unknown,
}
