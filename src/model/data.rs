
use chrono::prelude::*;
use chrono::DateTime;

pub enum DataType {
    String,
    Integer,
    Json,
}

pub enum Value {

}

pub struct Column {
    name: String,
    data_type: DataType,
    default: Value,
}

pub enum Constraint {
    /*
    Unique(...),
    Check(...),
    Reference(...),
    Key(...),
    */
}

pub enum SchemaAction {
    Add {
        columns: Vec<Column>,
        constraint: Vec<Constraint>,
    },
    /*Remove {
        column: Vec<String>,
        constraint: Vec<Constraint>,
    },
    Rename {
        column: Vec<(String, String)>,
    },
    Raw {
        up: String,
        down: String,
    },
    Revert,
    */
}

pub struct SchemaModification {
    date: DateTime<Local>,
    commit: String,
    action: SchemaAction,

}

pub struct Table {
    name: String,
    description: String,
    schema: Vec<SchemaModification>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            name: "test_name".to_string(),
            description: "description".to_string(),
            schema: vec![],
        }
    }
}

/*
pub struct TableSimplified {
    name: String,
    description: String,

}
*/


pub enum ManagerQuery {
    All,
    Name(String),
}

pub enum OnDuplicate {
    Fail,
    Ignore,
    Update,
}
