

use chrono::NaiveDateTime;
use serde_json;

use super::schema::{entity, table_schema, table_schema_history, query, query_history, script, script_history, user_account};
use model::schema;
use diesel::expression_methods::ExpressionMethods;


#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewRawEntity {
    pub scope_id: i64,
    pub created_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema"]
pub struct NewRawTable {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema_history"]
pub struct NewRawTableHistory {
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "query"]
pub struct NewRawQuery {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "query_history"]
pub struct NewRawQueryHistory {
    pub query_id: i64,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script"]
pub struct NewRawScript {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script_history"]
pub struct NewRawScriptHistory {
    pub script_id: i64,
    pub description: String,
    pub script_language: String,
    pub script_text: String,
    pub script_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_by: i64,
}

// Queryables
pub trait RawEntityTypes {
    type Meta;
    type Data;
    type DataHistory;
}

#[derive(Identifiable, Debug, Queryable)]
#[primary_key(entity_id)]
#[table_name = "entity"]
pub struct RawEntity {
    pub entity_id: i64,
    pub scope_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
}

#[derive(Identifiable, Debug, Queryable)]
#[primary_key(table_schema_id)]
#[table_name = "table_schema"]
pub struct RawTable {
    pub table_schema_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Identifiable, Associations, Debug, Queryable)]
#[primary_key(table_schema_history_id)]
#[table_name = "table_schema_history"]
#[belongs_to(RawTable, foreign_key = "table_schema_id")]
pub struct RawTableHistory {
    pub table_schema_history_id: i64,
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

pub struct RawTableEntityTypes;
impl RawEntityTypes for RawTableEntityTypes {
    type Meta = RawEntity;
    type Data = RawTable;
    type DataHistory = RawTableHistory;
}


#[derive(Identifiable, Debug, Queryable, Clone)]
#[primary_key(query_id)]
#[table_name = "query"]
pub struct RawQuery {
    pub query_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Identifiable, Associations, Debug, Queryable, Clone)]
#[primary_key(query_history_id)]
#[table_name = "query_history"]
#[belongs_to(RawQuery, foreign_key = "query_id")]
pub struct RawQueryHistory {
    pub query_history_id: i64,
    pub query_id: i64,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

pub struct RawQueryEntityTypes;
impl RawEntityTypes for RawQueryEntityTypes {
    type Meta = RawEntity;
    type Data = RawQuery;
    type DataHistory = RawQueryHistory;
}

#[derive(Identifiable, Debug, Queryable)]
#[primary_key(script_id)]
#[table_name = "script"]
pub struct RawScript {
    pub script_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Identifiable, Associations, Debug, Queryable)]
#[primary_key(script_history_id)]
#[table_name = "script_history"]
#[belongs_to(RawScript, foreign_key = "script_id")]
pub struct RawScriptHistory {
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

pub struct RawScriptEntityTypes;
impl RawEntityTypes for RawScriptEntityTypes {
    type Meta = RawEntity;
    type Data = RawScript;
    type DataHistory = RawScriptHistory;
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