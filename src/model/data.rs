
use std::collections::HashMap;
use std::collections::BTreeMap;

use chrono::prelude::*;
use chrono::DateTime;

use serde_json;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OnDuplicate {
    Update,
    Ignore,
    Fail,
}

impl Default for OnDuplicate {
    fn default() -> Self {
        OnDuplicate::Update
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CreationMethod {
    Update,
    IgnoreIfExists,
    FailIfExists,
    FailIfNotExists,
}


impl Default for CreationMethod {
    fn default() -> Self {
        CreationMethod::Update
    }
}

impl OnDuplicate {
    pub fn into_method(self) -> CreationMethod {
        match self {
            OnDuplicate::Update => CreationMethod::Update,
            OnDuplicate::Ignore => CreationMethod::IgnoreIfExists,
            OnDuplicate::Fail => CreationMethod::FailIfExists,
        }
    }
}


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

//TODO: Add output format: indexed, rows (default), flat rows, columns, schema
#[derive(Clone, Copy, Debug)]
pub enum TableDataFormat {
    Rows,
    FlatRows,
}

impl Serialize for TableDataFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            TableDataFormat::Rows => serializer.serialize_str("rows"),
            TableDataFormat::FlatRows => serializer.serialize_str("flatRows"),
        }
    }
}


impl<'de> Deserialize<'de> for TableDataFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct TableDataFormatVisitor;

        impl<'de> de::Visitor<'de> for TableDataFormatVisitor {
            type Value = TableDataFormat;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("expecting `rows` or `flatRows`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                match value {
                    "rows" => Ok(TableDataFormat::Rows),
                    "flatRows" => Ok(TableDataFormat::FlatRows),
                    _ => Err(E::custom(format!("unrecognized variant")))
                }
            }
        }
        deserializer.deserialize_str(TableDataFormatVisitor)
    }
}

impl Default for TableDataFormat {
    fn default() -> Self {
        TableDataFormat::Rows
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


impl RowData {
    pub fn into_table_data(self) -> TableData {
        match self {
            RowData::RowData(x) => TableData::RowsData(vec![x]),
            RowData::RowsFlatData { columns, data } => TableData::RowsFlatData {
                columns: columns,
                data: vec![data],
            },
        }
    }
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

    pub fn into_rows_flat_data(self) -> TableData {
        match self {
            TableData::RowsFlatData { .. } => self,
            TableData::RowsData(rows) => { //This is actually slow
                let mut columns = BTreeMap::new();
                for row in rows.iter() {
                    for row_column in row.keys() {
                        columns.insert(row_column.to_owned(), ());
                    }
                }
                let mut data = vec![];
                //TODO: handle case for missing values, right now it just puts null, but it should handle it as different
                for row in rows.iter() {
                    let mut new_row = vec![];
                    for key in columns.keys() {
                        let new_value = match row.get(key) {
                            Some(value) => value.to_owned(),
                            None => Value::Null,
                        };
                        new_row.push(new_value);
                    }
                    data.push(new_row);
                }

                TableData::RowsFlatData {
                    columns: columns.keys().cloned().collect::<Vec<String>>(),
                    data: data,
                }
            },
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
        #[serde(default)]
        constraint: Vec<Constraint>,
    },
    Remove {
        #[serde(default)]
        column: Vec<String>,
        #[serde(default)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryWithData {
    pub query: Query,
    pub data: TableData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub statement: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum QueryParams {
    //TODO: implement named parameters, unfortunately postgres doesn't have named parameters so...
    //Named(BTreeMap<String, Value>),
    Unnamed(Vec<Value>),
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams::Unnamed(vec![])
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub text: String,
}

pub type ScriptParam = Option<serde_json::Value>;