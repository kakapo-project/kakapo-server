

mod action_wrapper;
mod extensions;
mod procedure;
mod error;

use std::result::Result;
use std::result::Result::Ok;
use actix_web::test::TestApp;

// current module
use model::actions;
use connection::GetAppState;

use view::procedure::NoQuery;
use view::extensions::ProcedureExt;
use data;
use actix_web::middleware::cors::CorsBuilder;
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
pub struct Invite {
    pub email: String,
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

    pub fn authenticate(data: Value, query: Value) -> Result<impl Action, Error> {
        let auth_data: AuthData = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::Authenticate::<_>::new(auth_data.username, auth_data.password))
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
        let data: AuthData = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok(actions::SetUserPassword::<_>::new(data.username, data.password)) //TODO: add old password too
    }

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
        let role: data::auth::Role = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok(actions::AttachRoleForUser::<_>::new(get_user.user_identifier, role))
    }

    pub fn detach_role_for_user(data: Value, query: Value) -> Result<impl Action, Error> {
        let role: data::auth::Role = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok(actions::DetachRoleForUser::<_>::new(get_user.user_identifier, role))
    }

}

pub trait Router<S, B>
    where
        S: GetAppState<B> + 'static,
        B: Broadcaster,
{
    fn router(app: &mut Self) -> &mut Self;
}

macro_rules! implement_router {

    ($App:ident) => {
        impl<S, B> Router<S, B> for $App<S>
            where
                S: GetAppState<B> + 'static,
                B: Broadcaster,
        {
            fn router(app: &mut Self) -> &mut Self {
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
                    .procedure("/manage/modifyTableData", manage::modify_table_data)
                    .procedure("/manage/removeTableData", manage::remove_table_data)

                    .procedure("/manage/runQuery", manage::run_query)
                    .procedure("/manage/runScript", manage::run_script)

                    .procedure("/users/authenticate", users::authenticate)
                    .procedure("/users/getAllUsers", users::get_all_users)

                    .procedure("/users/addUser", users::add_user)
                    .procedure("/users/removeUser", users::remove_user)
                    .procedure("/users/inviteUser", users::invite_user)
                    .procedure("/users/setupUser", users::setup_user)
                    .procedure("/users/setUserPassword", users::set_user_password)

                    .procedure("/users/addRole", users::add_role)
                    .procedure("/users/removeRole", users::remove_role)
                    .procedure("/users/getAllRoles", users::get_all_roles)

                    .procedure("/users/attachPermissionForRole", users::attach_permission_for_role)
                    .procedure("/users/detachPermissionForRole", users::detach_permission_for_role)

                    .procedure("/users/attachRoleForUser", users::attach_role_for_user)
                    .procedure("/users/detachRoleForUser", users::detach_role_for_user)


            }
        }
    }
}

implement_router!(CorsBuilder);
implement_router!(TestApp);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_entities() {
        let data = json!({});
        let query = json!({
            "showDeleted": true
        });
        let all_tables_action = manage::get_all_tables(data.to_owned(), query.to_owned()).unwrap();
        let all_queries_action = manage::get_all_queries(data.to_owned(), query.to_owned()).unwrap();
        let all_scripts_action = manage::get_all_scripts(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_get_entity() {
        let data = json!({});
        let query = json!({
            "name": "foo"
        });
        let get_table_action = manage::get_table(data.to_owned(), query.to_owned()).unwrap();
        let get_query_action = manage::get_query(data.to_owned(), query.to_owned()).unwrap();
        let get_script_action = manage::get_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_create_entity() {
        let query = json!({});
        let mut data = json!({
            "name": "table_name",
            "description": "this is a really cool table",
            "schema": {
                "columns": [
                    {
                        "name": "col_a",
                        "dataType": "integer"
                    }
                ],
                "constraint": []
            }
        });
        let create_table_action = manage::create_table(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "query_name",
            "description": "this is a really cool query",
            "statement": "SELECT * FROM awesome_table"
        });
        let createt_query_action = manage::create_query(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "script_name",
            "description": "this is a really cool script",
            "text": "print('hello world')"
        });
        let create_script_action = manage::create_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_update_entity() {
        let query = json!({
            "name": "foo"
        });

        let mut data = json!({
            "name": "table_name",
            "description": "this is a really cool table",
            "schema": {
                "columns": [
                    {
                        "name": "col_a",
                        "dataType": "integer"
                    }
                ],
                "constraint": []
            }
        });
        let update_table_action = manage::update_table(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "query_name",
            "description": "this is a really cool query",
            "statement": "SELECT * FROM awesome_table"
        });
        let update_query_action = manage::update_query(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "script_name",
            "description": "this is a really cool script",
            "text": "print('hello world')"
        });
        let update_script_action = manage::update_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_delete_entity() {
        let data = json!({});
        let query = json!({
            "name": "foo"
        });
        let delete_table_action = manage::delete_table(data.to_owned(), query.to_owned()).unwrap();
        let delete_query_action = manage::delete_query(data.to_owned(), query.to_owned()).unwrap();
        let delete_script_action = manage::delete_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_query_table_data() {

    }

    #[test]
    fn test_insert_table_data() {

    }

    #[test]
    fn test_modify_table_data() {

    }

    #[test]
    fn test_remove_table_data() {

    }
}