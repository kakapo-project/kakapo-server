
use actix::prelude::*;
use actix_web::error::ResponseError;
use actix_web::ws;

use model::api;
use model::connection::DatabaseExecutor;

use model::manage;
use model::table;



// Create Table
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

// Get Table
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


// Websockets
pub struct TableSession {
    pub table_name: String,
    pub session_id: usize,
}

impl TableSession {
    pub fn new() -> Self {
        Self {
            table_name: "tmp".to_string(),
            session_id: 0,
        }
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for TableSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        unimplemented!();
    }
}
