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

pub struct Database;
pub trait DatabaseFunctions
    where Self: Send,
{
    fn exec(conn: &Conn, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError>;
}

impl DatabaseFunctions for Database {
    fn exec(conn: &Conn, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError> {
        unimplemented!()
    }
}