
use objekt::Clone;

use super::types::Identifier;
use super::table::Table;
use super::schema::Schema;
use super::query::{GetQuery, CreateQuery, UpdateQuery, DeleteQuery};

pub enum Error {
    TransactionError(Box<Transaction>, String),
    UsageError(String),
    SystemError(String),
}



pub trait Transaction: Clone {
    fn create_table(&self, schema: &Schema) -> Result<Box<Transaction>, Error>;
    fn delete_table(&self, id: &Identifier, truncate: bool) -> Result<Box<Transaction>, Error>;

    fn get(&self, id: &Identifier, query: &GetQuery) -> Result<Table, Error>;
    fn create(&self, id: &Identifier, query: &CreateQuery) -> Result<Box<Transaction>, Error>;
    fn update(&self, id: &Identifier, query: &UpdateQuery) -> Result<Box<Transaction>, Error>;
    fn delete(&self, id: &Identifier, query: &DeleteQuery) -> Result<Box<Transaction>, Error>;

    fn commit(&self) -> Result<(), Error>;
    fn rollback(&self) -> Result<(), Error>;
}
clone_trait_object!(Transaction);


pub trait Repository: Clone {
    fn transaction(&self) -> Result<Box<Transaction>, Error>;
}
clone_trait_object!(Repository);
