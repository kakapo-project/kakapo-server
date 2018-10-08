
use actix::prelude::*;
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse, Json,
};
use futures::future::{Future, result};
use super::connection::DatabaseExecutor;

use super::data::*;
use super::handlers::CreateTable;
use std::{thread, time};


pub struct TableManager;
impl TableManager {
    pub fn get(conn: &Addr<DatabaseExecutor>, query: ManagerQuery) -> Vec<Table> {
        vec![]
    }

    pub fn create(conn: &Addr<DatabaseExecutor>, tables: Table) -> () {

    }

    pub fn retrieve(conn: &Addr<DatabaseExecutor>) -> () {

    }

    pub fn insert(conn: &Addr<DatabaseExecutor>, on_duplicate: OnDuplicate) -> () {

    }

    pub fn update(conn: &Addr<DatabaseExecutor>) -> () {

    }

    pub fn delete(conn: &Addr<DatabaseExecutor>) -> () {

    }
}

