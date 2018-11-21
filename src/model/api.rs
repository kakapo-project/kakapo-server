
use super::data;

use diesel;
use std::fmt;
use std;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostTable {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub action: data::SchemaModification,
}

pub type TableData = data::TableData;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum GetTablesResult {
    Tables(Vec<data::Table>), //unrolls the tables
    DetailedTables(Vec<data::DetailedTable>), //Has the full history of the tables
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum GetTableResult {
    Table(data::Table), //unrolls
    DetailedTable(data::DetailedTable), //full history
}

#[derive(Debug, Serialize)]
pub struct CreateTableResult(pub data::Table);

#[derive(Debug, Serialize)]
pub struct GetTableDataResult(pub data::TableData);  //TODO: just need the data, give the user the option to have the schema as well maybe?

#[derive(Debug, Serialize)]
pub struct InsertTableDataResult(pub data::TableData);

#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
    InvalidStateError,
    TableNotFound,
    TooManyConnections,
    SerializationError,
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
            Error::TableNotFound => "Table could not be found",
            Error::TooManyConnections => "Too many connections, or too many requests",
            Error::SerializationError => "Could not serialize data",
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
    GetTableData {
        #[serde(skip_serializing_if = "Option::is_none")]
        begin: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<usize>,
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

