
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::utils::TableDataFormat;

use model::state::ActionState;
use data::channels::Channels;
use data::permissions::Permission;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::state::StateFunctions;
use model::entity::RetrieverFunctions;
use model::table::TableActionFunctions;

// Table Actions
#[derive(Debug)]
pub struct QueryTableData<S = ActionState> {
    pub table_name: String,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> QueryTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
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

impl<S> Action<S> for QueryTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = GetTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one( &self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                state
                    .get_table_controller()
                    .query(&table)
                    .or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new("QueryTableData", GetTableDataResult(res)))
    }
}


#[derive(Debug)]
pub struct InsertTableData<S = ActionState> {
    pub table_name: String,
    pub data: data::TableData, //payload
    pub format: TableDataFormat,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> InsertTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
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

impl<S> Action<S> for InsertTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = InsertTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let data = self.data.normalize();
                let table_controller = state.get_table_controller();
                match &self.on_duplicate {
                    OnDuplicate::Update => table_controller.upsert_row(&table, &data),
                    OnDuplicate::Ignore => table_controller.insert_row(&table, &data, false),
                    OnDuplicate::Fail => table_controller.insert_row(&table, &data, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new("InsertTableData", InsertTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct ModifyTableData<S = ActionState> {
    pub table_name: String,
    pub keyed_data: data::KeyedTableData,
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> ModifyTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
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

impl<S> Action<S> for ModifyTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = ModifyTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let (keys, data) = self.keyed_data.normalize();
                let table_controller = state.get_table_controller();
                match &self.on_not_found {
                    OnNotFound::Ignore => table_controller.update_row(&table, &keys, &data, false),
                    OnNotFound::Fail => table_controller.update_row(&table, &keys, &data, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new("ModifyTableData", ModifyTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct RemoveTableData<S = ActionState>  {
    pub table_name: String,
    pub keys: data::KeyData,
    pub format: TableDataFormat,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> RemoveTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
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

impl<S> Action<S> for RemoveTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = RemoveTableDataResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Table>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let keys = self.keys.normalize();
                let table_controller = state.get_table_controller();
                match &self.on_not_found {
                    OnNotFound::Ignore => table_controller.delete_row(&table, &keys, false),
                    OnNotFound::Fail => table_controller.delete_row(&table, &keys, true)
                }.or_else(|err| Err(Error::Table(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new("RemoveTableData", RemoveTableDataResult(res)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_common::random_identifier;
    use serde_json::from_value;
    use test_common::*;
    use model::actions::entity_actions;

    #[test]
    fn test_add_data() {
        with_state(|state| {
            let table_name = format!("my_table{}", random_identifier());
            let table: data::Table = from_value(json!({
                "name": table_name,
                "description": "table description",
                "schema": {
                    "columns": [
                        {
                            "name": "col_a",
                            "dataType": "integer"
                        },
                        {
                            "name": "col_b",
                            "dataType": "integer"
                        }
                    ],
                    "constraint": [
                    ]
                }
            })).unwrap();

            let create_action = entity_actions::CreateEntity::<data::Table, MockState>::new(table);
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            let data: data::TableData = from_value(json!([
                {
                    "col_a": 42,
                    "col_b": 43,
                },
                {
                    "col_a": 5000,
                    "col_b": 5500,
                }
            ])).unwrap();
            let create_action = InsertTableData::<MockState>::new(table_name, data);
            let result = create_action.call(&state);

            println!("result: {:?}", &result);
        });
    }
}