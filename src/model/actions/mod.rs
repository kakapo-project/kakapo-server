
pub mod results;
pub mod error;
pub mod users;

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

use data::schema;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::conversion;
use data::dbdata::RawEntityTypes;

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
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::decorator::*;
use std::fmt::Debug;

use model::auth::Auth;
use model::auth::AuthFunctions;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ActionOk<R>
    where
        R: Send,
{
    data: R,
    channels: Vec<Channels>,
}

impl<R> ActionOk<R>
    where R: Send
{
    pub fn get_data(self) -> R {
        self.data
    }
}

pub type ActionResult<R> = Result<ActionOk<R>, Error>;

pub struct ActionRes;
impl ActionRes {
    pub fn new<R>(result: R) -> ActionResult<R>
        where R: Send
    {
        let ok_result = ActionOk {
            data: result,
            channels: vec![],
        };

        Ok(ok_result)
    }
}

pub trait Action<S = State>
    where
        Self: Send,
        Self::Ret: Send + Debug + Serialize,
{
    type Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret>;
}

///decorator for permission in listing items
/// Only defined for GetAllEntities
pub struct WithFilterListByPermission<A, T, S = State, ER = entity::Controller>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    action: A,
    phantom_data: PhantomData<(T, S, ER)>,
}

impl<A, T, S, ER> WithFilterListByPermission<A, T, S, ER>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, T, S, ER> Action<S> for WithFilterListByPermission<A, T, S, ER>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    type Ret = <GetAllEntities<T, S, ER> as Action<S>>::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let user_permissions = AuthPermissions::get_permissions(state).unwrap_or_default();
        let raw_results = self.action.call(state)?;

        let GetAllEntitiesResult(inner_results) = raw_results.data;

        debug!("filtering list based on permissions");
        let filtered_results = inner_results.into_iter()
            .filter(|x| {
                let required = Permission::read_entity::<T>(x.get_name());
                user_permissions.contains(&required)
            })
            .collect();

        Ok(ActionOk {
            data: GetAllEntitiesResult(filtered_results),
            channels: raw_results.channels,
        })
    }
}

///get all tables
#[derive(Debug, Clone)]
pub struct GetAllEntities<T, S = State, ER = entity::Controller>
    where
        T: RawEntityTypes,
{
    pub show_deleted: bool,
    pub phantom_data: PhantomData<(T, S, ER)>,
}

impl<T, S, ER> GetAllEntities<T, S, ER>
    where
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(show_deleted: bool) -> WithFilterListByPermission<WithTransaction<Self, S>, T, S, ER> {
        let action = Self {
            show_deleted,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_filter = WithFilterListByPermission::new(action_with_transaction);

        action_with_filter
    }
}

impl<T, S, ER> Action<S> for GetAllEntities<T, S, ER>
    where
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection,
{
    type Ret = GetAllEntitiesResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let entities: Vec<T> = ER::get_all(state)
            .or_else(|err| Err(Error::Entity(err)))?;
        ActionRes::new(GetAllEntitiesResult::<T>(entities))
    }
}

///get one table
#[derive(Debug, Clone)]
pub struct GetEntity<T, S = State, ER = entity::Controller>
    where
        T: RawEntityTypes,
{
    pub name: String,
    pub phantom_data: PhantomData<(T, S, ER)>,
}

impl<T, S, ER> GetEntity<T, S, ER>
    where
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(name: String) -> WithPermissionRequired<WithTransaction<GetEntity<T, S, ER>, S>, S> { //weird syntax but ok
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

impl<T, S, ER> Action<S> for GetEntity<T, S, ER>
    where
        T: RawEntityTypes,
        ER: RetrieverFunctions<T, S>,
        S: GetConnection,
{
    type Ret = GetEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let maybe_entity: Option<T> = ER::get_one(state, &self.name)
            .or_else(|err| Err(Error::Entity(err)))?;

        match maybe_entity {
            Some(entity) => ActionRes::new(GetEntityResult::<T>(entity)),
            None => Err(Error::NotFound),
        }
    }
}

///create one table
#[derive(Debug, Clone)]
pub struct CreateEntity<T, S = State, EM = entity::Controller>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    pub data: T,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S, EM)>,
}

impl<T, S, EM> CreateEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
        ActionOk<<Self as Action<S>>::Ret>: Clone,
{
    pub fn new(data: T) -> WithPermissionFor<WithDispatch<WithTransaction<Self, S>, S>, S> {

        let name = data.get_name();
        let create_permission = Permission::create_entity::<T>();
        let update_permission = Permission::modify_entity::<T>(name);
        let on_duplicate = OnDuplicate::Ignore; //TODO:...
        let channel = Channels::all_entities::<T>(); //TODO: on update this should have table as well

        let action = Self {
            data,
            on_duplicate: OnDuplicate::Ignore,  //TODO:...
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
        let action_with_permission =
            WithPermissionFor::new(
                action_with_dispatch,
                move |user_permissions, all_permissions| {
                    match on_duplicate {
                        OnDuplicate::Update => if all_permissions.contains(&update_permission) {
                            user_permissions.contains(&update_permission)
                        } else {
                            user_permissions.contains(&create_permission)
                        },
                        _ => user_permissions.contains(&create_permission),
                    }
                });



        action_with_permission
    }
}

impl<T, S, EM> Action<S> for CreateEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    type Ret = CreateEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_duplicate {
            OnDuplicate::Update => {
                EM::upsert(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Upserted::Update { old, new } => ActionRes::new(CreateEntityResult::Updated { old, new }),
                            Upserted::Create { new } => ActionRes::new(CreateEntityResult::Created(new)),
                        }
                    })
            },
            OnDuplicate::Ignore => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Created::Success { new } => ActionRes::new(CreateEntityResult::Created(new)),
                            Created::Fail { existing } => ActionRes::new(CreateEntityResult::AlreadyExists { existing, requested: self.data.clone() } ),
                        }
                    })

            },
            OnDuplicate::Fail => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Created::Success { new } => ActionRes::new(CreateEntityResult::Created(new)),
                            Created::Fail { .. } => Err(Error::AlreadyExists),
                        }
                    })
            },
        }
    }
}

///update table
#[derive(Debug)]
pub struct UpdateEntity<T, S = State, EM = entity::Controller>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    pub name: String,
    pub data: T,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S, EM)>,
}

impl<T, S, EM> UpdateEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(name: String, data: T) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channels = vec![
            Channels::all_entities::<T>(),
            Channels::entity::<T>(&name),
        ];
        let action = Self {
            name: name.to_owned(),
            data,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new_multi(action_with_transaction, channels);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, S, EM> Action<S> for UpdateEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    type Ret = UpdateEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                EM::update(state, (&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Updated::Success { old, new } =>
                                ActionRes::new(UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail =>
                                ActionRes::new(UpdateEntityResult::NotFound { id: self.name.to_owned(), requested: self.data.clone() }),
                        }
                    })

            },
            OnNotFound::Fail => {
                EM::update(state, (&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Updated::Success { old, new } =>
                                ActionRes::new(UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

///delete table
#[derive(Debug)]
pub struct DeleteEntity<T, S = State, EM = entity::Controller>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    pub name: String,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(T, S, EM)>,
}

impl<T, S, EM> DeleteEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    pub fn new(name: String) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channels = vec![
            Channels::all_entities::<T>(),
            Channels::entity::<T>(&name),
        ];
        let action = Self {
            name: name.to_owned(),
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new_multi(action_with_transaction, channels);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, S, EM> Action<S> for DeleteEntity<T, S, EM>
    where
        T: RawEntityTypes,
        EM: ModifierFunctions<T, S>,
        S: GetConnection,
{
    type Ret = DeleteEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                EM::delete(state, &self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Deleted::Success { old } =>
                                ActionRes::new(DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => ActionRes::new(DeleteEntityResult::NotFound(self.name.to_owned())),
                        }
                    })

            },
            OnNotFound::Fail => {
                EM::delete(state, &self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        match res {
                            Deleted::Success { old } =>
                                ActionRes::new(DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

// Table Actions
#[derive(Debug)]
pub struct QueryTableData<S = State, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> QueryTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    pub fn new(table_name: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
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

impl<S, ER, TC> Action<S> for QueryTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    type Ret = GetTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
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
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new(GetTableDataResult(res)))
    }
}


#[derive(Debug)]
pub struct InsertTableData<S = State, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub data: data::TableData, //payload
    pub format: TableDataFormat,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> InsertTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    pub fn new(table_name: String, data: data::TableData) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channels = vec![Channels::table(&table_name)];
        let action = Self {
            table_name: table_name.to_owned(),
            data,
            format: TableDataFormat::Rows,
            on_duplicate: OnDuplicate::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new_multi(action_with_transaction, channels);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<S, ER, TC> Action<S> for InsertTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    type Ret = InsertTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let data = self.data.normalize();
                match &self.on_duplicate {
                    OnDuplicate::Update => TC::upsert_row(state, &table, &data),
                    OnDuplicate::Ignore => TC::insert_row(state, &table, &data, false),
                    OnDuplicate::Fail => TC::insert_row(state, &table, &data, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new(InsertTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct ModifyTableData<S = State, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub keyed_data: data::KeyedTableData,
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> ModifyTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    pub fn new(table_name: String, keyed_data: data::KeyedTableData) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channels = vec![Channels::table(&table_name)];
        let action = Self {
            table_name: table_name.to_owned(),
            keyed_data,
            format: TableDataFormat::Rows,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new_multi(action_with_transaction, channels);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<S, ER, TC> Action<S> for ModifyTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    type Ret = ModifyTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let (keys, data) = self.keyed_data.normalize();
                match &self.on_not_found {
                    OnNotFound::Ignore => TC::update_row(state, &table, &keys, &data,false),
                    OnNotFound::Fail => TC::update_row(state, &table, &keys, &data,true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new(ModifyTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct RemoveTableData<S = State, ER = entity::Controller, TC = table::TableAction>  {
    pub table_name: String,
    pub keys: data::KeyData,
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> RemoveTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    pub fn new(table_name: String, keys: data::KeyData) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channels = vec![Channels::table(&table_name)];
        let action = Self {
            table_name: table_name.to_owned(),
            keys,
            format: TableDataFormat::Rows,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new_multi(action_with_transaction, channels);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_table_data(table_name));

        action_with_permission
    }
}

impl<S, ER, TC> Action<S> for RemoveTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        S: GetConnection,
{
    type Ret = RemoveTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let keys = self.keys.normalize();
                match &self.on_not_found {
                    OnNotFound::Ignore => TC::delete_row(state, &table, &keys, false),
                    OnNotFound::Fail => TC::delete_row(state, &table, &keys, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new(RemoveTableDataResult(res)))
    }
}

// Query Action
#[derive(Debug)]
pub struct RunQuery<S = State, ER = entity::Controller, QC = query::QueryAction>  {
    pub query_name: String,
    pub params: data::QueryParams,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S, ER, QC)>,
}

impl<S, ER, QC> RunQuery<S, ER, QC>
    where
        ER: entity::RetrieverFunctions<data::Query, S>,
        QC: query::QueryActionFunctions<S>,
        S: GetConnection,
{
    pub fn new(query_name: String, params: data::QueryParams) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
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

impl<S, ER, QC> Action<S> for RunQuery<S, ER, QC>
    where
        ER: entity::RetrieverFunctions<data::Query, S> + Send,
        QC: query::QueryActionFunctions<S> + Send,
        S: GetConnection,
{
    type Ret = RunQueryResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.query_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Query>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|query| {
                QC::run_query(state, &query, &self.params)
                    .or_else(|err| Err(Error::Query(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new(RunQueryResult(res)))
    }
}

// Query Action
#[derive(Debug)]
pub struct RunScript<S = State, ER = entity::Controller, SC = script::ScriptAction>  {
    pub script_name: String,
    pub param: data::ScriptParam,
    pub phantom_data: PhantomData<(S, ER, SC)>,
}

impl<S, ER, SC> RunScript<S, ER, SC>
    where
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection,
{
    pub fn new(script_name: String, param: data::ScriptParam) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            script_name: script_name.to_owned(),
            param,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_script(script_name));

        action_with_permission
    }
}

impl<S, ER, SC> Action<S> for RunScript<S, ER, SC>
    where
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection,
{
    type Ret = RunScriptResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.script_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Script>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|script| {
                SC::run_script(state, &script)
                    .or_else(|err| Err(Error::Script(err)))
            })
            .and_then(|res| ActionRes::new(RunScriptResult(res)))
    }
}
