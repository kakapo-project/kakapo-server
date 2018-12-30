

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;
use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::DBError;

use model::schema;


#[derive(Debug, Serialize)]
pub struct GetAllTablesResult(pub Vec<data::Table>);

#[derive(Debug, Serialize)]
pub struct GetTableResult(pub data::Table);

#[derive(Debug, Serialize)]
pub struct GetAllQueriesResult(pub Vec<data::Query>);

#[derive(Debug, Serialize)]
pub struct GetQueryResult(pub data::Query);

#[derive(Debug, Serialize)]
pub struct GetAllScriptsResult(pub Vec<data::Script>);

#[derive(Debug, Serialize)]
pub struct GetScriptResult(pub data::Script);

#[derive(Debug, Serialize)]
pub struct CreateEntityResult<T>(pub T);

#[derive(Debug, Serialize)]
pub struct GetTableDataResult(pub data::TableData);

#[derive(Debug, Serialize)]
pub struct InsertTableDataResult(pub data::TableData);

#[derive(Debug, Serialize)]
pub struct UpdateTableDataResult(pub data::RowData);

#[derive(Debug, Serialize)]
pub struct DeleteTableDataResult(pub data::RowData);

#[derive(Debug, Serialize)]
pub struct RunQueryResult(pub data::TableData);

#[derive(Debug, Serialize)]
pub struct RunScriptResult(pub serde_json::Value);

pub trait NamedActionResult {
    const ACTION_NAME: &'static str = "No Action";
}

impl NamedActionResult for GetAllTablesResult {
    const ACTION_NAME: &'static str = "GetAllTables";
}

impl NamedActionResult for GetAllQueriesResult {
    const ACTION_NAME: &'static str = "GetAllQueries";
}

impl NamedActionResult for GetAllScriptsResult {
    const ACTION_NAME: &'static str = "GetAllScripts";
}

impl NamedActionResult for GetTableResult {
    const ACTION_NAME: &'static str = "GetTable";
}

impl NamedActionResult for GetQueryResult {
    const ACTION_NAME: &'static str = "GetQuery";
}

impl NamedActionResult for GetScriptResult {
    const ACTION_NAME: &'static str = "GetScript";
}


impl NamedActionResult for CreateEntityResult<data::Table> {
    const ACTION_NAME: &'static str = "CreateTable";
}

impl NamedActionResult for CreateEntityResult<data::Query> {
    const ACTION_NAME: &'static str = "CreateQuery";
}

impl NamedActionResult for CreateEntityResult<data::Script> {
    const ACTION_NAME: &'static str = "CreateScript";
}

impl NamedActionResult for () {
    const ACTION_NAME: &'static str = "None";
}

impl NamedActionResult for GetTableDataResult {
    const ACTION_NAME: &'static str = "GetTableData";
}


impl NamedActionResult for InsertTableDataResult {
    const ACTION_NAME: &'static str = "InsertTableData";
}


impl NamedActionResult for UpdateTableDataResult {
    const ACTION_NAME: &'static str = "UpdateTableData";
}


impl NamedActionResult for DeleteTableDataResult {
    const ACTION_NAME: &'static str = "DeleteTableData";
}

impl NamedActionResult for RunQueryResult {
    const ACTION_NAME: &'static str = "RunQuery";
}

impl NamedActionResult for RunScriptResult {
    const ACTION_NAME: &'static str = "RunScript";
}