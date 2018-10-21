
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
            let api::CreateTableResult(table) = res?;
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&table)?)
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

fn get_table_data((state, path): (State<AppState>, Path<String>)) -> AsyncResponse {
    let conn = &state.db_connection;

    conn.send(handlers::GetTableData {
            name: path.to_string(),
        })
        .from_err()
        .and_then(|res| {
            let unwrapped_result = res?;
            println!("final result: {:?}", &unwrapped_result);
            let api::GetTableDataResult(table_with_data) = unwrapped_result;
            let data = table_with_data.data; //TODO: just need the data, give the user the option to have it all maybe?
            let ok_result = serde_json::to_string(&data)?;
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(ok_result)
            )
        })
        .responder()
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