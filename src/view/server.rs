
use actix::prelude::*;

use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::{
    App, AsyncResponder, Error as ActixError, FromRequest,
    dev::JsonConfig, error, http, http::header::DispositionType, http::NormalizePath, middleware,
    HttpMessage, HttpRequest, HttpResponse, fs, fs::{NamedFile, StaticFileConfig, StaticFiles},
    Json, Path, Query, ResponseError, Responder, State, ws,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService, RequestIdentity};

use dotenv::{dotenv};
use env_logger::{Builder, Target};

use futures::{future::{Future, result}, stream::once};
use futures::future;

use json;
use json::JsonValue;

use log::LevelFilter;

use chrono::Duration;

use connection;
use connection::executor::DatabaseExecutor;
use data::api;

use model::auth;

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

use super::routes::*;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;


//TODO: implement for own Response Type
impl ResponseError for api::Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(serde_json::to_string(&json!({ "error": self.description().to_string() })).unwrap())
    }
}

fn get_www_path() -> String {
    dotenv().expect("could not parse dotenv file");
    let www_path = env::var("WWW_PATH").expect("WWW_PATH must be set");
    www_path
}

fn get_script_path() -> String {
    dotenv().expect("could not parse dotenv file");
    let script_path = env::var("SCRIPTS_PATH").expect("SCRIPTS_PATH must be set");
    script_path
}

fn get_is_secure() -> bool {
    dotenv().expect("could not parse dotenv file");
    let is_secure = env::var("SECURE").expect("SECURE must be set");
    is_secure == "true"
}

fn get_domain() -> String {
    dotenv().expect("could not parse dotenv file");
    let domain = env::var("SERVER_DOMAIN").expect("SERVER_DOMAIN must be set");
    domain
}

fn get_secret_key() -> String {
    dotenv().expect("could not parse dotenv file");
    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    key
}


//auth routes
#[derive(Clone, Message, Deserialize)]
#[rtype(result="Result<auth::Token, auth::AuthError>")]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl Handler<LoginData> for DatabaseExecutor {
    type Result = <LoginData as Message>::Result;

    fn handle(&mut self, msg: LoginData, _: &mut Self::Context) -> Self::Result {
        auth::Token::create_new(&self.get_connection(), msg.username, msg.password)
    }
}

#[derive(Clone, Message, Deserialize)]
#[rtype(result="Result<(), api::Error>")]
pub struct LogoutUser { token: String }

impl Handler<LogoutUser> for DatabaseExecutor {
    type Result = <LogoutUser as Message>::Result;

    fn handle(&mut self, msg: LogoutUser, _: &mut Self::Context) -> Self::Result {
        // no session management necessary since we are using jwt, maybe put in some validation, but not necessary
        Ok(())
    }
}

#[derive(Clone, Message, Deserialize)]
#[rtype(result="Result<bool, api::Error>")]
pub struct IsUserLoggedIn { token: String }

impl Handler<IsUserLoggedIn> for DatabaseExecutor {
    type Result = <IsUserLoggedIn as Message>::Result;

    fn handle(&mut self, msg: IsUserLoggedIn, _: &mut Self::Context) -> Self::Result {
        // no session management necessary since we are using jwt, maybe put in some validation, but not necessary
        if msg.token == "" {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}



fn login((login_data, req): (Json<LoginData>, HttpRequest<AppState>)) -> AsyncResponse {
    req.state()
        .connect(0 /* use master database connector for authentication */)
        .send(login_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                req.remember("test".to_string());
                Ok(HttpResponse::Ok().into())
            },
            Err(err) => Ok(HttpResponse::Unauthorized()
               .content_type("application/json")
               .body(serde_json::to_string(&json!({ "error": "not authorized" }))
                   .unwrap_or_default())),
        }).responder()
}

fn is_logged_in(req: HttpRequest<AppState>) -> AsyncResponse {
    req.state()
        .connect(0 /* use master database connector for authentication */)
        .send(IsUserLoggedIn { token: req.identity().unwrap_or("".to_string()) })
        .from_err()
        .and_then(move |res| match res {
            Ok(is_authenticated) => {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&json!({ "auth": is_authenticated }))
                        .unwrap_or_default()))
            },
            Err(err) => Ok(err.error_response()),
        }).responder()
}

fn logout(req: HttpRequest<AppState>) -> AsyncResponse {

    req.state()
        .connect(0 /* use master database connector for authentication */)
        .send(LogoutUser { token: req.identity().unwrap_or("".to_string()) })
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                req.forget();
                Ok(HttpResponse::Ok().into())
            },
            Err(err) => Ok(err.error_response()),
        }).responder()
}

//static routes
fn index(state: State<AppState>) -> Result<NamedFile, ActixError> {
    let www_path = get_www_path();
    let path = fsPath::new(&www_path).join("index.html");
    Ok(NamedFile::open(path)?)
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


pub fn serve() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = vec![
        connection::executor::create(&database_url),
        //TODO: a connection for each user
    ];


    actix_web::server::new(move || {

        let www_path = get_www_path();
        let script_path = get_script_path();
        let state = AppState::new(connection.clone(), &script_path, "Kakapo");

        App::with_state(state)
            .middleware(middleware::Logger::default())
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(get_secret_key().as_bytes())
                    .name("kakapo-auth")
                    .path("/")
                    .domain(get_domain())
                    .max_age(Duration::days(1))
                    .secure(get_is_secure()), // this can only be true if you have https
            ))
            .configure(|app| Cors::for_app(app)
                //.allowed_origin("http://localhost:3000") //TODO: this doesn't work in the current version of cors middleware https://github.com/actix/actix-web/issues/603
                //.allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                //login
                .resource("/api/auth/login", |r| {
                    r.method(http::Method::POST).with(login);
                })
                .resource("/api/auth/is_logged_in", |r| {
                    r.method(http::Method::GET).with(is_logged_in);
                })
                .resource("/api/auth/logout", |r| {
                    r.method(http::Method::DELETE).with(logout);
                })
                // metadata
                .resource("/api/manage/table", |r| {
                    r.method(http::Method::GET).with(get_tables);
                    r.method(http::Method::POST).with_config(post_tables, |((_, cfg, _),)| config(cfg));
                })
                .resource("/api/manage/table/{table_name}", |r| {
                    r.method(http::Method::GET).with(get_table);
                    r.method(http::Method::PUT).with(put_table);
                    r.method(http::Method::DELETE).with(delete_table);
                })
                .resource("/api/manage/query", |r| {
                    r.method(http::Method::GET).with(get_queries);
                    r.method(http::Method::POST).with_config(post_queries, |((_, cfg, _),)| config(cfg));
                })
                .resource("/api/manage/query/{query_name}", |r| {
                    r.method(http::Method::GET).with(get_query);
                    r.method(http::Method::PUT).with(put_query);
                    r.method(http::Method::DELETE).with(delete_query);
                })
                .resource("/api/manage/script", |r| {
                    r.method(http::Method::GET).with(get_scripts);
                    r.method(http::Method::POST).with_config(post_scripts, |((_, cfg, _),)| config(cfg));
                })
                .resource("/api/manage/script/{script_name}", |r| {
                    r.method(http::Method::GET).with(get_script);
                    r.method(http::Method::PUT).with(put_script);
                    r.method(http::Method::DELETE).with(delete_script);
                })
                //data
                .resource("/api/table/{table_name}", |r| {
                    r.method(http::Method::GET).with(get_table_data);
                    r.method(http::Method::POST).with_config(post_table_data, |((_, _, cfg, _),)| config(cfg));
                })
                .resource("/api/table/{table_name}/{id}", |r| {
                    r.method(http::Method::PUT).with(put_table_data);
                    r.method(http::Method::DELETE).with(delete_table_data);
                })
                .resource("/api/query/{query_name}", |r| {
                    r.method(http::Method::POST).with_config(post_query_data, |((_, _, cfg, _),)| config(cfg));
                })
                .resource("/api/script/{script_name}", |r| {
                    r.method(http::Method::POST).with_config(post_script_data, |((_, _, cfg),)| config(cfg));
                })
                //Websockets
                .resource("/api/listen/table/{table_name}", |r| {
                    r.method(http::Method::GET).f(websocket_table_listener)
                })
                .resource("/api/listen/query/{query_name}", |r| {
                    r.method(http::Method::GET).f(websocket_query_listener)
                })
                .resource("/api/listen/script/{script_name}", |r| {
                    r.method(http::Method::GET).f(websocket_script_listener)
                })
                .register())
            .resource("/", |r| {
                r.method(http::Method::GET).with(index)
            })
            .default_resource(|r| r.h(NormalizePath::default()))
            .handler(
                "/",
                fs::StaticFiles::new(fsPath::new(&www_path))
                    .unwrap()
                    .show_files_listing())
        })
        .workers(num_cpus::get())
        .keep_alive(None)
        .bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
}