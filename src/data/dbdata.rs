

use chrono::NaiveDateTime;
use serde_json;

use data::schema::{entity, table_schema, query, script, user, permission, invitation};
use data;
use std::fmt::Debug;
use serde::Serialize;
use model::auth::permissions::Permission;

#[derive(Identifiable, Debug, Queryable, Clone)]
#[primary_key(entity_id)]
#[table_name = "entity"]
pub struct RawEntity {
    pub entity_id: i64,
    pub scope_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewRawEntity {
    pub scope_id: i64,
    pub created_by: i64,
}

#[derive(Identifiable, Associations, Debug, Queryable, QueryableByName, Clone)]
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
    pub is_deleted: bool,
    pub modified_by: i64,
}

#[derive(Identifiable, Associations, Debug, Queryable, QueryableByName, Clone)]
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
    pub is_deleted: bool,
    pub modified_by: i64,
}

#[derive(Identifiable, Associations, Debug, Queryable, QueryableByName, Clone)]
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
    pub is_deleted: bool,
    pub modified_by: i64,
}


#[derive(Debug, Deserialize, Insertable)]
#[table_name = "user"]
pub struct NewRawUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[primary_key(user_id)]
#[table_name = "user"]
pub struct RawUser {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub user_info: serde_json::Value,
    pub joined_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "invitation"]
pub struct NewRawInvitation {
    pub email: String,
    pub token: String,
}

impl NewRawInvitation {
    pub fn new(email: String, token: String) -> Self {
        Self { email, token }
    }
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[primary_key(invitation_id)]
#[table_name = "invitation"]
pub struct RawInvitation {
    pub invitation_id: i64,
    pub email: String,
    pub token: String,
    pub token_info: serde_json::Value,
    pub sent_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}


#[derive(Clone, Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "permission"]
pub struct RawPermission {
    pub permission_id: i64,
    pub data: serde_json::Value,
}

impl RawPermission {
    pub fn as_permission(self) -> Option<Permission> {
        match serde_json::from_value(self.data) {
            Ok(res) => Some(res),
            Err(err) => None,
        }
    }
}