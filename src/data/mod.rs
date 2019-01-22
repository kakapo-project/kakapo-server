
use std::collections::HashMap;
use std::collections::BTreeMap;

use chrono::prelude::*;

use serde_json;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de;
use std::fmt;
use linked_hash_map::LinkedHashMap;
use linked_hash_map::serde::LinkedHashMapVisitor;
use data::utils::TableDataFormat;

pub mod utils;
pub mod auth;
pub mod dbdata;
pub mod schema;
pub mod conversion;
pub mod methods;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataType {
    SmallInteger,
    Integer,
    BigInteger,
    //Decimal { precision: u32, scale: u32 },
    Float,
    DoubleFloat,

    String,
    VarChar { length: u32 },

    Byte,

    Timestamp { with_tz: bool },
    Date,
    Time { with_tz: bool },
    //TimeInterval,

    Boolean,

    Json,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum IndexableValue {
    Integer(i64),
    String(String),
}

impl IndexableValue {
    pub fn into_value(self) -> Value {
        match self {
            IndexableValue::Integer(x) => Value::Integer(x),
            IndexableValue::String(x) => Value::String(x),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Value {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    DateTime(chrono::NaiveDateTime), //TODO: serialize
    Date(chrono::NaiveDate),
    Binary(Vec<u8>),
    Json(serde_json::Value),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTableDataColumns {
    keys: Vec<String>,
    values: Vec<String>
}

impl RawTableDataColumns {

    pub fn new(keys: Vec<String>, values: Vec<String>) -> Self {
        Self { keys, values }
    }

    pub fn get_value_columns(self) -> Vec<String> {
        self.values
    }

    pub fn get_key_columns(self) -> Vec<String> {
        self.keys
    }

    pub fn value_columns(&self) -> &Vec<String> {
        &self.values
    }

    pub fn key_columns(&self) -> &Vec<String> {
        &self.keys
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTableDataData {
    keys: Vec<IndexableValue>,
    values: Vec<Value>
}

impl RawTableDataData {
    pub fn get_values(self) -> Vec<Value> {
        self.values
    }

    pub fn get_keys(self) -> Vec<IndexableValue> {
        self.keys
    }
}

/// Default return value from a query
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawTableData  {
    columns: RawTableDataColumns,
    data: Vec<RawTableDataData>,
}

#[derive(Debug, Fail)]
pub enum DataError {
    #[fail(display = "mismatched columns")]
    MismatchedColumns,
}

impl RawTableData {
    pub fn new(key_names: Vec<String>, value_names: Vec<String>) -> Self {
        Self {
            columns: RawTableDataColumns::new(key_names, value_names),
            data: vec![],
        }
    }

    pub fn append(&mut self, other: Self) -> Result<(), DataError> {
        if self.columns.value_columns() != other.columns.value_columns() {
            return Err(DataError::MismatchedColumns)
        }

        if self.columns.key_columns() != other.columns.key_columns() {
            return Err(DataError::MismatchedColumns)
        }

        self.data.extend(other.data);
        Ok(())
    }

    pub fn format_with(self, format: &TableDataFormat) -> TableData {
        let col_names = self.columns.get_value_columns();

        match format {
            TableDataFormat::Rows => {
                let mut objects = vec![];
                for table_row in self.data {
                    let mut row = LinkedHashMap::new();
                    for (col_name, value) in col_names.iter().zip(table_row.get_values()) {
                        row.insert(col_name.to_owned(), value);
                    }
                    objects.push(row);
                }

                TableData::Data(ObjectValues(objects))
            },
            TableDataFormat::FlatRows => {
                let data = self.data.into_iter()
                    .map(|x| x.get_values())
                    .collect();
                TableData::FlatData(TabularValues::new(col_names, data))
            }
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePairObject {
    keys: LinkedHashMap<String, IndexableValue>,
    values: LinkedHashMap<String, Value>,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum KeyedTableData {
    /// ```
    /// {
    ///   42: {
    ///     "message": "hello world",
    ///     "category": "greeting",
    ///   },
    ///   43: {
    ///     "message": "goodbye world",
    ///     "category": "farewell",
    ///   }
    /// }
    /// ```
    Simplified(LinkedHashMap<
        IndexableValue,
        LinkedHashMap<String, Value>>), //can only be used if only one key exists
    /// ```
    /// [
    ///   {
    ///     "keys": {
    ///       "id": 42,
    ///     },
    ///     "values": {
    ///       "message": "hello world",
    ///       "category": "greeting",
    ///     }
    ///   },
    ///   {
    ///     "keys": {
    ///       "id": 43,
    ///     },
    ///     "values": {
    ///       "message": "goodbye world",
    ///       "category": "farewell",
    ///     }
    ///   }
    /// ]
    /// ```
    Data(Vec<KeyValuePairObject>),
    /// ```
    /// {
    ///   "columns": {
    ///     "keys": [ "id" ],
    ///     "values": [ "message", "category" ]
    ///   },
    ///   "data": [
    ///     {
    ///       "keys": [ 42 ],
    ///       "values": [ "hello world", "greeting" ]
    ///     },
    ///     {
    ///       "keys": [ 43 ],
    ///       "values": [ "goodbye world", "farewell" ]
    ///     }
    ///   ]
    /// }
    /// ```
    FlatData(RawTableData), //default output format
}

impl KeyedTableData {
    pub fn normalize(&self) -> (ObjectKeys, ObjectValues) {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum KeyData {
    Data(ObjectKeys),
    FlatData(TabularKeys),
    Keyed(KeyedTableData),
}

impl KeyData {
    pub fn normalize(&self) -> ObjectKeys {
        match self {
            KeyData::Data(object_keys) => object_keys.to_owned(),
            KeyData::FlatData(tabular_keys) => {
                let columns = tabular_keys.to_owned().get_columns();
                let data = tabular_keys.to_owned().get_data();

                let mut object_data = vec![];
                for row in data {
                    let mut object_row = LinkedHashMap::new();
                    for (col_name, value) in columns.iter().zip(row) {
                        object_row.insert(col_name.to_owned(), value);
                    }
                    object_data.push(object_row);
                }

                ObjectKeys::new(object_data)
            },
            KeyData::Keyed(keyed_table_data) => {
                let (object_keys, object_values) = keyed_table_data.normalize();

                object_keys
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum TableData {
    /// ```
    /// [
    ///   {
    ///     "id": 42,
    ///     "message": "hello world",
    ///     "category": "greeting",
    ///   },
    ///   {
    ///     "id": 43,
    ///     "message": "goodbye world",
    ///     "category": "farewell",
    ///   }
    /// ]
    /// ```
    Data(ObjectValues),
    //ColumnData(BTreeMap<String, Vec<Value>>),
    /// ```
    /// {
    ///   "columns": [ "id", "message", "category" ],
    ///   "data": [
    ///     [ 42, "hello world", "greeting" ],
    ///     [ 43, "goodbye world", "farewell" ],
    ///  ]
    /// }
    /// ```
    FlatData(TabularValues),
    Keyed(KeyedTableData),
}

impl TableData {
    pub fn normalize(&self) -> ObjectValues {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabularValues {
    columns: Vec<String>,
    data: Vec<Vec<Value>>,
}

impl TabularValues {
    pub fn new(columns: Vec<String>, data: Vec<Vec<Value>>) -> Self {
        Self { columns, data }
    }

    pub fn get_columns(self) -> Vec<String> {
        self.columns
    }

    pub fn get_data(self) -> Vec<Vec<Value>> {
        self.data
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabularKeys {
    columns: Vec<String>,
    data: Vec<Vec<IndexableValue>>,
}

impl TabularKeys {
    pub fn get_columns(self) -> Vec<String> {
        self.columns
    }

    pub fn get_data(self) -> Vec<Vec<IndexableValue>> {
        self.data
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectValues(Vec<LinkedHashMap<String, Value>>);

impl ObjectValues {
    pub fn new(data: Vec<LinkedHashMap<String, Value>>) -> Self {
        ObjectValues(data)
    }

    pub fn as_list(&self) -> &Vec<LinkedHashMap<String, Value>> {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectKeys(Vec<LinkedHashMap<String, IndexableValue>>);

impl ObjectKeys {
    pub fn new(data: Vec<LinkedHashMap<String, IndexableValue>>) -> Self {
        ObjectKeys(data)
    }

    pub fn as_list(&self) -> &Vec<LinkedHashMap<String, IndexableValue>> {
        &self.0
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
pub struct Table {
    pub name: String, //TODO: make sure this is an alphanumeric, otherwise SQL injection!
    pub description: String,
    pub schema: SchemaState,
}

impl Table {
    pub fn get_column_names(&self) -> Vec<String> {
        unimplemented!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum QueryParams {
    //TODO: implement named parameters, unfortunately postgres doesn't have named parameters so...
    //Named(BTreeMap<String, Value>),
    Unnamed(Vec<Value>),
}

impl QueryParams {
    pub fn value_list(&self) -> Vec<Value> {
        match self {
            QueryParams::Unnamed(x) => x.to_owned()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub statement: String,
}

pub type ScriptParam = Option<serde_json::Value>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub text: String,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams::Unnamed(vec![])
    }
}

/*
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
*/