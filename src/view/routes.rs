
use model::actions;

use view::procedure::NoQuery;
use view::extensions::ProcedureExt;
use data;
use actix_web::middleware::cors::CorsBuilder;
use model::actions::Action;
use serde_json::Value;
use serde_json::Error;
use serde_json::from_value;
use connection::AppStateLike;
use view::procedure::ProcedureBuilder;
use view::procedure::call_action;

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
pub struct GetUser {
    #[serde(rename = "username")]
    pub user_identifier: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRole {
    pub rolename: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub email: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RoleData {
    pub name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PasswordResetRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
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

    pub fn modify_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let keyed_data: data::KeyedTableData = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::ModifyTableData::<_>::new(get_table.name, keyed_data))
    }

    pub fn remove_table_data(data: Value, query: Value) -> Result<impl Action, Error> {
        let keys: data::KeyData = from_value(data)?;
        let get_table: GetEntity = from_value(query)?;
        Ok(actions::RemoveTableData::<_>::new(get_table.name, keys))
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

    pub fn login(data: Value, query: Value) -> Result<impl Action, Error> {
        let auth_data: AuthData = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::Login::<_>::new(auth_data.username, auth_data.password))
    }

    pub fn refresh(data: Value, query: Value) -> Result<impl Action, Error> {
        let auth_data: RefreshToken = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::Refresh::<_>::new(auth_data.refresh_token))
    }

    pub fn logout(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::Logout::<_>::new())
    }

    pub fn get_all_users(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::GetAllUsers::<_>::new())
    }

    pub fn add_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let new_user: data::auth::NewUser = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::AddUser::<_>::new(new_user))
    }

    pub fn remove_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok(actions::RemoveUser::<_>::new(get_user.user_identifier))
    }

    pub fn invite_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let invite: Invite = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::InviteUser::<_>::new(invite.email))
    }

    pub fn setup_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let new_user: data::auth::NewUser = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::SetupUser::<_>::new(new_user))
    }

    pub fn set_user_password(data: Value, query: Value) -> Result<impl Action, Error> {
        let data: PasswordResetRequest = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::SetUserPassword::<_>::new(data.username, data.new_password)) //TODO: add old password too
    }

    //TODO: modify user

    pub fn add_role(data: Value, query: Value) -> Result<impl Action, Error> {
        let role: data::auth::Role = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::AddRole::<_>::new(role))
    }

    pub fn remove_role(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok(actions::RemoveRole::<_>::new(role_id.rolename))
    }

    pub fn get_all_roles(data: Value, query: Value) -> Result<impl Action, Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::GetAllRoles::<_>::new())
    }

    pub fn attach_permission_for_role(data: Value, query: Value) -> Result<impl Action, Error> {
        let permission: data::permissions::Permission = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok(actions::AttachPermissionForRole::<_>::new(role_id.rolename, permission))
    }

    pub fn detach_permission_for_role(data: Value, query: Value) -> Result<impl Action, Error> {
        let permission: data::permissions::Permission = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok(actions::DetachPermissionForRole::<_>::new(role_id.rolename, permission))
    }

    pub fn attach_role_for_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let role: RoleData = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok(actions::AttachRoleForUser::<_>::new(get_user.user_identifier, role.name))
    }

    pub fn detach_role_for_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let role: RoleData = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok(actions::DetachRoleForUser::<_>::new(get_user.user_identifier, role.name))
    }

}


pub fn call_procedure<S>(
    procedure: &str,
    state: &S,
    query: serde_json::Value,
    data: serde_json::Value
) -> Result<serde_json::Value, serde_json::Value>
    where
        S: AppStateLike + 'static
{

    //TODO: put this in a macro, we are using this in the routes as well
    match procedure {
        "getAllTables" => call_action(manage::get_all_tables, state, data, query),
        "getAllQueries" => call_action(manage::get_all_queries, state, data, query),
        "getAllScripts" => call_action(manage::get_all_scripts, state, data, query),

        "getTable" => call_action(manage::get_table, state, data, query),
        "getQuery" => call_action(manage::get_query, state, data, query),
        "getScript" => call_action(manage::get_script, state, data, query),

        "createTable" => call_action(manage::create_table, state, data, query),
        "createQuery" => call_action(manage::create_query, state, data, query),
        "createScript" => call_action(manage::create_script, state, data, query),

        "updateTable" => call_action(manage::update_table, state, data, query),
        "updateQuery" => call_action(manage::update_query, state, data, query),
        "updateScript" => call_action(manage::update_script, state, data, query),

        "deleteTable" => call_action(manage::delete_table, state, data, query),
        "deleteQuery" => call_action(manage::delete_query, state, data, query),
        "deleteScript" => call_action(manage::delete_script, state, data, query),

        "queryTableData" => call_action(manage::query_table_data, state, data, query),
        "insertTableData" => call_action(manage::insert_table_data, state, data, query),
        "modifyTableData" => call_action(manage::modify_table_data, state, data, query),
        "removeTableData" => call_action(manage::remove_table_data, state, data, query),

        "runQuery" => call_action(manage::run_query, state, data, query),
        "runScript" => call_action(manage::run_script, state, data, query),

        _ => Err(json!({"error": "Could not understand command"})),
    }

}