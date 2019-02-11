


#[derive(Debug, Fail, PartialEq, Eq)]
pub enum ScriptError {
    #[fail(display = "io error: {:?}", 0)]
    IOError(String),
    #[fail(display = "could not execute")]
    ExecuteError,
    #[fail(display = "runtime error: {:?}", 0)]
    RuntimeError(String),
}
