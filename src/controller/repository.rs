
use super::types::Identifier;
use super::table::Table;
use super::schema::Schema;
use super::rows::{RowInsertion, RowUpdate, RowDeletion};

pub enum Error {
    TransactionError(Box<Transaction>, String),
    UsageError(String),
    SystemError(String),
}

pub trait Transaction {
    fn create_table(&self, schema: &Schema) -> Result<Box<Transaction>, Error>;
    fn delete_table(&self, id: &Identifier, truncate: bool) -> Result<Box<Transaction>, Error>;

    fn get(&self, id: &Identifier) -> Result<Table, Error>;
    fn create(&self, id: &Identifier, rows: &RowInsertion) -> Result<Box<Transaction>, Error>;
    fn update(&self, id: &Identifier, rows: &RowUpdate) -> Result<Box<Transaction>, Error>;
    fn delete(&self, id: &Identifier, rows: &RowDeletion) -> Result<Box<Transaction>, Error>;

    fn commit(&self) -> Result<(), Error>;
    fn rollback(&self) -> Result<(), Error>;
}

pub trait Repository {
    fn transaction(&self) -> Result<Box<Transaction>, Error>;
}