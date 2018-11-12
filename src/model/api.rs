
use super::data;

use diesel;
use std::fmt;
use std;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostTable {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub action: data::SchemaModification,
}

#[derive(Debug)]
pub enum GetTablesResult {
    Tables(Vec<data::Table>), //unrolls the tables
    DetailedTables(Vec<data::DetailedTable>), //Has the full history of the tables
}

#[derive(Debug)]
pub enum GetTableResult {
    Table(data::Table), //unrolls
    DetailedTable(data::DetailedTable), //full history
}

#[derive(Debug)]
pub struct CreateTableResult(pub data::Table);

#[derive(Debug)]
pub struct GetTableDataResult(pub data::TableWithData);

#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
    InvalidStateError,
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
            Error::InvalidStateError => "The state of the data is broken",
            Error::UnknownError => "Unknown error",
        }
    }
}

// For websockets
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum TableSessionRequest {
    GetTable,
    GetAllTableData {
        begin: usize,
        #[serde(rename = "chunkSize")]
        chunk_size: usize,
    },
    GetTableData {
        begin: usize,
        end: usize,
        #[serde(rename = "chunkSize")]
        chunk_size: usize,
    },
    Update(data::RowData),
    Create(data::RowData),
    Delete(data::IndexableValue),

}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum TableSessionResponse {
    Table {
        schema: data::Table,
    },
    TableData {
        table: data::TableData,
    },
    EndOfTableData,
    InvalidateAll,
    Deleted {
        begin: usize,
        end: usize,
    },
    Updated {
        begin: usize,
        end: usize,
    },
    New {
        begin: usize,
        end: usize,
    },

}

