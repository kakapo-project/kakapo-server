

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use data::api;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;
use model::manage;
use data::api::GetTablesResult;
use data::api::GetQueriesResult;
use data::api::GetScriptsResult;
use data::api::GetTableResult;
use data::api::GetQueryResult;
use data::api::GetScriptResult;
use data::api::CreateTableResult;
use data::api::CreateQueryResult;
use data::api::CreateScriptResult;
use model::table;
use data::api::GetTableDataResult;
use model::script;
use model::query;
use connection::py::PyRunner;
use data::api::InsertTableDataResult;
use data::api::UpdateTableDataResult;
use data::api::DeleteTableDataResult;
use data::api::RunQueryResult;
use data::api::RunScriptResult;

type State = PooledConnection<ConnectionManager<PgConnection>>;
type Error = data::api::Error;

pub type ActionResult = Result<(), ()>;

pub trait Action {
    type Result;
    fn call(&self, state: &State) -> Self::Result;
}

//get all actions
#[derive(Debug)]
pub struct GetAllTables {
    //pub detailed: bool, //TODO: is this needed?
    pub show_deleted: bool,
}

impl GetAllTables {
    pub fn new(detailed: bool, show_deleted: bool) -> Self {
        Self {
            show_deleted,
        }
    }
}

impl Action for GetAllTables {
    type Result = Result<GetTablesResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_tables(&state, false, self.show_deleted)
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GetAllQueries {
    pub show_deleted: bool,
}

impl Action for GetAllQueries {
    type Result = Result<GetQueriesResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_queries(&state, self.show_deleted)
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GetAllScripts {
    pub show_deleted: bool,
}

impl Action for GetAllScripts {
    type Result = Result<GetScriptsResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_scripts(&state) //TODO: why no show_deleted?
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

//get one actions
#[derive(Debug)]
pub struct GetTable {
    pub name: String,
    //pub detailed: bool, //TODO: is this needed?
}

impl GetTable {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

impl Action for GetTable {
    type Result = Result<GetTableResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_table(&state, self.name.to_owned(), false)
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GetQuery {
    pub name: String,
}

impl Action for GetQuery {
    type Result = Result<GetQueryResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_query(&state, self.name.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct GetScript {
    pub name: String,
}

impl Action for GetScript {
    type Result = Result<GetScriptResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::retrieve::get_script(&state, self.name.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// create actions
#[derive(Debug)]
pub struct CreateTable {
    pub reqdata: api::PostTable,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateTable {
    type Result = Result<CreateTableResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::create_table(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct CreateQuery {
    pub reqdata: api::PostQuery,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateQuery {
    type Result = Result<CreateQueryResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::create_query(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct CreateScript {
    pub reqdata: api::PostScript,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateScript {
    type Result = Result<CreateScriptResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::create_script(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// update actions
#[derive(Debug)]
pub struct UpdateTable {
    pub name: String,
    pub reqdata: api::PutTable,
}

impl Action for UpdateTable {
    type Result = Result<CreateTableResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::update_table(&state, self.name.to_owned(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct UpdateQuery {
    pub name: String,
    pub reqdata: api::PutQuery,
}

impl Action for UpdateQuery {
    type Result = Result<CreateQueryResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::update_query(&state, self.name.to_owned(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct UpdateScript {
    pub name: String,
    pub reqdata: api::PutScript,
}

impl Action for UpdateScript {
    type Result = Result<CreateScriptResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::update_script(&state, self.name.to_owned(), self.reqdata.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// delete actions
#[derive(Debug)]
pub struct DeleteTable {
    pub name: String,
}

impl Action for DeleteTable {
    type Result = Result<(), ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::delete_table(&state, self.name.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct DeleteQuery {
    pub name: String,
}

impl Action for DeleteQuery {
    type Result = Result<(), ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::delete_query(&state, self.name.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct DeleteScript {
    pub name: String,
}

impl Action for DeleteScript {
    type Result = Result<(), ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = manage::create::delete_script(&state, self.name.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// Table Actions
#[derive(Debug)]
pub struct QueryTableData {
    pub name: String,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub format: api::TableDataFormat,
}

impl Action for QueryTableData {
    type Result = Result<GetTableDataResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = table::get_table_data(&state, self.name.to_owned(), self.format.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}


#[derive(Debug)]
pub struct CreateTableData {
    pub name: String,
    pub data: api::TableData, //payload
    pub format: api::TableDataFormat,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateTableData {
    type Result = Result<InsertTableDataResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = table::insert_table_data(
            &state,
            self.name.to_owned(), self.data.to_owned(), self.format.to_owned(), api::CreationMethod::default())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct UpdateTableData {
    pub name: String,
    pub key: String,
    pub data: api::RowData, //payload
    pub format: api::TableDataFormat,
}

impl Action for UpdateTableData {
    type Result = Result<UpdateTableDataResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = table::update_table_data(
            &state,
            self.name.to_owned(), self.key.to_owned(), self.data.to_owned(), self.format.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

#[derive(Debug)]
pub struct DeleteTableData {
    pub name: String,
    pub key: String,
}

impl Action for DeleteTableData {
    type Result = Result<DeleteTableDataResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = table::delete_table_data(&state, self.name.to_owned(), self.key.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// Query Action
#[derive(Debug)]
pub struct RunQuery {
    pub name: String,
    pub params: api::QueryParams,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub format: api::TableDataFormat,
}

impl Action for RunQuery {
    type Result = Result<RunQueryResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = query::run_query(
            &state,
            self.name.to_owned(), self.format.to_owned(), self.params.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

// Query Action
#[derive(Debug)]
pub struct RunScript {
    pub name: String,
    pub params: api::ScriptParam,
    pub py_runner: PyRunner,
}

impl Action for RunScript {
    type Result = Result<RunScriptResult, ()>;
    fn call(&self, state: &State) -> Self::Result {
        let result = script::run_script(
            &state,
            self.py_runner.to_owned(), self.name.to_owned(), self.params.to_owned())
            .or_else(|err| Err(()))?;
        Ok(result)
    }
}

//Other utitlies
#[derive(Debug)]
pub struct Nothing;


impl Action for Nothing {
    type Result = Result<(), ()>;
    fn call(&self, state: &State) -> Self::Result {
        Ok(())
    }
}