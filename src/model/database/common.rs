

struct Database;
trait DatabaseFunctions {
    fn exec(&self) -> ();
}

impl DatabaseFunctions for Database {
    fn exec(&self) -> () {
        unimplemented!()
    }
}