
use actix::prelude::*;

use actix_web::{
    App, AsyncResponder, Error, dev::JsonConfig,
    http, http::NormalizePath, http::Method,
    HttpMessage, middleware, HttpRequest, HttpResponse,
    fs, fs::NamedFile,
    ResponseError, State, ws,
};

mod server;
mod client;

use state::AppState;
use sockets::client::WsClientSession;

#[derive(Debug, Clone, Message)]
pub struct Notification {
    data: serde_json::Value,
}

impl Notification {
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
    pub fn get_data(self) -> serde_json::Value {
        self.data
    }
}

pub fn handler(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, WsClientSession::new())
}