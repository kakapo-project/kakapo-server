

use actix::prelude::*;

use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::{
    App, AsyncResponder, Error as ActixError, FromRequest,
    dev::JsonConfig, error, http, http::header::DispositionType, http::NormalizePath, middleware,
    HttpMessage, HttpRequest, HttpResponse, fs, fs::{NamedFile, StaticFileConfig, StaticFiles},
    Json, Path, Query, ResponseError, Responder, State, ws,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};

use dotenv::{dotenv};
use env_logger::{Builder, Target};

use futures::{future::{Future, result}, stream::once};
use futures::future;

use json;
use json::JsonValue;

use log::LevelFilter;

use chrono::Duration;

use model::{api, connection, connection::executor::DatabaseExecutor};

use serde;
use serde_derive;
use serde_json;

use std::error::Error;
use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;
use std::env;

use super::handlers;
use super::session::{TableSession, QuerySession, ScriptSession};
use super::state::AppState;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;


fn http_response<M: Message<Result = Result<serde_json::Value, api::Error>>>
(state: &AppState, msg: M) -> AsyncResponse
    where
        M: Send + 'static,
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
{
    let conn = &state.connect(0);
    conn
        .send(msg)
        .from_err()
        .and_then(|res| {
            let unwrapped_result = res?;
            println!("final result: {:?}", &unwrapped_result);
            let ok_result = serde_json::to_string(&unwrapped_result)?;
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(ok_result)
            )
        })
        .responder()
}


fn websocket_response<
    S: Actor<Context = ws::WebsocketContext<S, AppState>>,
    M: Message<Result = Result<serde_json::Value, api::Error>>
>(
    ctx: &mut ws::WebsocketContext<S, AppState>,
    action_name: &str,
    msg: M,
)
    where
        M: Send + 'static,
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
{
    ctx.state()
        .connect(0)
        .send(msg)
        .wait()
        .or_else(|err| Err(api::Error::TooManyConnections))
        .and_then(|res| {
            let unwrapped_result = res?;
            println!("final result: {:?}", &unwrapped_result);
            let response = json!({
                "action": action_name,
                "data": unwrapped_result
            });
            let ok_result = serde_json::to_string(&response)
                .or_else(|err| Err(api::Error::SerializationError))?;

            ctx.text(ok_result);
            Ok(())
        })
        .or_else(|err| {
            println!("encountered error: {:?}", &err);
            Err(err)
        });
}


// getting
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTablesQuery {
    #[serde(default)]
    pub detailed: bool,
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetQueriesQuery {
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostTablesQuery {
    #[serde(default)]
    pub on_duplicate: api::OnDuplicate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostQueriesQuery {
    #[serde(default)]
    pub on_duplicate: api::OnDuplicate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostScriptsQuery {
    #[serde(default)]
    pub on_duplicate: api::OnDuplicate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTable {
    #[serde(default)]
    pub detailed: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTableDataQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
    #[serde(default)]
    pub format: api::TableDataFormat,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetQueryDataQuery { //ha
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
    #[serde(default)]
    pub format: api::TableDataFormat,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsertTableDataQuery {
    #[serde(default)]
    pub format: api::TableDataFormat,
    #[serde(default)]
    pub on_duplicate: api::OnDuplicate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTableDataQuery {
    #[serde(default)]
    pub format: api::TableDataFormat,
}


pub fn get_tables((state, query): (State<AppState>, Query<GetTablesQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &query);
    http_response(&state,handlers::GetTables {
        detailed: query.detailed,
        show_deleted: query.show_deleted,
    })
}

pub fn get_queries((state, query): (State<AppState>, Query<GetQueriesQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &query);
    http_response(&state,handlers::GetQueries {
        show_deleted: query.show_deleted,
    })
}


pub fn get_scripts(state: State<AppState>) -> AsyncResponse {
    http_response(&state,handlers::GetScripts)
}


pub fn post_tables((state, json, query): (State<AppState>, Json<api::PostTable>, Query<PostTablesQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::CreateTable {
        reqdata: json.into_inner(),
        on_duplicate: query.into_inner().on_duplicate,
    })
}

pub fn post_queries((state, json, query): (State<AppState>, Json<api::PostQuery>, Query<PostQueriesQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::CreateQuery {
        reqdata: json.into_inner(),
        on_duplicate: query.into_inner().on_duplicate,
    })
}

pub fn post_scripts((state, json, query): (State<AppState>, Json<api::PostScript>, Query<PostScriptsQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::CreateScript {
        reqdata: json.into_inner(),
        on_duplicate: query.into_inner().on_duplicate,
    })
}


pub fn get_table((state, path, query): (State<AppState>, Path<String>, Query<GetTable>)) -> AsyncResponse {
    println!("received message: {:?}", &query);
    http_response(&state,handlers::GetTable {
        name: path.to_string(),
        detailed: query.detailed,
    })
}

pub fn get_query((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    http_response(&state,handlers::GetQuery {
        name: path.to_string(),
    })
}

pub fn get_script((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    http_response(&state,handlers::GetScript {
        name: path.to_string(),
    })
}

pub fn put_table((state, path, json): (State<AppState>, Path<String>, Json<api::PutTable>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::UpdateTable {
        name: path.to_string(),
        reqdata: json.into_inner(),
    })
}

pub fn put_query((state, path, json): (State<AppState>, Path<String>, Json<api::PutQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::UpdateQuery {
        name: path.to_string(),
        reqdata: json.into_inner(),
    })
}

pub fn put_script((state, path, json): (State<AppState>, Path<String>, Json<api::PutScript>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::UpdateScript {
        name: path.to_string(),
        reqdata: json.into_inner()
    })
}

pub fn delete_table((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    println!("deleting: {:?}", &path);
    http_response(&state,handlers::DeleteTable {
        name: path.to_string(),
    })
}

pub fn delete_query((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    println!("deleting: {:?}", &path);
    http_response(&state,handlers::DeleteQuery {
        name: path.to_string(),
    })
}

pub fn delete_script((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    println!("deleting: {:?}", &path);
    http_response(&state,handlers::DeleteScript {
        name: path.to_string(),
    })
}


pub fn post_query_data((state, path, json, query): (State<AppState>, Path<String>, Json<api::QueryParams>, Query<GetQueryDataQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::RunQuery {
        name: path.to_string(),
        start: query.start,
        end: query.end,
        format: query.format,
        params: json.into_inner(),
    })
}

pub fn post_script_data((state, path, json): (State<AppState>, Path<String>, Json<api::ScriptParam>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::RunScript {
        name: path.to_string(),
        params: json.into_inner(),
        py_runner: state.get_py_runner(),
    })
}

pub fn get_table_data((state, path, query): (State<AppState>, Path<String>, Query<GetTableDataQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &query);
    http_response(&state,handlers::GetTableData {
        name: path.to_string(),
        start: query.start,
        end: query.end,
        format: query.format,
    })
}


pub fn post_table_data((state, path, json, query): (State<AppState>, Path<String>, Json<api::TableData>, Query<InsertTableDataQuery>)) -> AsyncResponse {
    println!("received message: {:?}", &json);
    http_response(&state,handlers::InsertTableData {
        name: path.to_string(),
        data: json.into_inner(),
        format: query.format,
        on_duplicate: query.into_inner().on_duplicate,
    })
}

pub fn put_table_data((state, path, json, query): (State<AppState>, Path<(String, String)>, Json<api::RowData>, Query<UpdateTableDataQuery>)) -> AsyncResponse {
    http_response(&state,handlers::UpdateTableData {
        name: path.0.to_string(),
        key: path.1.to_string(),
        data: json.into_inner(),
        format: query.format,
    })
}

pub fn delete_table_data((state, path): (State<AppState>, Path<(String, String)>)) -> AsyncResponse {
    http_response(&state,handlers::DeleteTableData {
        name: path.0.to_string(),
        key: path.1.to_string(),
    })
}


impl TableSession {

    fn handle_action(&self, ctx: &mut <Self as Actor>::Context, table_session_request: api::TableSessionRequest) {
        match table_session_request {
            api::TableSessionRequest::GetTable => {
                websocket_response(ctx, "getTable", handlers::GetTable {
                    name: self.table_name.to_string(),
                    detailed: false,
                })
            },
            api::TableSessionRequest::GetTableData { begin, end } => {
                websocket_response(ctx, "getTableData", handlers::GetTableData {
                    name: self.table_name.to_string(),
                    start: begin,
                    end: end,
                    format: api::FLAT_TABLE_DATA_FORMAT,
                })
            },
            api::TableSessionRequest::Create { data } => {
                websocket_response(ctx, "create", handlers::InsertTableData { //TODO: this is upsert
                    name: self.table_name.to_string(),
                    data: data.into_table_data(),
                    format: api::FLAT_TABLE_DATA_FORMAT,
                    on_duplicate: api::OnDuplicate::default(),
                })
            },
            api::TableSessionRequest::Update { data, key } => {
                websocket_response(ctx, "update", handlers::UpdateTableData { //TODO: this is upsert
                    name: self.table_name.to_string(),
                    key: key,
                    data: data,
                    format: api::FLAT_TABLE_DATA_FORMAT,
                })
            },
            api::TableSessionRequest::Delete { data, key } => {
                //TODO: implement me
                websocket_response(ctx, "delete",handlers::DeleteTableData {
                    name: self.table_name.to_string(),
                    key: key,
                })
            },
        };

    }

}


impl QuerySession {

    fn handle_action(&self, ctx: &mut <Self as Actor>::Context, query_session_request: api::QuerySessionRequest) {
        match query_session_request {
            api::QuerySessionRequest::GetQuery => {
                websocket_response(ctx, "getQuery", handlers::GetQuery {
                    name: self.query_name.to_string(),
                })
            },
            api::QuerySessionRequest::PostQuery { data } => {
                websocket_response(ctx, "postQuery", handlers::CreateQuery { //TODO: should be UpdateQuery
                    reqdata: data,
                    on_duplicate: api::OnDuplicate::default(),
                })
            },
            api::QuerySessionRequest::RunQuery { begin, end, params } => {
                websocket_response(ctx, "runQuery", handlers::RunQuery {
                    name: self.query_name.to_string(),
                    start: begin,
                    end: end,
                    format: api::FLAT_TABLE_DATA_FORMAT,
                    params: params,
                })
            },

        };

    }

}

impl ScriptSession {

    fn handle_action(&self, ctx: &mut <Self as Actor>::Context, script_session_request: api::ScriptSessionRequest) {
        match script_session_request {
            api::ScriptSessionRequest::GetScript => {
                websocket_response(ctx, "getScript", handlers::GetScript {
                    name: self.script_name.to_string(),
                })
            },
            api::ScriptSessionRequest::PostScript { data } => {
                websocket_response(ctx, "postScript", handlers::CreateScript { //TODO: should be UpdateQuery
                    reqdata: data,
                    on_duplicate: api::OnDuplicate::default(),
                })
            },
            api::ScriptSessionRequest::RunScript { params } => {
                let py_runner = ctx.state().get_py_runner();
                websocket_response(ctx, "runScript", handlers::RunScript {
                    name: self.script_name.to_string(),
                    params: params,
                    py_runner: py_runner,
                })
            },

        };

    }

}

impl StreamHandler<ws::Message, ws::ProtocolError> for TableSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                // parse json
                serde_json::from_str(&text)
                    .or_else::<serde_json::error::Error, _>(|err| {
                        println!("Error occured while parsing websocket request: {:?}", err);
                        ctx.stop();
                        //TODO: send error message
                        Err(err)
                    })
                    .and_then(|table_session_request: api::TableSessionRequest| {
                        self.handle_action(ctx, table_session_request);
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

impl StreamHandler<ws::Message, ws::ProtocolError> for QuerySession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                // parse json
                serde_json::from_str(&text)
                    .or_else::<serde_json::error::Error, _>(|err| {
                        println!("Error occured while parsing websocket request: {:?}", err);
                        ctx.stop();
                        //TODO: send error message
                        Err(err)
                    })
                    .and_then(|query_session_request: api::QuerySessionRequest| {
                        self.handle_action(ctx, query_session_request);
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

impl StreamHandler<ws::Message, ws::ProtocolError> for ScriptSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                // parse json
                serde_json::from_str(&text)
                    .or_else::<serde_json::error::Error, _>(|err| {
                        println!("Error occured while parsing websocket request: {:?}", err);
                        ctx.stop();
                        //TODO: send error message
                        Err(err)
                    })
                    .and_then(|script_session_request: api::ScriptSessionRequest| {
                        self.handle_action(ctx, script_session_request);
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

pub fn websocket_table_listener(req: &HttpRequest<AppState>) -> Result<HttpResponse, ActixError> {
    let path = Path::<String>::extract(req);
    let name = path?.to_string();

    ws::start(req, TableSession::new(name))
}

pub fn websocket_query_listener(req: &HttpRequest<AppState>) -> Result<HttpResponse, ActixError> {
    let path = Path::<String>::extract(req);
    let name = path?.to_string();

    ws::start(req, QuerySession::new(name))
}

pub fn websocket_script_listener(req: &HttpRequest<AppState>) -> Result<HttpResponse, ActixError> {
    let path = Path::<String>::extract(req);
    let name = path?.to_string();

    ws::start(req, ScriptSession::new(name))
}