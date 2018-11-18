
use std::collections::HashMap;
use std::collections::BTreeMap;

use chrono::prelude::*;
use chrono::DateTime;

use serde_json;


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    String,
    Integer,
    Json,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum IndexableValue {
    String(String),
    Integer(i64),
    //TODO: allow for multiple indexable values if the multiple keys exists
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Value {
    Null,
    String(String),
    Integer(i64),
    Json(serde_json::Value),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum RowData {
    RowData(BTreeMap<String, Value>),
    RowsFlatData {
        columns: Vec<String>,
        data: Vec<Value>,
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum TableData {
    //IndexedData(BTreeMap<IndexableValue, RowData>),
    RowsData(Vec<BTreeMap<String, Value>>),
    //ColumnData(BTreeMap<String, Vec<Value>>),
    RowsFlatData {
        columns: Vec<String>,
        data: Vec<Vec<Value>>,
    },
}

impl TableData {
    fn get_rows_data_from_rows_flat_data(columns: Vec<String>, data: Vec<Vec<Value>>) -> Vec<BTreeMap<String, Value>>{
        data.iter().map(|row| {

            let mut row_data = BTreeMap::new();
            for (name, value) in columns.iter().zip(row) {
                row_data.insert(name.to_owned(), value.to_owned());
            }

            row_data
        }).collect()
    }

    pub fn into_rows_data(self) -> TableData {
        match self {
            TableData::RowsFlatData { columns, data } => {
                let rows_data = TableData::get_rows_data_from_rows_flat_data(columns, data);
                TableData::RowsData(rows_data)
            },
            TableData::RowsData(_) => self,
        }
    }

    pub fn into_rows_data_vec(self) -> Vec<BTreeMap<String, Value>> {
        match self {
            TableData::RowsFlatData { columns, data } => {
                TableData::get_rows_data_from_rows_flat_data(columns, data)
            },
            TableData::RowsData(x) => x,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "op")]
pub enum Expression {
    Equals {
        column: String,
        value: Value
    },
    NotEqual {
        column: String,
        value: Value
    },
    GreaterThan {
        column: String,
        value: Value,
    },
    LessThan {
        column: String,
        value: Value,
    },
    In {
        column: String,
        values: Vec<Value>,
    },
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Constraint {
    Key(String),
    //KeyTogether(Vec<String>),
    Unique(String),
    UniqueTogether(Vec<String>),

    Check(Expression),

    Reference {
        column: String,
        #[serde(rename = "foreignTable")]
        foreign_table: String,
        #[serde(rename = "foreignColumn")]
        foreign_column: String,
    },
    ReferenceTogether {
        columns: Vec<String>,
        #[serde(rename = "foreignTable")]
        foreign_table: String,
        #[serde(rename = "foreignColumns")]
        foreign_columns: Vec<String>,
    },

}


// This is the same as SchemaModification::Create
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaState {
    pub columns: Vec<Column>,
    pub constraint: Vec<Constraint>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SchemaModification {
    Create {
        columns: Vec<Column>,
        constraint: Vec<Constraint>,
    },
    Remove {
        column: Vec<String>,
        constraint: Vec<Constraint>,
    },
    Rename {
        mapping: HashMap<String, String>,
    },
    Raw {
        up: String,
        down: String,
    },
    Nop,
    Revert,
    Delete,
}
impl Default for SchemaModification {
    fn default() -> Self {
        SchemaModification::Nop
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaModificationCommit {
    pub date: NaiveDateTime,
    pub action: SchemaModification,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedTable {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub schema: Vec<SchemaModificationCommit>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub schema: SchemaState,
}

impl Table {
    pub fn get_key(&self) -> Option<String> {
        let constraints = &self.schema.constraint;
        let keys: Vec<String> = constraints.iter().flat_map(|constraint| {
            match constraint {
                Constraint::Key(x) => vec![x],
                _ => vec![],
            }
        }).cloned().collect();

        if keys.len() > 1 {
            println!("ERROR: several keys exists, something is wrong with this table");
        }
        keys.iter().nth(0).map(|x| x.to_owned())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableWithData {
    pub table: Table,
    pub data: TableData,
}
