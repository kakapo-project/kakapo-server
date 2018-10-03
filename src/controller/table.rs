

use super::rows::Rows;
use super::types::DataType;
use super::schema::{Schema, Column};

/// Table
#[derive(Clone)]
pub struct Table {
    schema: Box<Schema>,
    rows: Box<Rows>,
}

impl Table {
    pub fn new(schema: &Box<Schema>, rows: &Box<Rows>) -> Self {
        Table {
            schema: schema.to_owned(),
            rows: rows.to_owned(),
        }
    }

    pub fn get_column_info(&self) -> Vec<Column> {
        let Table { schema, .. } = self;
        schema.get_columns()
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        let column_info = self.get_column_info();
        let column_index = column_info
            .iter()
            .position(|column| column.get_name() == column_name.to_owned());
        column_index
    }

    pub fn get_rows(&self) -> Rows {
        let Table { rows, .. } = self;
        Box::leak(rows.to_owned()).to_owned()
    }
}