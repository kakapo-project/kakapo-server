
use data;

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

pub struct NewTable {
    pub name: String,
    pub description: String,
    pub action: data::SchemaModification,
}


impl PostTable {
    pub fn into_new(self) -> NewTable {
        NewTable {
            name: self.name,
            description: self.description,
            action: self.action,
        }
    }
}

impl NewTable {
    pub fn deletion(name: String) -> Self {
        NewTable {
            name: name,
            description: "".to_string(),
            action: data::SchemaModification::Delete,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostQuery {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub statement: String,
}

pub struct NewQuery {
    pub name: String,
    pub description: String,
    pub statement: String,
    pub for_deletion: bool,
}

impl PostQuery {
    pub fn into_new(self) -> NewQuery {
        NewQuery {
            name: self.name,
            description: self.description,
            statement: self.statement,
            for_deletion: false,
        }
    }
}

impl NewQuery {
    pub fn deletion(name: String) -> Self {
        NewQuery {
            name: name,
            description: "".to_string(),
            statement: "".to_string(),
            for_deletion: true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostScript {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub text: String,
}

pub struct NewScript {
    pub name: String,
    pub description: String,
    pub text: String,
    pub for_deletion: bool,
}

impl PostScript {
    pub fn into_new(self) -> NewScript {
        NewScript {
            name: self.name,
            description: self.description,
            text: self.text,
            for_deletion: false,
        }
    }
}

impl NewScript {
    pub fn deletion(name: String) -> Self {
        NewScript {
            name: name,
            description: "".to_string(),
            text: "".to_string(),
            for_deletion: true,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PutTable {
    #[serde(default)]
    pub description: String,
    pub action: data::SchemaModification,
}

impl PutTable {
    pub fn with_name(self, name: String) -> NewTable {
        NewTable {
            name: name,
            description: self.description,
            action: self.action,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PutQuery {
    #[serde(default)]
    pub description: String,
    pub statement: String,
}

impl PutQuery {
    pub fn into_new(self, name: String) -> NewQuery {
        NewQuery {
            name: name,
            description: self.description,
            statement: self.statement,
            for_deletion: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PutScript {
    #[serde(default)]
    pub description: String,
    pub text: String,
}

impl PutScript {
    pub fn into_new(self, name: String) -> NewScript {
        NewScript {
            name: name,
            description: self.description,
            text: self.text,
            for_deletion: false,
        }
    }
}

pub type OnDuplicate = data::OnDuplicate;
pub type CreationMethod = data::CreationMethod;

pub type TableData = data::TableData;
pub type RowData = data::RowData;
pub type TableDataFormat = data::TableDataFormat;
pub type QueryParams = data::QueryParams;
pub type ScriptParam = data::ScriptParam;


pub const FLAT_TABLE_DATA_FORMAT: TableDataFormat = data::TableDataFormat::FlatRows;

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
pub struct GetQueriesResult(pub Vec<data::Query>);

#[derive(Debug, Serialize)]
pub struct GetQueryResult(pub data::Query);

#[derive(Debug, Serialize)]
pub struct GetScriptsResult(pub Vec<data::Script>);

#[derive(Debug, Serialize)]
pub struct GetScriptResult(pub data::Script);


#[derive(Debug, Serialize)]
pub struct CreateTableResult(pub data::Table);

#[derive(Debug, Serialize)]
pub struct CreateQueryResult(pub data::Query);

#[derive(Debug, Serialize)]
pub struct CreateScriptResult(pub data::Script);

#[derive(Debug, Serialize)]
pub struct GetTableDataResult(pub data::TableData);  //TODO: just need the data, give the user the option to have the schema as well maybe?

#[derive(Debug, Serialize)]
pub struct InsertTableDataResult(pub data::TableData);

#[derive(Debug, Serialize)]
pub struct UpdateTableDataResult(pub data::RowData);

#[derive(Debug, Serialize)]
pub struct DeleteTableDataResult(pub data::RowData);

#[derive(Debug, Serialize)]
pub struct RunQueryResult(pub data::TableData);

#[derive(Debug, Serialize)]
pub struct RunScriptResult(pub serde_json::Value);

#[derive(Debug)]
pub enum Error {
    DatabaseError(diesel::result::Error),
    ScriptError(String),
    InvalidStateError,
    TableNotFound,
    QueryNotFound,
    ScriptNotFound,
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
            Error::ScriptError(x) => &x[..],
            Error::InvalidStateError => "The state of the data is broken",
            Error::TableNotFound => "Table could not be found",
            Error::QueryNotFound => "Query could not be found",
            Error::ScriptNotFound => "Script could not be found",
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
    },
    Update {
        data: data::RowData,
        key: String,
    },
    Create {
        data: data::RowData,
    },
    Delete {
        key: String,
    },

}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum QuerySessionRequest {
    GetQuery,
    PostQuery {
        data: PostQuery
    },
    RunQuery {
        #[serde(skip_serializing_if = "Option::is_none")]
        begin: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end: Option<usize>,
        #[serde(default)]
        params: QueryParams,
    },

}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum ScriptSessionRequest {
    GetScript,
    PostScript {
        data: PostScript,
    },
    RunScript {
        #[serde(default)]
        params: Option<serde_json::Value>,
    },

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

