
use actix::prelude::*;

use actix_web::{
    App, Error as ActixError,
    http, http::NormalizePath,
    fs, fs::{NamedFile},
    State,
};

use actix_web::middleware::Logger;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use chrono::Duration;

use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;

use connection;
// current module
use model::actions;

use connection::AppState;
use connection::GetAppState;

use view::procedure::NoQuery;
use view::extensions::ProcedureExt;
use data;
use actix_web::middleware::cors::CorsBuilder;
use connection::Auth;
use connection::Broadcaster;

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
pub struct AuthData {
    pub username: String,
    pub password: String,
}

pub fn router<S, A, B>(app: &mut CorsBuilder<S>) -> &mut CorsBuilder<S>
    where
        S: GetAppState<A, B> + 'static,
        A: Auth,
        B: Broadcaster,
{
    app
        .procedure(
            "/manage/getAllTables",
            |_: NoQuery, get_all_entities: GetAllEntities|
                actions::GetAllEntities::<data::Table>::new(get_all_entities.show_deleted)
        )
        .procedure(
            "/manage/getAllQueries",
            |_: NoQuery, get_all_entities: GetAllEntities|
                actions::GetAllEntities::<data::Query>::new(get_all_entities.show_deleted)
        )
        .procedure(
            "/manage/getAllScripts",
            |_: NoQuery, get_all_entities: GetAllEntities|
                actions::GetAllEntities::<data::Script>::new(get_all_entities.show_deleted)
        )

        .procedure(
            "/manage/getTable",
            |_: NoQuery, get_entity: GetEntity|
                actions::GetEntity::<data::Table>::new(get_entity.name)
        )
        .procedure(
            "/manage/getQuery",
            |_: NoQuery, get_entity: GetEntity|
                actions::GetEntity::<data::Query>::new(get_entity.name)
        )
        .procedure(
            "/manage/getScript",
            |_: NoQuery, get_entity: GetEntity|
                actions::GetEntity::<data::Script>::new(get_entity.name)
        )
        .procedure(
            "/manage/createTable",
            |entity: data::Table, _: NoQuery|
                actions::CreateEntity::<data::Table>::new(entity)
        )
        .procedure(
            "/manage/createQuery",
            |entity: data::Query, _: NoQuery|
                actions::CreateEntity::<data::Query>::new(entity)
        )
        .procedure(
            "/manage/createScript",
            |entity: data::Script, _: NoQuery|
                actions::CreateEntity::<data::Script>::new(entity)
        )
        .procedure(
            "/manage/updateTable",
            |entity: data::Table, get_entity: GetEntity|
                actions::UpdateEntity::<data::Table>::new(get_entity.name, entity)
        )
        .procedure(
            "/manage/updateQuery",
            |entity: data::Query, get_entity: GetEntity|
                actions::UpdateEntity::<data::Query>::new(get_entity.name, entity)
        )
        .procedure(
            "/manage/updateScript",
            |entity: data::Script, get_entity: GetEntity|
                actions::UpdateEntity::<data::Script>::new(get_entity.name, entity)
        )
        .procedure(
            "/manage/deleteTable",
            |_: NoQuery, get_entity: GetEntity|
                actions::DeleteEntity::<data::Table>::new(get_entity.name)
        )
        .procedure(
            "/manage/deleteQuery",
            |_: NoQuery, get_entity: GetEntity|
                actions::DeleteEntity::<data::Query>::new(get_entity.name)
        )
        .procedure(
            "/manage/deleteScript",
            |_: NoQuery, get_entity: GetEntity|
                actions::DeleteEntity::<data::Script>::new(get_entity.name)
        )
        .procedure(
            "/manage/queryTableData",
            |_: NoQuery, get_table: GetEntity|
                actions::QueryTableData::<_>::new(get_table.name)
        )
        .procedure(
            "/manage/insertTableData",
            |data: data::TableData, get_table: GetEntity|
                actions::InsertTableData::<_>::new(get_table.name, data)
        )
        .procedure(
            "/manage/updateTableData",
            |keyed_data: data::KeyedTableData, get_table: GetEntity|
                actions::UpdateTableData::<_>::new(get_table.name, keyed_data)
        )
        .procedure(
            "/manage/deleteTableData",
            |keys: data::KeyData, get_table: GetEntity|
                actions::DeleteTableData::<_>::new(get_table.name, keys)
        )
        .procedure(
            "/manage/runQuery",
            |params: data::QueryParams, get_query: GetEntity|
                actions::RunQuery::<_>::new(get_query.name, params)
        )
        .procedure(
            "/manage/runScript",
            |param: data::ScriptParam, get_script: GetEntity|
                actions::RunScript::<_>::new(get_script.name, param)
        )
        .procedure(
            "/users/authenticate",
            |_: NoQuery, auth_data: AuthData|
                actions::users::Authenticate::<_>::new(auth_data.username, auth_data.password)
        )
}