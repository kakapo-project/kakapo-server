
use log::LevelFilter;
use env_logger::{Builder, Target};
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse, Json,
};
use bytes::BytesMut;
use json::JsonValue;
use serde_derive;
use serde_json;
use json;
use std;

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

type AsyncResponse = Box<Future<Item=HttpResponse, Error=Error>>;


use actix::prelude::*;
use std::sync::Arc;

use model::connection;
use model::connection::DatabaseExecutor;

use actix_web::*;
use bytes::Bytes;
use futures::stream::once;
use futures::future::{Future, result};
use actix_web::{http::NormalizePath};

use model::table;
use model::data;
use model::handlers;
use model::table::TableManager;

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

/*
fn get_tables(req: &HttpRequest<AppState>) -> AsyncResponse {
    let state = State::<AppState>::extract(req);
    let body = once(Ok(Bytes::from_static(b"test")));

    result(Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(Body::Streaming(Box::new(body)))
    )).responder()
}
*/


// getting

fn get_tables(state: State<AppState>) -> AsyncResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    let table = TableManager::get(&state.db_connection, data::ManagerQuery::All);

    result(
        Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(Body::Streaming(Box::new(body)))
        )
    ).responder()
}

fn create_table((state, json): (State<AppState>, Json<u32>)) -> AsyncResponse {
    let body = once(Ok(Bytes::from_static(b"test")));
    let conn = &state.db_connection;

    println!("trying to send data");
    println!("is connected: {}", conn.connected());
    let res = conn
        .send(handlers::CreateTable("table_name".to_string()))
        .from_err()
        .and_then(|result| {
            println!("user account: {:?}", result.unwrap());
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(Body::Streaming(Box::new(body)))
            )
        });

    res.responder()
}

fn get_table((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    result(Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(Body::Streaming(Box::new(body)))
    )).responder()
}

fn create_or_update_table((state, path, json): (State<AppState>, Path<String>, Json<u32>)) -> AsyncResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    result(Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(Body::Streaming(Box::new(body)))
    )).responder()
}

fn delete_table((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    let body = once(Ok(Bytes::from_static(b"test")));

    result(Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(Body::Streaming(Box::new(body)))
    )).responder()
}


pub fn routes() -> App<AppState> {
    let connection = connection::create();
    let state = AppState::new(connection, "ninchy");
    App::with_state(state)
        .middleware(middleware::Logger::default())
        .resource("/api/table/", |r| {
            r.method(http::Method::GET).with(get_tables);
            r.method(http::Method::POST).with(create_table);
        })
        .resource("/api/table/{table_name}/", |r| {
            r.method(http::Method::GET).with(get_table);
            r.method(http::Method::PUT).with(create_or_update_table);
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