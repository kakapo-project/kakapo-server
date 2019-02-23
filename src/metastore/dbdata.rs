
use std::fmt::Debug;

use serde::Serialize;
use chrono::NaiveDateTime;
use serde_json;

use metastore::schema::entity;
use metastore::schema::table_schema;
use metastore::schema::query;
use metastore::schema::script;
use metastore::schema::view;
use metastore::schema::user;
use metastore::schema::permission;
use metastore::schema::role;
use metastore::schema::invitation;
use metastore::schema::session;
use metastore::schema::channel;
use metastore::schema::user_channel;
use metastore::schema::message;

use data::permissions::Permission;
use data::Named;

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

impl Named for RawTable {
    fn my_name(&self) -> &str {
        &self.name
    }
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

impl Named for NewRawTable {
    fn my_name(&self) -> &str {
        &self.name
    }
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

impl Named for RawQuery {
    fn my_name(&self) -> &str {
        &self.name
    }
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

impl Named for NewRawQuery {
    fn my_name(&self) -> &str {
        &self.name
    }
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

impl Named for RawScript {
    fn my_name(&self) -> &str {
        &self.name
    }
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

impl Named for NewRawScript {
    fn my_name(&self) -> &str {
        &self.name
    }
}

#[derive(Identifiable, Associations, Debug, Queryable, QueryableByName, Clone)]
#[primary_key(view_id)]
#[table_name = "view"]
#[belongs_to(RawEntity, foreign_key = "entity_id")]
pub struct RawView {
    pub view_id: i64,
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub view_state: serde_json::Value,
    pub view_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

impl Named for RawView {
    fn my_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "view"]
pub struct NewRawView {
    pub entity_id: i64,
    pub name: String,
    pub description: String,
    pub view_state: serde_json::Value,
    pub view_info: serde_json::Value,
    pub is_deleted: bool,
    pub modified_by: i64,
}

impl Named for NewRawView {
    fn my_name(&self) -> &str {
        &self.name
    }
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
#[table_name = "role"]
pub struct NewRawRole {
    pub name: String,
    pub description: String,
}

impl NewRawRole {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[primary_key(role_id)]
#[table_name = "role"]
pub struct RawRole {
    pub role_id: i64,
    pub name: String,
    pub description: String,
    pub role_info: serde_json::Value,
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

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "session"]
pub struct NewRawSessionToken {
    pub token: String,
    pub user_id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[primary_key(session_id)]
#[table_name = "session"]
pub struct RawSessionToken {
    pub session_id: i64,
    pub token: String,
    pub user_id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "permission"]
pub struct NewRawPermission {
    pub data: serde_json::Value,
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

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "channel"]
pub struct NewRawChannel {
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "channel"]
pub struct RawChannel {
    pub channel_id: i64,
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "user_channel"]
pub struct NewRawUserChannel {
    pub user_id: i64,
    pub channel_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "user_channel"]
pub struct RawUserChannel {
    pub user_channel_id: i64,
    pub user_id: i64,
    pub channel_id: i64,
    pub subscribed_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "message"]
pub struct NewRawMessage {
    pub channel_id: i64,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name = "message"]
pub struct RawMessage {
    pub message_id: i64,
    pub channel_id: i64,
    pub data: serde_json::Value,
    pub sent_at: chrono::NaiveDateTime,
}