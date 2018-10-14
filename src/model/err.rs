
#[derive(Debug, Fail)]
pub enum StateError {
    #[fail(display = "cannot revert an already empty state")]
    RevertError,
}