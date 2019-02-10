use connection::executor::Conn;
use data;

#[derive(Debug, Fail)]
pub enum DbError {
    #[fail(display = "value already exists")]
    AlreadyExists,
    #[fail(display = "value does not exists")]
    NotFound,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

