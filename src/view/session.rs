use actix_web::ws;
use actix::Addr;
use model::connection::DatabaseExecutor;
use actix::Actor;
use view::state::AppState;
use diesel::{prelude::*, r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;
use std::sync::Arc;

pub struct TableSession {
    pub table_name: String,
    pub session_id: usize,
}

impl Actor for TableSession {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl TableSession {
    pub fn new(table_name: String) -> Self {
        Self {
            table_name: table_name,
            session_id: 0,
        }
    }
}