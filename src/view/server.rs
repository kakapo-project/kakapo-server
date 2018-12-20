
use actix::prelude::*;

use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web::{
    App, AsyncResponder, Error as ActixError, FromRequest,
    dev::JsonConfig, dev::Handler as MsgHandler, error, http, http::header::DispositionType, http::NormalizePath, middleware,
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

use serde;
use serde_derive;
use serde_json;

use std::error::Error;
use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;
use std::env;


use model::auth;
use connection;
use connection::executor::DatabaseExecutor;
use data::api;

// current module
use super::handlers;
use super::session::{TableSession, QuerySession, ScriptSession};
use super::state::AppState;

use super::routes::*;
use actix_web::middleware::cors::CorsBuilder;

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

/* TODO: put in actions */
struct _State {
    database: String,
    user: String,
}

struct _Error {

}

trait Action {
    type Return;
    fn call(&self, state: _State) -> Result<Self::Return, _Error>;
}

#[derive(Deserialize, Debug, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTablesAction {
    #[serde(default)]
    pub detailed: bool,
}

impl Action for GetTablesAction {
    type Return = serde_json::Value;
    fn call(&self, state: _State) -> Result<Self::Return, _Error> {
        Ok(serde_json::to_value(&json!({ "success": "get table procedure" })).unwrap())
    }

}

/* END */

//Api functions

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTablesQuery {
    #[serde(default)]
    pub detailed: bool,
    #[serde(default)]
    pub show_deleted: bool,
}


// build procedure (I don't like that this is depending on HttpRequest)
type ResultMessage = Result<serde_json::Value, api::Error>;

trait ProcedureBuilder<M: Action> {
    fn build(req: &HttpRequest<AppState>) -> M;
}

impl ProcedureBuilder<GetTablesAction> for GetTablesAction { // This is user generated
    fn build(req: &HttpRequest<AppState>) -> GetTablesAction {
        GetTablesAction { detailed: false }
    }
}

impl Handler<GetTablesAction> for DatabaseExecutor {
    type Result = <GetTablesAction as Message>::Result;

    fn handle(&mut self, msg: GetTablesAction, _: &mut Self::Context) -> Self::Result {
        //TODO:...
        Ok(serde_json::to_value(&json!({ "success": "get table procedure" })).unwrap())
    }
}

// handle procedure and turn into http result
struct ProcedureHandler
<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M>>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>
{
    procedure_builder: PB,
    ph: std::marker::PhantomData<M>,
}


impl<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static> ProcedureHandler<M, PB>
where
    M::Result: Send,
    DatabaseExecutor: Handler<M>,
{
    pub fn setup(procedure_builder: PB) -> Self {
        ProcedureHandler { procedure_builder, ph: std::marker::PhantomData }
    }
}


impl<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
    MsgHandler<AppState> for ProcedureHandler<M, PB>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
{
    type Result = AsyncResponse;

    fn handle(&self, req: &HttpRequest<AppState>) -> AsyncResponse {

        req.state()
            .connect(0 /* use master database connector for authentication */)
            .send(PB::build(req))
            .from_err()
            .and_then(|res| {
                let fin = HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&json!({ "success": "all is well" }))
                        .unwrap_or_default());

                Ok(fin)
            })
            .responder()
    }
}

// extend actix router
pub trait CorsBuilderExt {
    fn procedure<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>;

}

impl CorsBuilderExt for CorsBuilder<AppState> {
    fn procedure<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>,
    {
        self.resource(path, |r| {
            r.method(http::Method::POST).h(ProcedureHandler::<M, PB>::setup(procedure_builder));
        })
    }
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
                .procedure("/manage/getTables", GetTablesAction { detailed: false })
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