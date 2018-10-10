
use log::LevelFilter;
use env_logger::{Builder, Target};
use actix_web::Error as ActixError;
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, HttpMessage,
    HttpRequest, HttpResponse, Json, ResponseError,
};
use bytes::BytesMut;
use json::JsonValue;
use serde;
use serde_derive;
use serde_json;
use json;

/*
#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses `HttpRequest::json()` for loading json object.
fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    req.json()
        .from_err()  // convert all errors into `Error`
        .and_then(|val: MyObj| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().json(val))  // <- send response
        })
        .responder()
}

/// This handler uses json extractor
fn extract_item(item: Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
fn extract_item_limit((item, _req): (Json<MyObj>, HttpRequest)) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_manual(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
    // HttpRequest::payload() is stream of Bytes objects
    req.payload()
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()

        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<MyObj>(&body)?;
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        })
        .responder()
}
*/

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;


use actix::prelude::*;

use model::connection;
use model::connection::DatabaseExecutor;

use actix_web::*;
use bytes::Bytes;
use futures::stream::once;
use futures::future::{Future, result};
use actix_web::{http::NormalizePath};
use actix_web::dev::JsonConfig;

use model::api;
use std::error::Error;
use super::handlers;

pub struct AppState {
    db_connection: Addr<DatabaseExecutor>,
    app_name: String,
}

impl AppState {
    pub fn new(connection: Addr<DatabaseExecutor>, app_name: &str) -> Self {
        AppState {
            db_connection: connection,
            app_name: app_name.to_string(),
        }
    }
}


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
struct GetTables {
    #[serde(default)]
    pub detailed: bool,
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
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
            let ok_result = match unwrapped_result {
                api::GetTablesResult::Tables(tables) => serde_json::to_string(&tables)?,
                api::GetTablesResult::DetailedTables(tables) => serde_json::to_string(&tables)?,
            };
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
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&json!({"success": true}))?)
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
            let ok_result = match unwrapped_result {
                api::GetTableResult::Table(table) => serde_json::to_string(&table)?,
                api::GetTableResult::DetailedTable(table) => serde_json::to_string(&table)?,
            };
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
    let state = AppState::new(connection, "ninchy");
    App::with_state(state)
        .middleware(middleware::Logger::default())
        .resource("/api/table", |r| {
            r.method(http::Method::GET).with(get_tables);
            r.method(http::Method::POST).with_config(post_tables, |((_, cfg),)| config(cfg));
        })
        .resource("/api/table/{table_name}", |r| {
            r.method(http::Method::GET).with(get_table);
            r.method(http::Method::PUT).with(put_table);
            r.method(http::Method::DELETE).with(delete_table);
        })
        /*
        .resource("/api/table/{table_name}/retrieve", |r| r.method(http::Method::GET).with(retrieve_table))
        .resource("/api/table/{table_name}/insert", |r| r.method(http::Method::POST).with(insert_into_table))
        .resource("/api/table/{table_name}/insert_or_update", |r| r.method(http::Method::POST).with(insert_or_update_table))
        .resource("/api/table/{table_name}/insert_or_ignore", |r| r.method(http::Method::POST).with(insert_or_ignore_table))
        .resource("/api/table/{table_name}/update", |r| r.method(http::Method::POST).with(update_table))
        .resource("/api/table/{table_name}/delete", |r| r.method(http::Method::POST).with(delete_from_table))
        */
        .default_resource(|r| r.h(NormalizePath::default()))
}