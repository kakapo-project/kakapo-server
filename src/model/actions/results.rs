

use actix::prelude::*;

use std::result::Result;
use std::result::Result::Ok;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;
use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::EntityError;

use model::schema;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct GetAllEntitiesResult<T>(pub Vec<T>);

#[derive(Debug)]
pub struct GetEntityResult<T>(pub T);

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum DeleteEntityResult<T> {
    Deleted {
        id: String,
        old: T,
    },
    NotFound(String),
    _PhantomData(PhantomData<T>),
}

#[derive(Debug)]
pub struct GetTableDataResult(pub data::TableData);

#[derive(Debug)]
pub struct InsertTableDataResult(pub data::TableData);

#[derive(Debug)]
pub struct UpdateTableDataResult(pub data::RowData);

#[derive(Debug)]
pub struct DeleteTableDataResult(pub data::RowData);

#[derive(Debug)]
pub struct RunQueryResult(pub data::TableData);

#[derive(Debug)]
pub struct RunScriptResult(pub serde_json::Value);
