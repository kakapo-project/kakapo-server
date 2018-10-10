
use super::data;

use diesel;
use std::fmt;
use std;

#[derive(Deserialize, Debug)]
pub struct PostTable {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub action: data::SchemaAction,
    #[serde(default)]
    pub delete: bool,
}

#[derive(Debug)]
pub enum GetTablesResult {
    Tables(Vec<data::Table>),
    DetailedTables(Vec<data::DetailedTable>),
}

#[derive(Debug)]
pub enum GetTableResult {
    Table(data::Table),
    DetailedTable(data::DetailedTable),
}


#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
    UnknownError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::DatabaseError(x) => x.description(),
            Error::UnknownError => "Unknown error",
        }
    }
}