
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
use model::actions::Action;
use serde_json::Value;
use serde_json::Error;
use serde_json::from_value;

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

pub mod manage {
    use super::*;

    pub fn get_all_tables(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        Ok(actions::GetAllEntities::<data::Table>::new(get_all_entities.show_deleted))
    }

    pub fn get_all_queries(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        Ok(actions::GetAllEntities::<data::Query>::new(get_all_entities.show_deleted))
    }

    pub fn get_all_scripts(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        Ok(actions::GetAllEntities::<data::Script>::new(get_all_entities.show_deleted))
    }

    pub fn get_table(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::GetEntity::<data::Table>::new(get_entity.name))
    }

    pub fn get_query(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::GetEntity::<data::Query>::new(get_entity.name))
    }

    pub fn get_script(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::GetEntity::<data::Script>::new(get_entity.name))
    }

    pub fn create_table(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Table = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::CreateEntity::<data::Table>::new(entity))
    }

    pub fn create_query(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Query = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::CreateEntity::<data::Query>::new(entity))
    }

    pub fn create_script(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Script = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::CreateEntity::<data::Script>::new(entity))
    }

    pub fn update_table(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Table = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::UpdateEntity::<data::Table>::new(get_entity.name, entity))
    }

    pub fn update_query(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Query = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::UpdateEntity::<data::Query>::new(get_entity.name, entity))
    }

    pub fn update_script(data: Value, query: Value) -> Result<impl Action, Error> {
        let entity: data::Script = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::UpdateEntity::<data::Script>::new(get_entity.name, entity))
    }

    pub fn delete_table(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::DeleteEntity::<data::Table>::new(get_entity.name))
    }

    pub fn delete_query(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::DeleteEntity::<data::Query>::new(get_entity.name))
    }

    pub fn delete_script(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        Ok(actions::DeleteEntity::<data::Script>::new(get_entity.name))
    }

    pub fn query_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::QueryTableData::<_>::new(get_table.name))
    }

    pub fn insert_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let table_data: data::TableData = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::InsertTableData::<_>::new(get_table.name, table_data))
    }

    pub fn update_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let keyed_data: data::KeyedTableData = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::UpdateTableData::<_>::new(get_table.name, keyed_data))
    }

    pub fn delete_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let keys: data::KeyData = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::DeleteTableData::<_>::new(get_table.name, keys))
    }

    pub fn run_query(data: Value, query: Value) -> Result<impl Action, Error> {
        let params: data::QueryParams = from_value(data)?;
        let get_query: GetEntity = from_value(query)?;
        Ok(actions::RunQuery::<_>::new(get_query.name, params))
    }

    pub fn run_script(data: Value, query: Value) -> Result<impl Action, Error> {
        let param: data::ScriptParam = from_value(data)?;
        let get_script: GetEntity = from_value(query)?;
        Ok(actions::RunScript::<_>::new(get_script.name, param))
    }
}

pub mod users {
    use super::*;

    pub fn authenticate(data: Value, query: Value) -> Result<impl Action, Error> {
        let auth_data: AuthData = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::users::Authenticate::<_>::new(auth_data.username, auth_data.password))
    }
}

pub fn router<S, A, B>(app: &mut CorsBuilder<S>) -> &mut CorsBuilder<S>
    where
        S: GetAppState<A, B> + 'static,
        A: Auth,
        B: Broadcaster,
{
    app
        .procedure("/manage/getAllTables", manage::get_all_tables)
        .procedure("/manage/getAllQueries", manage::get_all_queries)
        .procedure("/manage/getAllScripts", manage::get_all_scripts)

        .procedure("/manage/getTable", manage::get_table)
        .procedure("/manage/getQuery", manage::get_query)
        .procedure("/manage/getScript", manage::get_script)

        .procedure("/manage/createTable", manage::create_table)
        .procedure("/manage/createQuery", manage::create_query)
        .procedure("/manage/createScript", manage::create_script)

        .procedure("/manage/updateTable", manage::update_table)
        .procedure("/manage/updateQuery", manage::update_query)
        .procedure("/manage/updateScript", manage::update_script)

        .procedure("/manage/deleteTable", manage::delete_table)
        .procedure("/manage/deleteQuery", manage::delete_query)
        .procedure("/manage/deleteScript", manage::delete_script)

        .procedure("/manage/queryTableData", manage::query_table_data)
        .procedure("/manage/insertTableData", manage::insert_table_data)
        .procedure("/manage/updateTableData", manage::update_table_data)
        .procedure("/manage/deleteTableData", manage::delete_table_data)

        .procedure("/manage/runQuery", manage::run_query)
        .procedure("/manage/runScript", manage::run_script)

        .procedure("/users/authenticate", users::authenticate)
}