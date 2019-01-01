
pub mod results;
pub mod error;
pub mod channels;

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;

use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::DBError;

use model::schema;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use model::entity::conversion;
use model::dbdata::RawEntityTypes;

use model::entity::Retriever;
use model::entity::Modifier;

use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use data::utils::TableDataFormat;
use model::table;
use model::table::TableActionFunctions;
use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;


pub trait Action<S = State>: Send
    where
        Self::Ret: Send
{
    type Ret;
    fn call(&self, state: &S) -> Result<Self::Ret, Error>;
}

///decorator for permission
pub struct WithPermissionRequired<A: Action<S>, S = State> {
    action: A,
    phantom_data: PhantomData<S>,
    //permission: ...
}

impl<A: Action<S>, S> WithPermissionRequired<A, S> {
    pub fn new(action: A/*, permission: Permission*/) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
            //permission,
        }
    }
}

impl<A: Action<S>, S> Action<S> for WithPermissionRequired<A, S>
    where
        S: GetConnection + Send,
{
    type Ret = A::Ret; //TODO: 403 error
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        self.action.call(state)
    }
}

///decorator for permission in listing items
pub struct WithFilterListByPermission<A: Action<S>, S = State>
    where
        S: GetConnection + Send,
{
    action: A,
    phantom_data: PhantomData<S>,
    //permission: ...
}

///decorator for transactions
pub struct WithTransaction<A: Action<S>, S = State>
    where
        S: GetConnection + Send,
{
    action: A,
    phantom_data: PhantomData<S>,
    //permission: ...
}

impl<A: Action<S>, S> WithTransaction<A, S>
    where
        S: GetConnection + Send,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A: Action<S>, S> Action<S> for WithTransaction<A, S>
    where
        S: GetConnection + Send,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        //TODO: transactions
        self.action.call(state)
    }
}

///decorator for dispatching to channel
pub struct WithDispatch<A: Action<S>, S = State> {
    action: A,
    phantom_data: PhantomData<S>,
    //permission: ...
}

///get all tables
#[derive(Debug)]
pub struct GetAllEntities<T, S = State, ER = entity::Retriever>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
{
    pub show_deleted: bool,
    pub phantom_data: PhantomData<(T, S, ER)>,
}

impl<T, S, ER> GetAllEntities<T, S, ER>
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

impl<T, S, ER> Action<S> for GetAllEntities<T, S, ER>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetAllEntitiesResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        let entities: Vec<T> = ER::get_all(state)
            .or_else(|err| Err(Error::DB(err)))?;
        Ok(GetAllEntitiesResult::<T>(entities))
    }
}

///get one table
#[derive(Debug)]
pub struct GetEntity<T, S = State, ER = entity::Retriever>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
{
    pub name: String,
    pub phantom_data: PhantomData<(T, S, ER)>,
}

impl<T, S, ER> GetEntity<T, S, ER>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    pub fn new(name: String) -> WithPermissionRequired<WithTransaction<GetEntity<T, S, ER>, S>, S> { //weird syntax but ok
        let action = Self {
            name,
            phantom_data: PhantomData,
        };
        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithPermissionRequired::new(action_with_transaction /*, ... */);

        action_with_permission
    }
}

impl<T, S, ER> Action<S> for GetEntity<T, S, ER>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::ConvertRaw<<T as RawEntityTypes>::Data>,
        ER: RetrieverFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetEntityResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        let maybe_entity: Option<T> = ER::get_one(state, &self.name)
            .or_else(|err| Err(Error::DB(err)))?;

        match maybe_entity {
            Some(entity) => Ok(GetEntityResult::<T>(entity)),
            None => Err(Error::NotFound),
        }
    }
}

///create one table
#[derive(Debug)]
pub struct CreateEntity<T, S = State, EM = entity::Modifier>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub data: T,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S, EM)>,
}

impl<T, S> CreateEntity<T, S>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            on_duplicate: OnDuplicate::Ignore,
            phantom_data: PhantomData,
        }
    }
}

impl<T, S, EM> Action<S> for CreateEntity<T, S, EM>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        EM: ModifierFunctions<T, S> + Send,
        S: GetConnection + Send,
{
    type Ret = CreateEntityResult<T>;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        match &self.on_duplicate {
            OnDuplicate::Update => {
                EM::upsert(state, self.data.clone())
                    .or_else(|err| Err(Error::DB(err)))
                    .and_then(|res| {
                        match res {
                            Upserted::Update { old, new } => Ok(CreateEntityResult::Updated { old, new }),
                            Upserted::Create { new } => Ok(CreateEntityResult::Created(new)),
                        }
                    })
            },
            OnDuplicate::Ignore => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::DB(err)))
                    .and_then(|res| {
                        match res {
                            Created::Success { new } => Ok(CreateEntityResult::Created(new)),
                            Created::Fail { existing } => Ok(CreateEntityResult::AlreadyExists { existing, requested: self.data.clone() } ),
                        }
                    })

            },
            OnDuplicate::Fail => {
                EM::create(state, self.data.clone())
                    .or_else(|err| Err(Error::DB(err)))
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
pub struct UpdateEntity<T, S = State, EM = entity::Modifier>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub name: String,
    pub data: T,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S, EM)>,
}

impl<T, S, EM> UpdateEntity<T, S, EM>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub fn new(name: String, data: T) -> Self {
        Self {
            name,
            data,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        }
    }
}

impl<T, S, EM> Action<S> for UpdateEntity<T, S, EM>
    where
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
                    .or_else(|err| Err(Error::DB(err)))
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
                    .or_else(|err| Err(Error::DB(err)))
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
pub struct DeleteEntity<T, S = State, EM = entity::Modifier>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub name: String,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(T, S, EM)>,
}

impl<T, S, EM> DeleteEntity<T, S, EM>
    where
        T: Send + Clone,
        T: RawEntityTypes,
        T: conversion::GenerateRaw<<T as RawEntityTypes>::NewData>,
        S: GetConnection + Send,
{
    pub fn new(name: String) -> Self {
        Self {
            name,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        }
    }
}

impl<T, S, EM> Action<S> for DeleteEntity<T, S, EM>
    where
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
                    .or_else(|err| Err(Error::DB(err)))
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
                    .or_else(|err| Err(Error::DB(err)))
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
pub struct QueryTableData<S = State, ER = entity::Retriever, TC = table::TableAction> {
    pub table_name: String,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> QueryTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            format: TableDataFormat::Rows,
            phantom_data: PhantomData,
        }
    }
}

impl<S, ER, TC> Action<S> for QueryTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S> + Send,
        TC: table::TableActionFunctions<S> + Send,
        S: GetConnection + Send,
{
    type Ret = GetTableDataResult;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        ER::get_one(state, &self.table_name)
            .or_else(|err| Err(Error::DB(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let query_result = TC::query(state, table)
                    .or_else(|err| Err(Error::TableQuery(err)))?;
                Ok(query_result)
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
pub struct InsertTableData {
    pub name: String,
    //pub data: api::TableData, //payload
    //pub format: api::TableDataFormat,
    //pub on_duplicate: api::OnDuplicate,
}

impl InsertTableData {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl Action for InsertTableData {
    type Ret = InsertTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::insert_table_data(
        //    &state,
        //    self.name.to_owned(), self.data.to_owned(), self.format.to_owned(), api::CreationMethod::default())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(Error::Unknown)
    }
}

#[derive(Debug)]
pub struct UpdateTableData {
    pub name: String,
    //pub key: String,
    //pub data: api::RowData, //payload
    //pub format: api::TableDataFormat,
}

impl UpdateTableData {
    pub fn new(name: String) -> impl Action<Ret = UpdateTableDataResult> {
        Self {
            name
        }
    }
}

impl Action for UpdateTableData {
    type Ret = UpdateTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::update_table_data(
        //    &state,
        //    self.name.to_owned(), self.key.to_owned(), self.data.to_owned(), self.format.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(Error::Unknown)
    }
}

#[derive(Debug)]
pub struct DeleteTableData {
    pub name: String,
    //pub key: String,
}

impl DeleteTableData {
    pub fn new(name: String) -> impl Action<Ret = DeleteTableDataResult> {
        Self {
            name
        }
    }
}

impl Action for DeleteTableData {
    type Ret = DeleteTableDataResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = table::delete_table_data(&state, self.name.to_owned(), self.key.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(Error::Unknown)
    }
}

// Query Action
#[derive(Debug)]
pub struct RunQuery {
    pub name: String,
    //pub params: api::QueryParams,
    //pub start: Option<usize>,
    //pub end: Option<usize>,
    //pub format: api::TableDataFormat,
}

impl RunQuery {
    pub fn new(name: String) -> impl Action<Ret = RunQueryResult> {
        Self {
            name
        }
    }
}

impl Action for RunQuery {
    type Ret = RunQueryResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = query::run_query(
        //    &state,
        //    self.name.to_owned(), self.format.to_owned(), self.params.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(Error::Unknown)
    }
}

// Query Action
#[derive(Debug)]
pub struct RunScript {
    pub name: String,
    //pub params: api::ScriptParam,
}

impl RunScript {
    pub fn new(name: String) -> impl Action<Ret = RunScriptResult> {
        Self {
            name
        }
    }
}

impl Action for RunScript {
    type Ret = RunScriptResult;
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        //let result = script::run_script(
        //    &state,
        //    self.py_runner.to_owned(), self.name.to_owned(), self.params.to_owned())
        //    .or_else(|err| Err(()))?;
        //Ok(result)
        Err(Error::Unknown)
    }
}

//Auth
pub struct AddUser;
pub struct RemoveUser;

pub struct AddRole;
pub struct RemoveRole;

pub struct AttachPermissionForRole;
pub struct DetachPermissionForRole;

//Other utitlies
#[derive(Debug)]
pub struct Nothing;

impl Nothing {
    pub fn new() -> impl Action<Ret = ()> {
        Nothing
    }
}

impl Action for Nothing {
    type Ret = ();
    fn call(&self, state: &State) -> Result<Self::Ret, Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestEntityRetriever;

    struct TestState(TestConn);
    struct TestConn;

    impl GetConnection for TestState {
        type Connection = TestConn;
        fn get_conn<'a>(&'a self) -> &'a TestConn {
            &self.0
        }
    }

    impl RetrieverFunctions<data::Table, TestState> for TestEntityRetriever {
        fn get_all(conn: &TestState) -> Result<Vec<data::Table>, DBError> {
            unimplemented!()
        }

        fn get_one(conn: &TestState, name: &str) -> Result<Option<data::Table>, DBError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_get_all_entities_for_table() {
        let state = TestState(TestConn);

        let action = GetAllEntities::<data::Table, TestState, TestEntityRetriever>::new(true);
        let action_result = action.call(&state);
    }


}