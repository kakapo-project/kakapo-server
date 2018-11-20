
use actix::prelude::*;

use actix::Arbiter;
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::{
    App, AsyncResponder, Error as ActixError, FromRequest,
    dev::JsonConfig, error, http, http::header::DispositionType, http::NormalizePath, middleware,
    server, HttpMessage, HttpRequest, HttpResponse, fs, fs::{NamedFile, StaticFileConfig, StaticFiles},
    Json, Path, Query, ResponseError, Responder, State, ws,
};

use bytes::{Bytes, BytesMut};

use env_logger::{Builder, Target};

use futures::{future::{Future, result}, stream::once};
use futures::future;

use json;
use json::JsonValue;

use log::LevelFilter;

use model::{api, connection, connection::DatabaseExecutor};

use serde;
use serde_derive;
use serde_json;

use std::{error::Error, path::PathBuf};
use std::result::Result;
use std::result::Result::Ok;

use super::handlers;
use super::session::TableSession;
use super::state::AppState;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;


//TODO: implement for own Response Type
impl ResponseError for api::Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": self.description().to_string() })).unwrap())
    }
}




// getting
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTables {
    #[serde(default)]
    pub detailed: bool,
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GetTable {
    #[serde(default)]
    pub detailed: bool,
}


fn get_tables((state, query): (State<AppState>, Query<GetTables>)) -> AsyncResponse {
    let conn = &state.db_connection;

    println!("received message: {:?}", &query);
    conn.send(handlers::GetTables {
            detailed: query.detailed,
            show_deleted: query.show_deleted,
        })
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



fn post_tables((state, json): (State<AppState>, Json<api::PostTable>)) -> AsyncResponse {
    let conn = &state.db_connection;

    println!("received message: {:?}", &json);
    conn.send(handlers::CreateTable {
            reqdata: json.into_inner()
        })
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

fn get_table((state, path, query): (State<AppState>, Path<String>, Query<GetTable>)) -> AsyncResponse {
    let conn = &state.db_connection;

    println!("received message: {:?}", &query);
    conn.send(handlers::GetTable {
            name: path.to_string(),
            detailed: query.detailed,
        })
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

fn put_table((state, path, json): (State<AppState>, Path<String>, Json<u32>)) -> AsyncResponse {

    result(Ok(
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": "method not implemented" })).unwrap())
    )).responder()
}

fn delete_table((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {

    result(Ok(
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": "method not implemented" })).unwrap())
    )).responder()
}

fn get_table_data((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    let conn = &state.db_connection;

    conn.send(handlers::GetTableData {
            name: path.to_string(),
        })
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

fn post_table_data((state, path, json): (State<AppState>, Path<String>, Json<api::TableData>)) -> AsyncResponse {
    //TODO: on duplicate - update (default), ignore, fail
    //TODO: implement
    let conn = &state.db_connection;

    conn.send(handlers::InsertTableData {
            name: path.to_string(),
            data: json.into_inner(),
        })
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

fn put_table_data((state, path, json): (State<AppState>, Path<(String, String)>, Json<u32>)) -> AsyncResponse {
    //TODO: implement
    result(Ok(
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": "method not implemented" })).unwrap())
    )).responder()
}

fn delete_table_data((state, path): (State<AppState>, Path<(String, String)>)) -> AsyncResponse {
    //TODO: implement
    result(Ok(
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": "method not implemented" })).unwrap())
    )).responder()
}


impl TableSession {

    fn handle_action(&self, ctx: &mut <Self as Actor>::Context, table_session_request: api::TableSessionRequest) -> () {
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
        }
    }

}


pub struct Test {
    pub name: String,
}

impl Message for Test {
    type Result = Result<String, api::Error>;
}

impl Handler<Test> for DatabaseExecutor {
    type Result = Result<String, api::Error>;

    fn handle(&mut self, msg: Test, _: &mut Self::Context) -> Self::Result {
        println!("got a message! {}", msg.name);

        Ok("ok".to_string())
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for TableSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                /*
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
                */
                println!("starting...");
                //send to handler
                let res = ctx.state().db_connection
                    .send(Test {
                        name: "hi!".to_string(),
                    })
                    .wait().unwrap().unwrap();
                ctx.text(format!("result: {}", &res));
            },
            ws::Message::Close(_) => {
                ctx.stop();
            },
            _ => {}
        }
    }
}

fn websocket_table_listener(req: &HttpRequest<AppState>) -> Result<HttpResponse, ActixError> {
    let path = Path::<String>::extract(req);
    let table_name = path?.to_string();

    ws::start(req, TableSession::new(table_name))
}


fn index(state: State<AppState>) -> Result<NamedFile, ActixError> {
    Ok(NamedFile::open("./www/index.html")?)
}

fn config(cfg: &mut JsonConfig<AppState>) -> () {
    cfg.limit(4096)
        .error_handler(|err, req| {
            println!("error: {:?}", err);
            let response =  HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(serde_json::to_string(&json!({ "error": err.to_string() }))
                    .unwrap_or_default());
            error::InternalError::from_response(
                err, response).into()
        });
}

pub fn routes() -> App<AppState> {
    let connection = connection::create();
    let state = AppState::new(connection.clone(), "Kakapo");
    App::with_state(state)
        .middleware(middleware::Logger::default())
        .resource("/", |r| {
            r.method(http::Method::GET).with(index)
        })
        .resource("/api/manage/table", |r| {
            r.method(http::Method::GET).with(get_tables);
            r.method(http::Method::POST).with_config(post_tables, |((_, cfg),)| config(cfg));
        })
        .resource("/api/manage/table/{table_name}", |r| {
            r.method(http::Method::GET).with(get_table);
            r.method(http::Method::PUT).with(put_table);
            r.method(http::Method::DELETE).with(delete_table);
        })
        .resource("/api/table/{table_name}", |r| {
            r.method(http::Method::GET).with(get_table_data);
            r.method(http::Method::POST).with(post_table_data);
        })
        .resource("/api/table/{table_name}/{id}", |r| {
            r.method(http::Method::PUT).with(put_table_data);
            r.method(http::Method::DELETE).with(delete_table_data);
        })
        .resource("/api/listen/table/{table_name}", |r| {
            r.method(http::Method::GET).f(websocket_table_listener)
        })
        .default_resource(|r| r.h(NormalizePath::default()))
        .handler(
            "/",
            fs::StaticFiles::new("./www/")
                .unwrap()
                .show_files_listing())
}