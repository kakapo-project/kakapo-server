
struct State {
    database: String,
    user: String,
}

struct Error {

}

trait Action {
    type Return;
    fn call(&self, state: State) -> Result<Self::Return, Error>;
}

struct Authorized<A> {
    action: A,
}

impl<A:Action> Action for Authorized<A> {
    type Return = A::Return;

    fn call(&self, state: State) -> Result<Self::Return, Error> {
        println!("authorizing...");
        self.action.call(state)
    }
}


struct GetAllTables;

impl Action for GetAllTables {
    type Return = Vec<i32>;

    fn call(&self, state: State) -> Result<Self::Return, Error> {
        println!("getting all tables");
        Ok(vec![1, 2, 3])
    }
}

struct GetOneTable;

impl Action for GetOneTable {
    type Return = i32;

    fn call(&self, state: State) -> Result<Self::Return, Error> {
        println!("getting all tables");
        Ok(4)
    }
}