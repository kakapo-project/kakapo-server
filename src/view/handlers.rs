
use actix::prelude::*;
use actix_web::error::ResponseError;
use actix_web::ws;

use serde_json;

use model::api;
use model::connection::DatabaseExecutor;

use model::manage;
use model::table;


// Create or Update Table
pub struct CreateTable {
    pub reqdata: api::PostTable,
}

impl Message for CreateTable {
    type Result = Result<api::CreateTableResult, api::Error>;
}

impl Handler<CreateTable> for DatabaseExecutor {
    type Result = Result<api::CreateTableResult, api::Error>;

    fn handle(&mut self, msg: CreateTable, _: &mut Self::Context) -> Self::Result {
        manage::create_table(self.get_connection(), msg.reqdata)
    }
}

// Get All Table
pub struct GetTables {
    pub detailed: bool,
    pub show_deleted: bool,
}

impl Message for GetTables {
    type Result = Result<api::GetTablesResult, api::Error>;
}

impl Handler<GetTables> for DatabaseExecutor {
    type Result = Result<api::GetTablesResult, api::Error>;

    fn handle(&mut self, msg: GetTables, _: &mut Self::Context) -> Self::Result {
        manage::get_tables(self.get_connection(), msg.detailed, msg.show_deleted)
    }
}

// Get Table
pub struct GetTable {
    pub name: String,
    pub detailed: bool,
}

impl Message for GetTable {
    type Result = Result<api::GetTableResult, api::Error>;
}

impl Handler<GetTable> for DatabaseExecutor {
    type Result = Result<api::GetTableResult, api::Error>;

    fn handle(&mut self, msg: GetTable, _: &mut Self::Context) -> Self::Result {
        manage::get_table(self.get_connection(), msg.name, msg.detailed)
    }
}


//

// Get Table Data
pub struct GetTableData {
    pub name: String,
}

impl Message for GetTableData {
    type Result = Result<api::GetTableDataResult, api::Error>;
}

impl Handler<GetTableData> for DatabaseExecutor {
    type Result = Result<api::GetTableDataResult, api::Error>;

    fn handle(&mut self, msg: GetTableData, _: &mut Self::Context) -> Self::Result {
        table::get_table_data(self.get_connection(), msg.name)
    }
}

// Insert Table Data
pub struct InsertTableData {
    pub name: String,
    pub data: api::TableData,
}

impl Message for InsertTableData {
    type Result = Result<api::InsertTableDataResult, api::Error>;
}

impl Handler<InsertTableData> for DatabaseExecutor {
    type Result = Result<api::InsertTableDataResult, api::Error>;

    fn handle(&mut self, msg: InsertTableData, _: &mut Self::Context) -> Self::Result {
        table::insert_table_data(self.get_connection(), msg.name, msg.data)
    }
}