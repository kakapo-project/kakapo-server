
use chrono::prelude::*;
use chrono::DateTime;

#[derive(Debug, Deserialize, Serialize)]
pub enum DataType {
    String,
    Integer,
    Json,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
    name: String,
    data_type: DataType,
    default: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Constraint {
    /*
    Unique(...),
    Check(...),
    Reference(...),
    Key(...),
    */
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SchemaAction {
    Create {
        columns: Vec<Column>,
        constraint: Vec<Constraint>,
    },
    Nop,
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
impl Default for SchemaAction {
    fn default() -> Self {
        SchemaAction::Nop
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaModification {
    pub date: NaiveDateTime,
    pub action: SchemaAction,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DetailedTable {
    pub name: String,
    pub description: String,
    pub schema: Vec<SchemaModification>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
    pub name: String,
    pub description: String,
    pub schema: Vec<SchemaModification>,
}
