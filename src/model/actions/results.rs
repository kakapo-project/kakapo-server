

use data;
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize)]
pub struct GetAllEntitiesResult<T>(pub Vec<T>);

#[derive(Debug, Clone, Serialize)]
pub struct GetEntityResult<T>(pub T);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum CreateEntityResult<T> {
    Updated {
        old: T,
        new: T,
    },
    Created {
        new: T,
    },
    AlreadyExists {
        existing: T,
        requested: T,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum UpdateEntityResult<T> {
    Updated {
        id: String,
        old: T,
        new: T,
    },
    NotFound {
        id: String,
        requested: T,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum DeleteEntityResult<T> {
    Deleted {
        id: String,
        old: T,
    },
    NotFound {
        id: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTableDataResult(pub data::TableData);

#[derive(Debug, Clone, Serialize)]
pub struct InsertTableDataResult(pub data::TableData);

#[derive(Debug, Clone, Serialize)]
pub struct ModifyTableDataResult(pub data::TableData);

#[derive(Debug, Clone, Serialize)]
pub struct RemoveTableDataResult(pub data::TableData);

#[derive(Debug, Clone, Serialize)]
pub struct RunQueryResult(pub data::TableData);

#[derive(Debug, Clone, Serialize)]
pub struct RunScriptResult(pub serde_json::Value);


#[derive(Debug, Clone, Serialize)]
pub struct UserResult(pub data::auth::User);

#[derive(Debug, Clone, Serialize)]
pub struct AllUsersResult(pub Vec<data::auth::User>);

#[derive(Debug, Clone, Serialize)]
pub struct InvitationToken(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct RoleResult(pub data::auth::Role);

#[derive(Debug, Clone, Serialize)]
pub struct AllRolesResult(pub Vec<data::auth::Role>);