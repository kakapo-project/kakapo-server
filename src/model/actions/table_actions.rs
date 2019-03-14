
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;

use data::channels::Channels;
use data::permissions::Permission;

use model::actions::decorator::*;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;

use model::entity::RetrieverFunctions;
use model::table::DatastoreActionOps;

use state::ActionState;
use state::StateFunctions;

// Table Actions
#[derive(Debug)]
pub struct QueryTableData<S = ActionState> {
    pub table_name: String,
    pub format: serde_json::Value,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> QueryTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(table_name: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            table_name: table_name.to_owned(),
            format: json!({}), //TODO:...
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
        debug!("Calling QueryTableData");

        state
            .get_entity_retreiver_functions()
            .get_one( &self.table_name)
            .map_err(|err| Error::Entity(err))
            .and_then(|res: Option<data::DataStoreEntity>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                state
                    .get_table_controller()
                    .query(&table)
                    .map_err(|err| Error::Datastore(err))
            })
            .and_then(|res| ActionRes::new("QueryTableData", GetTableDataResult(res)))
    }
}


#[derive(Debug)]
pub struct InsertTableData<S = ActionState> {
    pub table_name: String,
    pub data: serde_json::Value, //payload
    pub format: serde_json::Value,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> InsertTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(table_name: String, data: serde_json::Value) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channel = Channels::table(&table_name);
        let action = Self {
            table_name: table_name.to_owned(),
            data,
            format: json!({}), //TODO:...
            on_duplicate: OnDuplicate::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
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
        debug!("Calling InsertTableData");

        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::DataStoreEntity>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let table_controller = state.get_table_controller();
                match &self.on_duplicate {
                    OnDuplicate::Update => table_controller.upsert_row(&table, &self.data),
                    OnDuplicate::Ignore => table_controller.insert_row(&table, &self.data, false),
                    OnDuplicate::Fail => table_controller.insert_row(&table, &self.data, true)
                }.or_else(|err| Err(Error::Datastore(err)))
            })
            .and_then(|res| ActionRes::new("InsertTableData", InsertTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct ModifyTableData<S = ActionState> {
    pub table_name: String,
    pub keyed_data: serde_json::Value,
    pub format: serde_json::Value,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> ModifyTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(table_name: String, keyed_data: serde_json::Value) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channel = Channels::table(&table_name);
        let action = Self {
            table_name: table_name.to_owned(),
            keyed_data,
            format: json!({}), //TODO:...
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
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
        debug!("Calling ModifyTableData");

        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::DataStoreEntity>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let table_controller = state.get_table_controller();
                match &self.on_not_found {
                    OnNotFound::Ignore => table_controller.update_row(&table, &self.keyed_data, false),
                    OnNotFound::Fail => table_controller.update_row(&table, &self.keyed_data, true)
                }.or_else(|err| Err(Error::Datastore(err)))
            })
            .and_then(|res| ActionRes::new("ModifyTableData", ModifyTableDataResult(res)))
    }
}

#[derive(Debug)]
pub struct RemoveTableData<S = ActionState>  {
    pub table_name: String,
    pub keys: serde_json::Value,
    pub format: serde_json::Value,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> RemoveTableData<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(table_name: String, keys: serde_json::Value) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channel = Channels::table(&table_name);
        let action = Self {
            table_name: table_name.to_owned(),
            keys,
            format: json!({}), //TODO:...
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
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
        debug!("Calling RemoveTableData");

        state
            .get_entity_retreiver_functions()
            .get_one(&self.table_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::DataStoreEntity>| {
                match res {
                    Some(table) => Ok(table),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|table| {
                let table_controller = state.get_table_controller();
                match &self.on_not_found {
                    OnNotFound::Ignore => table_controller.delete_row(&table, &self.keys, false),
                    OnNotFound::Fail => table_controller.delete_row(&table, &self.keys, true)
                }.or_else(|err| Err(Error::Datastore(err)))
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
            let table: data::DataStoreEntity = from_value(json!({
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

            let create_action = entity_actions::CreateEntity::<data::DataStoreEntity, MockState>::new(table);
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            let data = json!([
                {
                    "col_a": 42,
                    "col_b": 43,
                },
                {
                    "col_a": 5000,
                    "col_b": 5500,
                }
            ]);
            let create_action = InsertTableData::<MockState>::new(table_name, data);
            let result = create_action.call(&state);

            println!("result: {:?}", &result);
        });
    }
}