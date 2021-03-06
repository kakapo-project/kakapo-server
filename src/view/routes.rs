
use model::actions;

use view::procedure::NoQuery;
use data;
use model::actions::Action;
use serde_json::Value;
use serde_json::Error;
use serde_json::from_value;
use connection::AppStateLike;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAllEntities {
    pub domain: String,
    #[serde(default)]
    pub show_deleted: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetEntity {
    pub name: String,
    pub domain: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFromDomain {
    pub domain: String,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TimeRange {
    #[serde(rename = "start")]
    pub start_time: chrono::NaiveDateTime,
    #[serde(rename = "end")]
    pub end_time: chrono::NaiveDateTime,
}


pub mod manage {
    use super::*;

    pub fn get_all_domains(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::GetAllDomains::<_>::new()))
    }

    pub fn get_all_tables(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        let domain = get_all_entities.domain;
        Ok((Some(domain), actions::GetAllEntities::<data::DataStoreEntity>::new(get_all_entities.show_deleted)))
    }

    pub fn get_all_queries(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        let domain = get_all_entities.domain;
        Ok((Some(domain), actions::GetAllEntities::<data::DataQueryEntity>::new(get_all_entities.show_deleted)))
    }

    pub fn get_all_scripts(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_all_entities: GetAllEntities = from_value(query)?;
        let domain = get_all_entities.domain;
        Ok((Some(domain), actions::GetAllEntities::<data::Script>::new(get_all_entities.show_deleted)))
    }

    pub fn create_table(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::DataStoreEntity = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::CreateEntity::<data::DataStoreEntity>::new(entity)))
    }

    pub fn create_query(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::DataQueryEntity = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::CreateEntity::<data::DataQueryEntity>::new(entity)))
    }

    pub fn create_script(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::Script = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::CreateEntity::<data::Script>::new(entity)))
    }

    pub fn get_table(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::GetEntity::<data::DataStoreEntity>::new(get_entity.name)))
    }

    pub fn get_query(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::GetEntity::<data::DataQueryEntity>::new(get_entity.name)))
    }

    pub fn get_script(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::GetEntity::<data::Script>::new(get_entity.name)))
    }

    pub fn update_table(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::DataStoreEntity = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::UpdateEntity::<data::DataStoreEntity>::new(get_entity.name, entity)))
    }

    pub fn update_query(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::DataQueryEntity = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::UpdateEntity::<data::DataQueryEntity>::new(get_entity.name, entity)))
    }

    pub fn update_script(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let entity: data::Script = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::UpdateEntity::<data::Script>::new(get_entity.name, entity)))
    }

    pub fn delete_table(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::DeleteEntity::<data::DataStoreEntity>::new(get_entity.name)))
    }

    pub fn delete_query(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::DeleteEntity::<data::DataQueryEntity>::new(get_entity.name)))
    }

    pub fn delete_script(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::DeleteEntity::<data::Script>::new(get_entity.name)))
    }

    pub fn query_table_data(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let table_query: Value = data;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::QueryTableData::<_>::new(get_entity.name, table_query)))
    }

    pub fn insert_table_data(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let table_data: Value = data;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::InsertTableData::<_>::new(get_entity.name, table_data)))
    }

    pub fn modify_table_data(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let keyed_data: Value = data;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::ModifyTableData::<_>::new(get_entity.name, keyed_data)))
    }

    pub fn remove_table_data(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let keys: Value = data;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::RemoveTableData::<_>::new(get_entity.name, keys)))
    }

    pub fn run_query(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let params: Value = data;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::RunQuery::<_>::new(get_entity.name, params)))
    }

    pub fn run_script(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let param: data::ScriptParam = from_value(data)?;
        let get_entity: GetEntity = from_value(query)?;
        let domain = get_entity.domain;
        Ok((Some(domain), actions::RunScript::<_>::new(get_entity.name, param)))
    }
}

pub mod pubsub {
    use super::*;

    pub fn subscribe_to(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let channel: data::channels::Channels = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::SubscribeTo::<_>::new(channel)))
    }

    pub fn unsubscribe_from(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let channel: data::channels::Channels = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::UnsubscribeFrom::<_>::new(channel)))
    }

    pub fn unsubscribe_all(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;

        Ok((None, actions::UnsubscribeAll::<_>::new()))
    }

    pub fn get_subscribers(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let channel: data::channels::Channels = from_value(data)?;
        let domain_query: GetFromDomain = from_value(query)?;
        let domain = domain_query.domain;
        Ok((Some(domain), actions::GetSubscribers::<_>::new(channel)))
    }

    pub fn get_messages(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let range: TimeRange = from_value(query)?;

        Ok((None, actions::GetMessages::<_>::new(range.start_time, range.end_time)))

    }
}

pub mod users {
    use super::*;

    pub fn login(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let auth_data: AuthData = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::Login::<_>::new(auth_data.username, auth_data.password)))
    }

    pub fn refresh(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let auth_data: RefreshToken = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::Refresh::<_>::new(auth_data.refresh_token)))
    }

    pub fn logout(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::Logout::<_>::new()))
    }

    pub fn get_all_users(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::GetAllUsers::<_>::new()))
    }

    pub fn add_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let new_user: data::auth::NewUser = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::AddUser::<_>::new(new_user)))
    }

    pub fn remove_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok((None, actions::RemoveUser::<_>::new(get_user.user_identifier)))
    }

    pub fn invite_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let invite: Invite = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::InviteUser::<_>::new(invite.email)))
    }

    pub fn setup_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let new_user: data::auth::NewUser = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::SetupUser::<_>::new(new_user)))
    }

    pub fn set_user_password(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let data: PasswordResetRequest = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::SetUserPassword::<_>::new(data.username, data.new_password))) //TODO: add old password too
    }

    //TODO: modify user

    pub fn add_role(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let role: data::auth::Role = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::AddRole::<_>::new(role)))
    }

    pub fn remove_role(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok((None, actions::RemoveRole::<_>::new(role_id.rolename)))
    }

    pub fn get_all_roles(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let _: NoQuery = from_value(data)?;
        let _: NoQuery = from_value(query)?;
        Ok((None, actions::GetAllRoles::<_>::new()))
    }

    pub fn attach_permission_for_role(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let permission: data::permissions::Permission = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok((None, actions::AttachPermissionForRole::<_>::new(role_id.rolename, permission)))
    }

    pub fn detach_permission_for_role(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let permission: data::permissions::Permission = from_value(data)?;
        let role_id: GetRole = from_value(query)?;
        Ok((None, actions::DetachPermissionForRole::<_>::new(role_id.rolename, permission)))
    }

    pub fn attach_role_for_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let role: RoleData = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok((None, actions::AttachRoleForUser::<_>::new(get_user.user_identifier, role.name)))
    }

    pub fn detach_role_for_user(data: Value, query: Value) -> Result<(Option<String>, impl Action), Error> {
        let role: RoleData = from_value(data)?;
        let get_user: GetUser = from_value(query)?;
        Ok((None, actions::DetachRoleForUser::<_>::new(get_user.user_identifier, role.name)))
    }

}

