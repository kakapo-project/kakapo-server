

use actix::prelude::*;
use diesel;
use diesel::result::Error;
use diesel::{
    prelude::*,
    insert_into,
    delete,
    update,
};
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use chrono::NaiveDateTime;
use serde_json;

use std::error;
use std::collections::HashMap;

use failure::Fail;

use super::api;
use super::data;
use super::err::StateError;
use super::schema::{entity, table_schema, table_schema_history};



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
