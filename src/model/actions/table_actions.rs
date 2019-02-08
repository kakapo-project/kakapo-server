
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use model::entity;
use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::utils::TableDataFormat;

use model::table;
use model::state::ActionState;
use model::state::GetConnection;
use model::state::Channels;
use model::auth::permissions::*;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::auth::permissions::GetUserInfo;
use model::state::GetBroadcaster;
use model::state::StateFunctions;


// Table Actions
#[derive(Debug)]
pub struct QueryTableData<S = ActionState, ER = entity::Controller, TC = table::TableAction> {
    pub table_name: String,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S, ER, TC)>,
}

impl<S, ER, TC> QueryTableData<S, ER, TC>
    where
        ER: entity::RetrieverFunctions<data::Table, S>,
        TC: table::TableActionFunctions<S>,
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
            .and_then(|res| ActionRes::new("QueryTableData", GetTableDataResult(res)))
    }
}


#[derive(Debug)]
pub struct InsertTableData<S = ActionState, ER = entity::Controller, TC = table::TableAction> {
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
            .and_then(|res| ActionRes::new("InsertTableData", InsertTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct ModifyTableData<S = ActionState, ER = entity::Controller, TC = table::TableAction> {
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + GetBroadcaster + StateFunctions<'a>,
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
            .and_then(|res| ActionRes::new("ModifyTableData", ModifyTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct RemoveTableData<S = ActionState, ER = entity::Controller, TC = table::TableAction>  {
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + StateFunctions<'a>,
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
        for<'a> S: GetConnection + GetUserInfo + GetBroadcaster + StateFunctions<'a>,
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
            .and_then(|res| ActionRes::new("RemoveTableData", RemoveTableDataResult(res)))
    }
}
