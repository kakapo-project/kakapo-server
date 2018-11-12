
use actix::prelude::*;
use actix_web::error::ResponseError;
use actix_web::ws;

use serde_json;

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
    pub conn: Addr<DatabaseExecutor>,
    pub table_name: String,
    pub session_id: usize,
}

impl TableSession {
    pub fn new(conn: Addr<DatabaseExecutor>, table_name: String) -> Self {
        Self {
            conn: conn,
            table_name: table_name,
            session_id: 0,
        }
    }


    fn handle_action(&self, table_session_request: api::TableSessionRequest) -> () {
        match table_session_request {
            api::TableSessionRequest::GetTable => {

            },
            api::TableSessionRequest::GetAllTableData { begin, chunk_size } => {

            },
            api::TableSessionRequest::GetTableData { begin, end, chunk_size } => {

            },
            api::TableSessionRequest::Create(row) => {

            },
            api::TableSessionRequest::Update(row) => {

            },
            api::TableSessionRequest::Delete(index) => {

            },
        };
    }

}

impl StreamHandler<ws::Message, ws::ProtocolError> for TableSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                serde_json::from_str(&text)
                    .and_then(|table_session_request: api::TableSessionRequest| {
                        self.handle_action(table_session_request);
                        Ok(())
                    })
                    .or_else::<serde_json::error::Error, _>(|err| {
                        println!("Error occured while parsing websocket request: {:?}", err);
                        ctx.stop();
                        //TODO: send error message
                        Ok(())
                    });
            },
            ws::Message::Close(_) => {
                ctx.stop();
            },
            _ => {}
        }
    }
}
