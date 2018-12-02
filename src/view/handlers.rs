
use actix::prelude::*;
use actix_web::error::ResponseError;
use actix_web::ws;

use serde_json;

use model::api;
use model::connection::{executor::DatabaseExecutor, py::PyRunner};

use model::manage;
use model::{table, query, script};
use actix_broker::BrokerMsg;
use view::state::AppState;
use view::session::TableSession;


// Create Table
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct CreateTable {
    pub reqdata: api::PostTable,
    pub method: api::CreationMethod,
}

impl Handler<CreateTable> for DatabaseExecutor {
    type Result = <CreateTable as Message>::Result;

    fn handle(&mut self, msg: CreateTable, _: &mut Self::Context) -> Self::Result {
        let result = manage::create_table(&self.get_connection(), msg.method, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct CreateQuery {
    pub reqdata: api::PostQuery,
    pub method: api::CreationMethod,
}

impl Handler<CreateQuery> for DatabaseExecutor {
    type Result = <CreateQuery as Message>::Result;

    fn handle(&mut self, msg: CreateQuery, _: &mut Self::Context) -> Self::Result {
        let result = manage::create_query(&self.get_connection(), msg.method, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct CreateScript {
    pub reqdata: api::PostScript,
    pub method: api::CreationMethod,
}

impl Handler<CreateScript> for DatabaseExecutor {
    type Result = <CreateScript as Message>::Result;

    fn handle(&mut self, msg: CreateScript, _: &mut Self::Context) -> Self::Result {
        let result = manage::create_script(&self.get_connection(), msg.method, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}


// Update Table
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct UpdateTable {
    pub name: String,
    pub reqdata: api::PutTable,
}

impl Handler<UpdateTable> for DatabaseExecutor {
    type Result = <UpdateTable as Message>::Result;

    fn handle(&mut self, msg: UpdateTable, _: &mut Self::Context) -> Self::Result {
        let result = manage::update_table(&self.get_connection(), msg.name, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct UpdateQuery {
    pub name: String,
    pub reqdata: api::PutQuery,
}

impl Handler<UpdateQuery> for DatabaseExecutor {
    type Result = <UpdateQuery as Message>::Result;

    fn handle(&mut self, msg: UpdateQuery, _: &mut Self::Context) -> Self::Result {
        let result = manage::update_query(&self.get_connection(), msg.name, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct UpdateScript {
    pub name: String,
    pub reqdata: api::PutScript,
}

impl Handler<UpdateScript> for DatabaseExecutor {
    type Result = <UpdateScript as Message>::Result;

    fn handle(&mut self, msg: UpdateScript, _: &mut Self::Context) -> Self::Result {
        let result = manage::update_script(&self.get_connection(), msg.name, msg.reqdata)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}


// Get All Table
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTables {
    pub detailed: bool,
    pub show_deleted: bool,
}

impl Handler<GetTables> for DatabaseExecutor {
    type Result = <GetTables as Message>::Result;

    fn handle(&mut self, msg: GetTables, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_tables(&self.get_connection(), msg.detailed, msg.show_deleted)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetQueries {
    pub show_deleted: bool,
}

impl Handler<GetQueries> for DatabaseExecutor {
    type Result = <GetQueries as Message>::Result;

    fn handle(&mut self, msg: GetQueries, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_queries(&self.get_connection(), msg.show_deleted)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}


#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetScripts;

impl Handler<GetScripts> for DatabaseExecutor {
    type Result = <GetScripts as Message>::Result;

    fn handle(&mut self, msg: GetScripts, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_scripts(&self.get_connection())?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

// Get Table
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTable {
    pub name: String,
    pub detailed: bool,
}

impl Handler<GetTable> for DatabaseExecutor {
    type Result = <GetTable as Message>::Result;

    fn handle(&mut self, msg: GetTable, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_table(&self.get_connection(), msg.name, msg.detailed)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetQuery {
    pub name: String,
}

impl Handler<GetQuery> for DatabaseExecutor {
    type Result = <GetQuery as Message>::Result;

    fn handle(&mut self, msg: GetQuery, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_query(&self.get_connection(), msg.name)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetScript {
    pub name: String,
}

impl Handler<GetScript> for DatabaseExecutor {
    type Result = <GetScript as Message>::Result;

    fn handle(&mut self, msg: GetScript, _: &mut Self::Context) -> Self::Result {
        let result = manage::get_script(&self.get_connection(), msg.name)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}


//

// Get Table Data
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTableData {
    pub name: String,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub format: api::TableDataFormat,
}

impl Handler<GetTableData> for DatabaseExecutor {
    type Result = <GetTableData as Message>::Result;

    fn handle(&mut self, msg: GetTableData, _: &mut Self::Context) -> Self::Result {
        let result = table::get_table_data(&self.get_connection(), msg.name, msg.format)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

// Insert Table Data
#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct InsertTableData {
    pub name: String,
    pub data: api::TableData, //payload
    pub format: api::TableDataFormat,
}

impl Handler<InsertTableData> for DatabaseExecutor {
    type Result = <InsertTableData as Message>::Result;

    fn handle(&mut self, msg: InsertTableData, _: &mut Self::Context) -> Self::Result {
        let result = table::insert_table_data(&self.get_connection(), msg.name, msg.data, msg.format)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct RunQuery {
    pub name: String,
    pub params: api::QueryParams,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub format: api::TableDataFormat,
}

impl Handler<RunQuery> for DatabaseExecutor {
    type Result = <RunQuery as Message>::Result;

    fn handle(&mut self, msg: RunQuery, _: &mut Self::Context) -> Self::Result {
        let result = query::run_query(&self.get_connection(), msg.name, msg.format, msg.params)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}

#[derive(Clone, Message)]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct RunScript {
    pub name: String,
    pub params: api::ScriptParam,
    pub py_runner: PyRunner,
}

impl Handler<RunScript> for DatabaseExecutor {
    type Result = <RunScript as Message>::Result;

    fn handle(&mut self, msg: RunScript, ctx: &mut Self::Context) -> Self::Result {
        let result = script::run_script(&self.get_connection(), msg.py_runner, msg.name, msg.params)?;
        Ok(serde_json::to_value(&result).or_else(|err| Err(api::Error::SerializationError))?)
    }
}