

use super::rows::RowContainer;
use super::types::{Column, DataType};
use super::schema::Schema;

/// Table
#[derive(Clone)]
pub struct Table {
    schema: Box<Schema>,
    rows: Box<RowContainer>,
}

impl Table {
    pub fn new(schema: &Box<Schema>, rows: &Box<RowContainer>) -> Self {
        Table {
            schema: schema.to_owned(),
            rows: rows.to_owned(),
        }
    }
}