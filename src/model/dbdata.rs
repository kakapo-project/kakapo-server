

use chrono::NaiveDateTime;
use serde_json;

use super::schema::{entity, table_schema, table_schema_history, query, query_history, script, script_history, user_account};



#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewEntity {
    pub scope_id: i64,
    pub created_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema"]
pub struct NewTableSchema {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema_history"]
pub struct NewTableSchemaHistory {
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "query"]
pub struct NewDataQuery {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "query_history"]
pub struct NewDataQueryHistory {
    pub query_id: i64,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script"]
pub struct NewDataScript {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script_history"]
pub struct NewDataScriptHistory {
    pub script_id: i64,
    pub description: String,
    pub script_language: String,
    pub script_text: String,
    pub script_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_by: i64,
}

#[derive(Debug, Queryable)]
pub struct Entity {
    pub entity_id: i64,
    pub scope_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
}

#[derive(Debug, Queryable)]
pub struct TableSchema {
    pub table_schema_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct TableSchemaHistory {
    pub table_schema_history_id: i64,
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

#[derive(Debug, Queryable)]
pub struct DataQuery {
    pub query_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct DataQueryHistory {
    pub query_history_id: i64,
    pub query_id: i64,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

#[derive(Debug, Queryable)]
pub struct DataScript {
    pub script_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct DataScriptHistory {
    pub script_history_id: i64,
    pub script_id: i64,
    pub description: String,
    pub script_language: String,
    pub script_text: String,
    pub script_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "user_account"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub user_account_id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
}