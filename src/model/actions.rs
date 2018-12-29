

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use data::api;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;
use data::api::GetTablesResult;
use data::api::GetQueriesResult;
use data::api::GetScriptsResult;
use data::api::GetTableResult;
use data::api::GetQueryResult;
use data::api::GetScriptResult;
use data::api::CreateTableResult;
use data::api::CreateQueryResult;
use data::api::CreateScriptResult;
use data::api::GetTableDataResult;
use connection::py::PyRunner;
use data::api::InsertTableDataResult;
use data::api::UpdateTableDataResult;
use data::api::DeleteTableDataResult;
use data::api::RunQueryResult;
use data::api::RunScriptResult;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::error::DBError;

use super::schema;

type State = PooledConnection<ConnectionManager<PgConnection>>; //TODO: should include user data
pub type Error = ();


pub type ActionResult = Result<(), ()>;

pub trait Action: Send
    where
        Self::Ret: Send + serde::Serialize
{
    type Ret;
    fn call(&self, state: &State/*, session: Session*/) -> Result<Self::Ret, Error>;
}

///decorator for permission
pub struct WithPermissionRequired<A: Action> {
    action: A,
    //permission: ...
}

impl<A: Action> Action for WithPermissionRequired<A> {
    type Ret = A::Ret; //TODO: 403 error
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        self.action.call(state)
    }
}

impl<A: Action> WithPermissionRequired<A> {
    pub fn new(action: A/*, permission: Permission*/) -> Self {
        Self {
            action,
            //permission,
        }
    }
}

///decorator for transactions
pub struct WithTransaction<A: Action> {
    action: A,
    //permission: ...
}

impl<A: Action> Action for WithTransaction<A> {
    type Ret = A::Ret;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //TODO: transactions
        self.action.call(state)
    }
}

impl<A: Action> WithTransaction<A> {
    pub fn new(action: A) -> Self {
        Self {
            action,
        }
    }
}

///get all tables
#[derive(Debug)]
pub struct GetAllTables {
    //pub detailed: bool, //TODO: is this needed?
    pub show_deleted: bool,
}

impl Action for GetAllTables {
    type Ret = GetTablesResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        let tables: Result<Vec<data::Table>, DBError> = entity::Retriever::get_all(&state);
        Err(())
    }
}

impl GetAllTables {
    pub fn new(show_deleted: bool) -> Self {
        Self {
            show_deleted,
        }
    }
}

///get all queries
#[derive(Debug)]
pub struct GetAllQueries {
    pub show_deleted: bool,
}

impl Action for GetAllQueries {
    type Ret = GetQueriesResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let queries: Vec<data::Query> = entity::query::Retriever::get_all(&state);
        Err(())
    }
}

impl GetAllQueries {
    pub fn new(show_deleted: bool) -> Self {
        Self {
            show_deleted,
        }
    }
}

///get all scripts
#[derive(Debug)]
pub struct GetAllScripts {
    pub show_deleted: bool,
}

impl Action for GetAllScripts {
    type Ret = GetScriptsResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let script: Vec<data::Script> = entity::script::Retriever::get_all(&state);
        Err(())
    }
}

impl GetAllScripts {
    pub fn new(show_deleted: bool) -> Self {
        Self {
            show_deleted,
        }
    }
}

///get one table
#[derive(Debug)]
pub struct GetTable {
    pub name: String,
    //pub detailed: bool, //TODO: is this needed?
}

impl Action for GetTable {
    type Ret = GetTableResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        println!("name: {:?}", &self.name);
        let table: Result<Option<data::Table>, DBError> = entity::Retriever::get_one(&state, &self.name);
        Err(())
    }
}

impl GetTable {
    pub fn new(name: String) -> impl Action<Ret = GetTableResult> { //weird syntax but ok
        let action = Self {
            name,
        };
        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithPermissionRequired::new(action_with_transaction /*, ... */);

        action_with_permission
    }
}

///get one query
#[derive(Debug)]
pub struct GetQuery {
    pub name: String,
}

impl GetQuery {
    pub fn new(name: String) -> impl Action<Ret = GetQueryResult> { //weird syntax but ok
        let action = Self {
            name,
        };
        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithPermissionRequired::new(action_with_transaction /*, ... */);

        action_with_permission
    }
}

impl Action for GetQuery {
    type Ret = GetQueryResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let query: data::Query = entity::query::Retriever::get_one(&state, &self.name).unwrap();
        Err(())
    }
}

///get one script
#[derive(Debug)]
pub struct GetScript {
    pub name: String,
}

impl Action for GetScript {
    type Ret = GetScriptResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let script: data::Script = entity::script::Retriever::get_one(&state, &self.name).unwrap();
        Err(())
    }
}

impl GetScript {
    pub fn new(name: String) -> impl Action<Ret = GetScriptResult> { //weird syntax but ok
        let action = Self {
            name,
        };
        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithPermissionRequired::new(action_with_transaction /*, ... */);

        action_with_permission
    }
}

///create one table
#[derive(Debug)]
pub struct CreateTable {
    pub reqdata: api::PostTable,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateTable {
    type Ret = CreateTableResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::create_table(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///create one query
#[derive(Debug)]
pub struct CreateQuery {
    pub reqdata: api::PostQuery,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateQuery {
    type Ret = CreateQueryResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::create_query(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///create one script
#[derive(Debug)]
pub struct CreateScript {
    pub reqdata: api::PostScript,
    //pub on_duplicate: api::OnDuplicate,
}

impl Action for CreateScript {
    type Ret = CreateScriptResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::create_script(&state, api::OnDuplicate::default(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///update table
#[derive(Debug)]
pub struct UpdateTable {
    pub name: String,
    pub reqdata: api::PutTable,
}

impl Action for UpdateTable {
    type Ret = CreateTableResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::update_table(&state, self.name.to_owned(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///update query
#[derive(Debug)]
pub struct UpdateQuery {
    pub name: String,
    pub reqdata: api::PutQuery,
}

impl Action for UpdateQuery {
    type Ret = CreateQueryResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::update_query(&state, self.name.to_owned(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///update script
#[derive(Debug)]
pub struct UpdateScript {
    pub name: String,
    pub reqdata: api::PutScript,
}

impl Action for UpdateScript {
    type Ret = CreateScriptResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::update_script(&state, self.name.to_owned(), self.reqdata.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///delete table
#[derive(Debug)]
pub struct DeleteTable {
    pub name: String,
}

impl Action for DeleteTable {
    type Ret = ();
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::delete_table(&state, self.name.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///delete query
#[derive(Debug)]
pub struct DeleteQuery {
    pub name: String,
}

impl Action for DeleteQuery {
    type Ret = ();
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::delete_query(&state, self.name.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

///delete script
#[derive(Debug)]
pub struct DeleteScript {
    pub name: String,
}

impl Action for DeleteScript {
    type Ret = ();
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = manage::create::delete_script(&state, self.name.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
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
    type Ret = GetTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::get_table_data(&state, self.name.to_owned(), self.format.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
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
    type Ret = InsertTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::insert_table_data(
        //    &state,
        //    self.name.to_owned(), self.data.to_owned(), self.format.to_owned(), api::CreationMethod::default())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
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
    type Ret = UpdateTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::update_table_data(
        //    &state,
        //    self.name.to_owned(), self.key.to_owned(), self.data.to_owned(), self.format.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

#[derive(Debug)]
pub struct DeleteTableData {
    pub name: String,
    pub key: String,
}

impl Action for DeleteTableData {
    type Ret = DeleteTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::delete_table_data(&state, self.name.to_owned(), self.key.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
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
    type Ret = RunQueryResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = query::run_query(
        //    &state,
        //    self.name.to_owned(), self.format.to_owned(), self.params.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
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
    type Ret = RunScriptResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = script::run_script(
        //    &state,
        //    self.py_runner.to_owned(), self.name.to_owned(), self.params.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(())
    }
}

//Other utitlies
#[derive(Debug)]
pub struct Nothing;


impl Action for Nothing {
    type Ret = ();
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        Ok(())
    }
}