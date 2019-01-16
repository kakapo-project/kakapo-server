
mod environment;
mod state;
mod extensions;
mod session;
mod actions;
mod action_wrapper;

use actix::prelude::*;

use actix_web::{
    App, Error as ActixError,
    dev::JsonConfig, error as http_error, http, http::NormalizePath, middleware,
    HttpRequest, HttpResponse, fs, fs::{NamedFile},
    ResponseError, State,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::Logger;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use chrono::Duration;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;

use std::error::Error;

use server::environment::Env;
use server::state::AppState;
use actix_web::Path;
use actix_web::Responder;
use server::action_wrapper::Connector;

//static routes
fn index(_state: State<AppState>) -> Result<NamedFile, ActixError> {
    let www_path = Env::www_path();
    let path = fsPath::new(&www_path).join("index.html");
    Ok(NamedFile::open(path)?)
}


fn test_route(req: &HttpRequest<AppState>) -> String {
    "test data".to_string()
}

pub fn serve() {

    let connection = Connector::new()
        .host(Env::database_host())
        .port(Env::database_port())
        .user(Env::database_user())
        .pass(Env::database_pass())
        .db(Env::database_db())
        .done();

    let server_addr = Env::server_addr();
    let is_secure = Env::is_secure();

    let mut server_cfg = actix_web::server::new(move || {

        let www_path = Env::www_path();
        let script_path = Env::script_path();
        let state = AppState::new(connection.clone(), &script_path, "KakapoArbiter");

        App::with_state(state)
            .middleware(Logger::new("Responded [%s] %b bytes %Dms"))
            .middleware(Logger::new("Requested [%r] FROM %a \"%{User-Agent}i\""))
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(Env::secret_key().as_bytes())
                    .name("kakapo-arbiter")
                    .path("/")
                    .domain(Env::domain())
                    .max_age(Duration::days(1))

                    .secure(is_secure), // this can only be true if you have https
            ))
            .configure(|app| Cors::for_app(app)
                //.allowed_origin("http://localhost:3000") //TODO: this doesn't work in the current version of cors middleware https://github.com/actix/actix-web/issues/603
                //.allowed_origin("http://localhost:8080")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .resource("/test", |r| r.f(test_route))
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
    });

    server_cfg = server_cfg
        .workers(num_cpus::get())
        .keep_alive(30);

    let http_server = if is_secure {
        let ssl_cert_privkey_path = Env::ssl_cert_privkey_path();
        let ssl_cert_fullchain_path = Env::ssl_cert_fullchain_path();

        let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_builder
            .set_private_key_file(ssl_cert_privkey_path, SslFiletype::PEM)
            .unwrap();
        ssl_builder.set_certificate_chain_file(ssl_cert_fullchain_path).unwrap();


        server_cfg
            .bind_ssl(&server_addr, ssl_builder)
            .unwrap()

    } else {
        server_cfg
            .bind(&server_addr)
            .unwrap()
    };

    http_server
        .shutdown_timeout(30)
        .start();

    info!("Kakapo server started on \"{}\"", &server_addr);

}