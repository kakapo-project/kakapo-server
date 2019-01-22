
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
use data::*;

#[derive(Debug, Fail)]
pub enum DataError {
    #[fail(display = "mismatched columns")]
    MismatchedColumns,
}

impl IndexableValue {
    pub fn into_value(self) -> Value {
        match self {
            IndexableValue::Integer(x) => Value::Integer(x),
            IndexableValue::String(x) => Value::String(x),
        }
    }
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

impl RawTableDataData {
    pub fn get_values(self) -> Vec<Value> {
        self.values
    }

    pub fn get_keys(self) -> Vec<IndexableValue> {
        self.keys
    }
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

impl KeyedTableData {
    pub fn normalize(&self) -> (ObjectKeys, ObjectValues) {
        unimplemented!()
    }
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

impl TableData {
    pub fn normalize(&self) -> ObjectValues {
        match self {
            TableData::Data(object_values) => object_values.to_owned(),
            TableData::FlatData(tabular_values) => {
                let columns = tabular_values.to_owned().get_columns();
                let data = tabular_values.to_owned().get_data();

                let mut object_data = vec![];
                for row in data {
                    let mut object_row = LinkedHashMap::new();
                    for (col_name, value) in columns.iter().zip(row) {
                        object_row.insert(col_name.to_owned(), value);
                    }
                    object_data.push(object_row);
                }

                ObjectValues::new(object_data)
            },
            TableData::Keyed(keyed_table_data) => {
                let (object_keys, object_values) = keyed_table_data.normalize();

                object_values
            },
        }
    }
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

impl TabularKeys {
    pub fn get_columns(self) -> Vec<String> {
        self.columns
    }

    pub fn get_data(self) -> Vec<Vec<IndexableValue>> {
        self.data
    }
}

impl ObjectValues {
    pub fn new(data: Vec<LinkedHashMap<String, Value>>) -> Self {
        ObjectValues(data)
    }

    pub fn as_list(&self) -> &Vec<LinkedHashMap<String, Value>> {
        &self.0
    }
}

impl ObjectKeys {
    pub fn new(data: Vec<LinkedHashMap<String, IndexableValue>>) -> Self {
        ObjectKeys(data)
    }

    pub fn as_list(&self) -> &Vec<LinkedHashMap<String, IndexableValue>> {
        &self.0
    }
}


impl Table {
    pub fn get_column_names(&self) -> Vec<String> {
        unimplemented!()
    }
}

impl QueryParams {
    pub fn value_list(&self) -> Vec<Value> {
        match self {
            QueryParams::Unnamed(x) => x.to_owned()
        }
    }
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams::Unnamed(vec![])
    }
}