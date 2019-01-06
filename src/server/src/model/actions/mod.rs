
pub mod results;
pub mod error;

mod decorator;

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;
use diesel::Connection;

use data;

use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::EntityError;

use model::schema;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use model::entity::conversion;
use model::dbdata::RawEntityTypes;

use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use data::utils::TableDataFormat;

use model::table;
use model::table::TableActionFunctions;
use model::query;
use model::query::QueryActionFunctions;
use model::script;
use model::script::ScriptActionFunctions;

use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;
use model::state::ChannelBroadcaster;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::decorator::*;
use data::KeyData;

pub trait Action<B, S = State<B>>: Send
    where
        B: ChannelBroadcaster + Send + 'static,
        Self::Ret: Send
{
    type Ret;
    fn call(&self, state: &S) -> Result<Self::Ret, Error>;
}

///decorator for permission in listing items
/// Only defined for GetAllEntities
pub struct WithFilterListByPermission<T, B, S = State<B>, ER = entity::Controller>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone + RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    action: GetAllEntities<T, B, S, ER>,
    phantom_data: PhantomData<(T, S, B)>,
}

impl<T, B, S, ER> WithFilterListByPermission<T, B, S, ER>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone + RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection + Send,
{
    pub fn new(action: GetAllEntities<T, B, S, ER>) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<T, B, ER> Action<B, State<B>> for WithFilterListByPermission<T, B, State<B>, ER>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone + RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, State<B>> + Send,
        for<'a> Vec<T>: FromIterator<&'a T>,
{
    type Ret = <GetAllEntities<T, B, State<B>, ER> as Action<B, State<B>>>::Ret;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
        let user_permissions = AuthPermissions::get_permissions(state);
        let raw_results = self.action.call(state)?;

        let GetAllEntitiesResult(inner_results) = raw_results;

        debug!("filtering list based on permissions");
        let filtered_results = inner_results.iter()
            .filter(|x| {
                let required = Permission::read_entity::<T>(x.get_name());
                user_permissions.contains(&required)
            })
            .collect();

        Ok(GetAllEntitiesResult(filtered_results))
    }
}

///get all tables
#[derive(Debug)]
pub struct GetAllEntities<T, B, S = State<B>, ER = entity::Controller>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
{
    pub show_deleted: bool,
    pub phantom_data: PhantomData<(T, B, S, ER)>,
}

impl<T, B, S, ER> GetAllEntities<T, B, S, ER>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(show_deleted: bool) -> Self {
        Self {
            show_deleted,
            phantom_data: PhantomData,
        }
    }
}

impl<T, B, S, ER> Action<B, S> for GetAllEntities<T, B, S, ER>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetAllEntitiesResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        let entities: Vec<T> = ER::get_all(state)
            .or_else(|err| Err(Error::Entity(err)))?;
        Ok(GetAllEntitiesResult::<T>(entities))
    }
}

///get one table
#[derive(Debug)]
pub struct GetEntity<T, B, S = State<B>, ER = entity::Controller>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
{
    pub name: String,
    pub phantom_data: PhantomData<(T, B, S, ER)>,
}

impl<T, B, S, ER> GetEntity<T, B, S, ER>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>, //because WithTransaction isn't fully generic
{
    pub fn new(name: String) -> WithPermissionRequired<WithTransaction<GetEntity<T, B, S, ER>, B, S>, B, S> { //weird syntax but ok
        let action = Self {
            name: name.to_owned(),
            phantom_data: PhantomData,
        };
        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::read_entity::<T>(name));

        action_with_permission
    }
}

impl<T, B, S, ER> Action<B, S> for GetEntity<T, B, S, ER>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetEntityResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        let maybe_entity: Option<T> = ER::get_one(state, &self.name)
            .or_else(|err| Err(Error::Entity(err)))?;

        match maybe_entity {
            Some(entity) => Ok(GetEntityResult::<T>(entity)),
            None => Err(Error::NotFound),
        }
    }
}

///create one table
#[derive(Debug)]
pub struct CreateEntity<T, B, EM = entity::Controller>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, State<B>> + Send,
        State<B>: GetConnection + Send,
{
    pub data: T,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(B, EM)>,
}

impl<T, B, EM> CreateEntity<T, B, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, State<B>> + Send,
        State<B>: GetConnection + Send,
{
    pub fn new(data: T) -> WithTransaction<WithPermissionRequiredOnReturn<Self, B, State<B>>, B, State<B>> {
        let action = Self {
            data,
            on_duplicate: OnDuplicate::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_permission =
            WithPermissionRequiredOnReturn::new(
                action,
                Permission::create_entity::<T>(),
                |result| {
                    match result {
                        CreateEntityResult::Updated { old, .. } => Some(Permission::modify_entity::<T>(old.get_name())),
                        _ => None,
                    }
                });

        let action_with_transaction = WithTransaction::new(action_with_permission);

        action_with_transaction
    }
}

impl<T, B, EM> Action<B, State<B>> for CreateEntity<T, B, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, State<B>> + Send,
        State<B>: GetConnection + Send,
{
    type Ret = CreateEntityResult<T>;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
        match &self.on_duplicate {
            OnDuplicate::Update => {
                EM::upsert(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Upserted::Update { old, new } => Ok(CreateEntityResult::Updated { old, new }),
                            Upserted::Create { new } => Ok(CreateEntityResult::Created(new)),
                        }
                    })
            },
            OnDuplicate::Ignore => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Created::Success { new } => Ok(CreateEntityResult::Created(new)),
                            Created::Fail { existing } => Ok(CreateEntityResult::AlreadyExists { existing, requested: self.data.clone() } ),
                        }
                    })

            },
            OnDuplicate::Fail => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Created::Success { new } => Ok(CreateEntityResult::Created(new)),
                            Created::Fail { .. } => Err(Error::AlreadyExists),
                        }
                    })
            },
        }
    }
}

///update table
#[derive(Debug)]
pub struct UpdateEntity<T, B, S = State<B>, EM = entity::Controller>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    pub name: String,
    pub data: T,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(B, S, EM)>,
}

impl<T, B, S, EM> UpdateEntity<T, B, S, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>, //because WithTransaction isn't fully generic
{
    pub fn new(name: String, data: T) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            name: name.to_owned(),
            data,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, B, S, EM> Action<B, S> for UpdateEntity<T, B, S, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = UpdateEntityResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                EM::update(state, (&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Updated::Success { old, new } =>
                                Ok(UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail =>
                                Ok(UpdateEntityResult::NotFound { id: self.name.to_owned(), requested: self.data.clone() }),
                        }
                    })

            },
            OnNotFound::Fail => {
                EM::update(state, (&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Updated::Success { old, new } =>
                                Ok(UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

///delete table
#[derive(Debug)]
pub struct DeleteEntity<T, B, S = State<B>, EM = entity::Controller>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    pub name: String,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(T, B, S, EM)>,
}

impl<T, B, S, EM> DeleteEntity<T, B, S, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>, //because WithTransaction isn't fully generic
{
    pub fn new(name: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            name: name.to_owned(),
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, B, S, EM> Action<B, S> for DeleteEntity<T, B, S, EM>
    where
        B: ChannelBroadcaster + Send + 'static,
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = DeleteEntityResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                EM::delete(state, &self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Deleted::Success { old } =>
                                Ok(DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => Ok(DeleteEntityResult::NotFound(self.name.to_owned())),
                        }
                    })

            },
            OnNotFound::Fail => {
                EM::delete(state, &self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Deleted::Success { old } =>
                                Ok(DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

// Table Actions
#[derive(Debug)]
pub struct QueryTableData<B, S = State<B>, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(B, S, ER, TC)>,
}

impl<B, S, ER, TC> QueryTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(table_name: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            table_name: table_name.to_owned(),
            format: TableDataFormat::Rows,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::get_table_data(table_name));

        action_with_permission
    }
}

impl<B, S, ER, TC> Action<B, S> for QueryTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetTableDataResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                TC::query(state, &table)
                    .or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                match &self.format {
                    TableDataFormat::Rows => Ok(table_data.into_rows_data()),
                    TableDataFormat::FlatRows => Ok(table_data.into_rows_flat_data()),
                }
            })
            .and_then(|res| Ok(GetTableDataResult(res)))
    }
}


#[derive(Debug)]
pub struct InsertTableData<B, S = State<B>, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub data: data::TableData, //payload
    pub format: TableDataFormat,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(B, S, ER, TC)>,
}

impl<B, S, ER, TC> InsertTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(table_name: String, data: data::TableData) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            table_name: table_name.to_owned(),
            data,
            format: TableDataFormat::Rows,
            on_duplicate: OnDuplicate::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<B, S, ER, TC> Action<B, S> for InsertTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,

{
    type Ret = InsertTableDataResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                match &self.on_duplicate {
                    OnDuplicate::Update => TC::upsert_row(state, &table, &self.data),
                    OnDuplicate::Ignore => TC::insert_row(state, &table, &self.data, false),
                    OnDuplicate::Fail => TC::insert_row(state, &table, &self.data, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                match &self.format {
                    TableDataFormat::Rows => Ok(table_data.into_rows_data()),
                    TableDataFormat::FlatRows => Ok(table_data.into_rows_flat_data()),
                }
            })
            .and_then(|res| Ok(InsertTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct UpdateTableData<B, S = State<B>, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub keys: KeyData,
    pub data: data::TableData, //payload
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(B, S, ER, TC)>,
}

impl<B, S, ER, TC> UpdateTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(table_name: String, keys: KeyData, data: data::TableData) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            table_name: table_name.to_owned(),
            keys,
            data,
            format: TableDataFormat::Rows,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<B, S, ER, TC> Action<B, S> for UpdateTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = UpdateTableDataResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                match &self.on_not_found {
                    OnNotFound::Ignore => TC::update_row(state, &table, &self.keys, &self.data,false),
                    OnNotFound::Fail => TC::update_row(state, &table, &self.keys, &self.data,true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                match &self.format {
                    TableDataFormat::Rows => Ok(table_data.into_rows_data()),
                    TableDataFormat::FlatRows => Ok(table_data.into_rows_flat_data()),
                }
            })
            .and_then(|res| Ok(UpdateTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct DeleteTableData<B, S = State<B>, ER = entity::Controller, TC = table::TableAction>  {
    pub table_name: String,
    pub keys: KeyData,
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(B, S, ER, TC)>,
}

impl<B, S, ER, TC> DeleteTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(table_name: String, keys: KeyData) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            table_name: table_name.to_owned(),
            keys,
            format: TableDataFormat::Rows,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<B, S, ER, TC> Action<B, S> for DeleteTableData<B, S, ER, TC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = DeleteTableDataResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                match &self.on_not_found {
                    OnNotFound::Ignore => TC::delete_row(state, &table, &self.keys, false),
                    OnNotFound::Fail => TC::delete_row(state, &table, &self.keys, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                match &self.format {
                    TableDataFormat::Rows => Ok(table_data.into_rows_data()),
                    TableDataFormat::FlatRows => Ok(table_data.into_rows_flat_data()),
                }
            })
            .and_then(|res| Ok(DeleteTableDataResult(res)))
    }
}

// Query Action
#[derive(Debug)]
pub struct RunQuery<B, S = State<B>, ER = entity::Controller, QC = query::QueryAction>  {
    pub query_name: String,
    pub params: String, //TODO: not a string
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(B, S, ER, QC)>,
}

impl<B, S, ER, QC> RunQuery<B, S, ER, QC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Query, S> + Send,
        QC: query::QueryActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(query_name: String, params: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            query_name: query_name.to_owned(),
            params,
            format: TableDataFormat::Rows,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_query(query_name));

        action_with_permission
    }
}

impl<B, S, ER, QC> Action<B, S> for RunQuery<B, S, ER, QC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Query, S> + Send,
        QC: query::QueryActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = RunQueryResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.query_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Query>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                QC::run_query()
                    .or_else(|err| Err(Error::Query(err)))
            })
            .and_then(|table_data| {
                match &self.format {
                    TableDataFormat::Rows => Ok(table_data.into_rows_data()),
                    TableDataFormat::FlatRows => Ok(table_data.into_rows_flat_data()),
                }
            })
            .and_then(|res| Ok(RunQueryResult(res)))
    }
}

// Query Action
#[derive(Debug)]
pub struct RunScript<B, S = State<B>, ER = entity::Controller, SC = script::ScriptAction>  {
    pub script_name: String,
    pub params: String, //TODO: not a string
    pub phantom_data: PhantomData<(B, S, ER, SC)>,
}

impl<B, S, ER, SC> RunScript<B, S, ER, SC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(script_name: String, params: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            script_name: script_name.to_owned(),
            params,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_script(script_name));

        action_with_permission
    }
}

impl<B, S, ER, SC> Action<B, S> for RunScript<B, S, ER, SC>
    where
        B: ChannelBroadcaster + Send + 'static,
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = RunScriptResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.script_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Script>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                SC::run_script()
                    .or_else(|err| Err(Error::Script(err)))
            })
            .and_then(|res| Ok(RunScriptResult(res)))
    }
}

/// User Auth: Add user
pub struct AddUser<B, S = State<B>> {
    username: String,
    email: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> AddUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(username: String, email: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            username,
            email,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for AddUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// User Auth: Remove User
pub struct RemoveUser<B, S = State<B>> {
    userid: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> RemoveUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(userid: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            userid,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for RemoveUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}


/// User Auth: Get All users
pub struct GetAllUsers<B, S = State<B>> {
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> GetAllUsers<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new() -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for GetAllUsers<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// User Auth: Set user password
pub struct SetUserPassword<B, S = State<B>> {
    userid: String,
    password: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> SetUserPassword<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(userid: String, password: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            userid,
            password,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for SetUserPassword<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}


/// User Auth: Email user for invitation
pub struct InviteUser<B, S = State<B>> {
    email: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> InviteUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(email: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            email,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for InviteUser<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// Role Auth: Add Role
pub struct AddRole<B, S = State<B>> {
    rolename: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> AddRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(rolename: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            rolename,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for AddRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// Role Auth: Remove role
pub struct RemoveRole<B, S = State<B>> {
    rolename: String,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> RemoveRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(rolename: String) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            rolename,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for RemoveRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// Role Auth: get all role
pub struct GetAllRoles<B, S = State<B>> {
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> GetAllRoles<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new() -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for GetAllRoles<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// Role Auth: add permission
pub struct AttachPermissionForRole<B, S = State<B>> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> AttachPermissionForRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(rolename: String, permission: Permission) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            rolename,
            permission,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for AttachPermissionForRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

/// Role Auth: remove permission
pub struct DetachPermissionForRole<B, S = State<B>> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(B, S)>,
}

impl<B, S> DetachPermissionForRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        WithTransaction<Self, B, S>: Action<B, S>,
{
    pub fn new(rolename: String, permission: Permission) -> WithPermissionRequired<WithTransaction<Self, B, S>, B, S> {
        let action = Self {
            rolename,
            permission,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::add_user());

        action_with_permission
    }
}

impl<B, S> Action<B, S> for DetachPermissionForRole<B, S>
    where
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = ();
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

//Other utitlies
#[derive(Debug)]
pub struct Nothing;

impl Nothing {
    pub fn new() -> Self {
        Nothing
    }
}

impl<B> Action<B> for Nothing
    where
        B: ChannelBroadcaster + Send + 'static,
{
    type Ret = ();
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
        Ok(())
    }
}
