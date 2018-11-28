use actix_web::ws;
use actix::Actor;
use view::state::AppState;

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

pub struct QuerySession {
    pub query_name: String,
    pub session_id: usize,
}

impl Actor for QuerySession {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl QuerySession {
    pub fn new(query_name: String) -> Self {
        Self {
            query_name: query_name,
            session_id: 0,
        }
    }
}