extern crate diesel;
extern crate serde_json;

mod column;
mod dummy_expression;
mod schema;
mod table;

pub use column::{Column, VecColumn, ValueList, DynamicValue, DynamicValueType};
pub use schema::Schema;
pub use table::Table;

pub fn table<T>(name: T) -> Table<T> {
    Table::new(name)
}

pub fn schema<T>(name: T) -> Schema<T> {
    Schema::new(name)
}

