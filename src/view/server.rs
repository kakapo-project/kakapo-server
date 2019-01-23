
use actix::prelude::*;

use actix_web::{
    App, Error as ActixError,
    http, http::NormalizePath,
    fs, fs::{NamedFile},
    State,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::Logger;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use chrono::Duration;

use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;

use connection;
// current module
use model::actions;

use super::state::AppState;
use super::procedure::NoQuery;
use super::extensions::ProcedureExt;
use data;
use view::environment::Env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllEntities {
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEntity {
    pub name: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SocketRequest {
    GetTables { show_deleted: bool },
    StopGetTables,
}


#[derive(Clone)]
struct SessionHandler {}

impl SessionHandler {
    pub fn new() -> Self {
        Self {}
    }
}


pub fn serve() {

    let connection = connection::executor::Connector::new()
        .host(Env::database_host())
        .port(Env::database_port())
        .user(Env::database_user())
        .pass(Env::database_pass())
        .db(Env::database_db())
        .script_path(Env::script_path())
        .done();

    let server_addr = Env::server_addr();
    let is_secure = Env::is_secure();

    let mut server_cfg = actix_web::server::new(move || {

        let state = AppState::new(connection.clone(),"Kakapo");

        App::with_state(state)
            .middleware(Logger::new("Responded [%s] %b bytes %Dms"))
            .middleware(Logger::new("Requested [%r] FROM %a \"%{User-Agent}i\""))
            .middleware(IdentityService::new(
                CookieIdentityPolicy::new(Env::secret_key().as_bytes())
                    .name("kakapo-server")
                    .path("/")
                    .domain(Env::domain())
                    .max_age(Duration::days(1))
                    .secure(is_secure),
            ))
            .scope("/manage", |scope| {
                scope
                .procedure(
                    "/getAllTables",
                    |_: NoQuery, get_all_entities: GetAllEntities|
                        actions::GetAllEntities::<data::Table>::new(get_all_entities.show_deleted)
                )
                .procedure(
                    "/getAllQueries",
                    |_: NoQuery, get_all_entities: GetAllEntities|
                        actions::GetAllEntities::<data::Query>::new(get_all_entities.show_deleted)
                )
                .procedure(
                    "/getAllScripts",
                    |_: NoQuery, get_all_entities: GetAllEntities|
                        actions::GetAllEntities::<data::Script>::new(get_all_entities.show_deleted)
                )

                .procedure(
                    "/getTable",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::GetEntity::<data::Table>::new(get_entity.name)
                )
                .procedure(
                    "/getQuery",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::GetEntity::<data::Query>::new(get_entity.name)
                )
                .procedure(
                    "/getScript",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::GetEntity::<data::Script>::new(get_entity.name)
                )
                .procedure(
                    "/createTable",
                    |entity: data::Table, _: NoQuery|
                        actions::CreateEntity::<data::Table>::new(entity)
                )
                .procedure(
                    "/createQuery",
                    |entity: data::Query, _: NoQuery|
                        actions::CreateEntity::<data::Query>::new(entity)
                )
                .procedure(
                    "/createScript",
                    |entity: data::Script, _: NoQuery|
                        actions::CreateEntity::<data::Script>::new(entity)
                )
                .procedure(
                    "/updateTable",
                    |entity: data::Table, get_entity: GetEntity|
                        actions::UpdateEntity::<data::Table>::new(get_entity.name, entity)
                )
                .procedure(
                    "/updateQuery",
                    |entity: data::Query, get_entity: GetEntity|
                        actions::UpdateEntity::<data::Query>::new(get_entity.name, entity)
                )
                .procedure(
                    "/updateScript",
                    |entity: data::Script, get_entity: GetEntity|
                        actions::UpdateEntity::<data::Script>::new(get_entity.name, entity)
                )
                .procedure(
                    "/deleteTable",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::DeleteEntity::<data::Table>::new(get_entity.name)
                )
                .procedure(
                    "/deleteQuery",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::DeleteEntity::<data::Query>::new(get_entity.name)
                )
                .procedure(
                    "/deleteScript",
                    |_: NoQuery, get_entity: GetEntity|
                        actions::DeleteEntity::<data::Script>::new(get_entity.name)
                )
                .procedure(
                    "/queryTableData",
                    |_: NoQuery, get_table: GetEntity|
                        actions::QueryTableData::<_>::new(get_table.name)
                )
                .procedure(
                    "/insertTableData",
                    |data: data::TableData, get_table: GetEntity|
                        actions::InsertTableData::<_>::new(get_table.name, data)
                )
                .procedure(
                    "/updateTableData",
                    |keyed_data: data::KeyedTableData, get_table: GetEntity|
                        actions::UpdateTableData::<_>::new(get_table.name, keyed_data)
                )
                .procedure(
                    "/deleteTableData",
                    |keys: data::KeyData, get_table: GetEntity|
                        actions::DeleteTableData::<_>::new(get_table.name, keys)
                )
                .procedure(
                    "/runQuery",
                    |params: data::QueryParams, get_query: GetEntity|
                        actions::RunQuery::<_>::new(get_query.name, params)
                )
                .procedure(
                    "/runScript",
                    |param: data::ScriptParam, get_script: GetEntity|
                        actions::RunScript::<_>::new(get_script.name, param)
                )
            })
            .default_resource(|r| r.h(NormalizePath::default()))
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