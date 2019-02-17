


#[derive(Debug, Fail, PartialEq, Eq)]
pub enum ScriptError {
    #[fail(display = "io error: {:?}", 0)]
    IOError(String),
    #[fail(display = "could not execute {:?}", 0)]
    ExecuteError(String),
    #[fail(display = "runtime error: {:?}", 0)]
    RuntimeError(String),
    #[fail(display = "An unknown error occurred")]
    Unknown,
}
