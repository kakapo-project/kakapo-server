
use objekt::Clone;

use super::schema::Schema;
use super::rows::Rows;
use super::query::{GetQuery, CreateQuery, UpdateQuery, DeleteQuery};

pub enum Error {
    TransactionError(Box<Transaction>, String),
    UsageError(String),
    SystemError(String),
}



pub trait Transaction: Clone {
    fn create_table(&self, schema: &Schema) -> Result<Box<Transaction>, Error>;
    fn delete_table(&self, id: &str, truncate: bool) -> Result<Box<Transaction>, Error>;

    fn schema(&self, id: &str) -> Result<Schema, Error>;
    fn get(&self, id: &str, query: &GetQuery) -> Result<Rows, Error>;
    fn create(&self, id: &str, query: &CreateQuery) -> Result<Box<Transaction>, Error>;
    fn update(&self, id: &str, query: &UpdateQuery) -> Result<Box<Transaction>, Error>;
    fn delete(&self, id: &str, query: &DeleteQuery) -> Result<Box<Transaction>, Error>;

    fn commit(&self) -> Result<(), Error>;
    fn rollback(&self) -> Result<(), Error>;
}
clone_trait_object!(Transaction);


pub trait Repository: Clone {
    fn transaction(&self) -> Result<Box<Transaction>, Error>;
}
clone_trait_object!(Repository);
