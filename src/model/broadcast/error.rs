
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum BroadcastError {
    #[fail(display = "Failed to serialize")]
    SerializationError,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}