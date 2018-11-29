
use actix::prelude::*;
use diesel;
use diesel::result::Error;
use diesel::{
    prelude::*,
    sql_types,
    insert_into,
    delete,
    update,
};
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::sql_types::*;

use failure::Fail;
use std::io::Write;
use std::io;
use std::collections::BTreeMap;
use std;
use std::ops::Deref;

use super::data;
use super::data::DataType;
use super::api;

use super::dbdata::*;
use super::schema::{entity, query, query_history};
use super::manage::{get_single_table, unroll_table};
use super::database;

pub fn run_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    script_name: String,
    params: serde_json::Value,
) -> Result<api::RunScriptResult, api::Error> {
    Ok(api::RunScriptResult(json!(null)))
}