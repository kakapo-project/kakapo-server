
use actix::prelude::*;

use actix_web::{
    App, Error as ActixError,
    dev::JsonConfig, error, http, http::NormalizePath, middleware,
    HttpRequest, HttpResponse, fs, fs::{NamedFile},
    ResponseError, State,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};

use dotenv::{dotenv};

use chrono::Duration;

use serde_json;

use std::error::Error;
use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;
use std::env;

use connection;
use connection::executor::DatabaseExecutor;
use data::api;

// current module
use super::state::AppState;

use super::actions;

use super::procedure::ProcedureBuilder;
use super::procedure::CorsBuilderExt;
use view::procedure;


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


//static routes
fn index(_state: State<AppState>) -> Result<NamedFile, ActixError> {
    let www_path = get_www_path();
    let path = fsPath::new(&www_path).join("index.html");
    Ok(NamedFile::open(path)?)
}

fn config(cfg: &mut JsonConfig<AppState>) -> () {
    cfg.limit(4096)
        .error_handler(|err, _req| {
            println!("error: {:?}", err);
            let response =  HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(serde_json::to_string(&json!({ "error": err.to_string() }))
                    .unwrap_or_default());
            error::InternalError::from_response(
                err, response).into()
        });
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTable {
    #[serde(default)]
    pub detailed: bool,
}

impl procedure::Parameters for GetTable {
    fn temp() {
        //...
    }
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
                .procedure(
                    "/manage/getTables",
                    |get_table: GetTable| actions::GetTablesAction { detailed: get_table.detailed })
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