use connection::executor::Conn;

pub struct Database;
pub trait DatabaseFunctions {
    fn exec(conn: &Conn, query: &str) -> ();
}

impl DatabaseFunctions for Database {
    fn exec(conn: &Conn, query: &str) -> () {
        unimplemented!()
    }
}