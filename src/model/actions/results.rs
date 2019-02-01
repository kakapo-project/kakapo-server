

use actix::prelude::*;

use data;
use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::EntityError;

use data::schema;
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize)]
pub struct GetAllEntitiesResult<T>(pub Vec<T>);

#[derive(Debug, Clone, Serialize)]
pub struct GetEntityResult<T>(pub T);

#[derive(Debug, Clone, Serialize)]
pub enum CreateEntityResult<T> {
    Updated {
        old: T,
        new: T,
    },
    Created(T),
    AlreadyExists {
        existing: T,
        requested: T,
    },
}

#[derive(Debug, Clone, Serialize)]
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
pub enum DeleteEntityResult<T> {
    Deleted {
        id: String,
        old: T,
    },
    NotFound(String),
    _PhantomData(PhantomData<T>),
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