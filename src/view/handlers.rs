
use actix::prelude::*;
use model::api;
use model::connection::DatabaseExecutor;

use actix_web::error::ResponseError;

use model::actions;

pub struct CreateTable {
    pub reqdata: api::PostTable,
}

impl Message for CreateTable {
    type Result = Result<(), api::Error>;
}

impl Handler<CreateTable> for DatabaseExecutor {
    type Result = Result<(), api::Error>;

    fn handle(&mut self, msg: CreateTable, _: &mut Self::Context) -> Self::Result {
        actions::create_table(self.get_connection(), msg.reqdata)
    }
}


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
        actions::get_tables(self.get_connection(), msg.detailed, msg.show_deleted)
    }
}