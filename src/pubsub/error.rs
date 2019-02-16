
#[derive(Debug, Fail, PartialEq, Eq)]
pub enum BroadcastError {
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
