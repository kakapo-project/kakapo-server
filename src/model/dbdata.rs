

use chrono::NaiveDateTime;
use serde_json;

use super::schema::{entity, table_schema, query, script, user_account};
use model::schema;
use diesel::expression_methods::ExpressionMethods;
use data;


// Queryables
pub trait RawEntityTypes {
    type Data;
    type NewData;
}

#[derive(Identifiable, Debug, Queryable, Clone)]
#[primary_key(entity_id)]
#[table_name = "entity"]
pub struct RawEntity {
    pub entity_id: i64,
    pub scope_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
}

#[derive(Identifiable, Associations, Debug, Queryable, Clone)]
#[primary_key(table_schema_id)]
#[table_name = "table_schema"]
#[belongs_to(RawEntity, foreign_key = "entity_id")]
pub struct RawTable {
    pub table_schema_id: i64,
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub table_data: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}


#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema"]
pub struct NewRawTable {
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub table_data: serde_json::Value,
    pub modified_by: i64,
}

impl RawEntityTypes for data::Table {
    type Data = RawTable;
    type NewData = NewRawTable;
}


#[derive(Identifiable, Associations, Debug, Queryable, Clone)]
#[primary_key(query_id)]
#[table_name = "query"]
#[belongs_to(RawEntity, foreign_key = "entity_id")]
pub struct RawQuery {
    pub query_id: i64,
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "query"]
pub struct NewRawQuery {
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub statement: String,
    pub query_info: serde_json::Value,
    pub modified_by: i64,
}

impl RawEntityTypes for data::Query {
    type Data = RawQuery;
    type NewData = NewRawQuery;
}

#[derive(Identifiable, Associations, Debug, Queryable, Clone)]
#[primary_key(script_id)]
#[table_name = "script"]
#[belongs_to(RawEntity, foreign_key = "entity_id")]
pub struct RawScript {
    pub script_id: i64,
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub script_language: String,
    pub script_text: String,
    pub script_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script"]
pub struct NewRawScript {
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub script_language: String,
    pub script_text: String,
    pub script_info: serde_json::Value,
    pub modified_by: i64,
}

impl RawEntityTypes for data::Script {
    type Data = RawScript;
    type NewData = NewRawScript;
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