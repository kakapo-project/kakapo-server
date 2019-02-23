
use std::marker::PhantomData;

use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::Named;
use data::channels::Channels;
use data::permissions::*;

use model::actions::results::*;
use model::actions::error::Error;
use model::actions::decorator::*;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;

use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::RawEntityTypes;
use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use model::entity::update_state::UpdateActionFunctions;

use state::StateFunctions;
use state::ActionState;
use state::authorization::AuthorizationOps;

///decorator for permission in listing items
/// Only defined for GetAllEntities
#[derive(Debug, Clone)]
pub struct WithFilterListByPermission<A, T, S = ActionState>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    action: A,
    phantom_data: PhantomData<(T, S)>,
}

impl<A, T, S> WithFilterListByPermission<A, T, S>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, T, S> Action<S> for WithFilterListByPermission<A, T, S>
    where
        A: Action<S, Ret = GetAllEntitiesResult<T>>,
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = <GetAllEntities<T, S> as Action<S>>::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let user_permissions = state
            .get_authorization()
            .permissions()
            .unwrap_or_default();
        let raw_results = self.action.call(state)?;
        let raw_results_name = raw_results.get_name();

        let GetAllEntitiesResult(inner_results) = raw_results.get_data();

        debug!("filtering list based on permissions");
        let filtered_results = inner_results.into_iter()
            .filter(|x| {
                let required = Permission::read_entity::<T>(x.my_name().to_owned());
                user_permissions.contains(&required)
            })
            .collect();

        ActionRes::new(&raw_results_name, GetAllEntitiesResult(filtered_results))
    }
}

///get all tables
#[derive(Debug, Clone)]
pub struct GetAllEntities<T, S = ActionState>
    where
        T: RawEntityTypes,
{
    pub show_deleted: bool,
    pub phantom_data: PhantomData<(T, S)>,
}

impl<T, S> GetAllEntities<T, S>
    where
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(show_deleted: bool) -> WithFilterListByPermission<WithTransaction<Self, S>, T, S> {
        let action = Self {
            show_deleted,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_filter = WithFilterListByPermission::new(action_with_transaction);

        action_with_filter
    }
}

impl<T, S> Action<S> for GetAllEntities<T, S>
    where
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = GetAllEntitiesResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let entities: Vec<T> = state
            .get_entity_retreiver_functions()
            .get_all()
            .or_else(|err| Err(Error::Entity(err)))?;
        ActionRes::new("GetAllEntities", GetAllEntitiesResult::<T>(entities))
    }
}

///get one table
#[derive(Debug, Clone)]
pub struct GetEntity<T, S = ActionState>
    where
        T: RawEntityTypes,
{
    pub name: String,
    pub phantom_data: PhantomData<(T, S)>,
}

impl<T, S> GetEntity<T, S>
    where
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(name: String) -> WithPermissionRequired<WithTransaction<GetEntity<T, S>, S>, S> { //weird syntax but ok
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

impl<T, S> Action<S> for GetEntity<T, S>
    where
        T: RawEntityTypes,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = GetEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let maybe_entity: Option<T> = state
            .get_entity_retreiver_functions()
            .get_one(&self.name)
            .or_else(|err| Err(Error::Entity(err)))?;

        match maybe_entity {
            Some(entity) => ActionRes::new("GetEntity", GetEntityResult::<T>(entity)),
            None => Err(Error::NotFound),
        }
    }
}

///create one table
#[derive(Debug, Clone)]
pub struct CreateEntity<T, S = ActionState>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    pub data: T,
    pub on_duplicate: OnDuplicate,
    pub phantom_data: PhantomData<(S)>,
}

impl<T, S> CreateEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
        <Self as Action<S>>::Ret: Clone,
{
    pub fn new(data: T) -> WithPermissionFor<WithDispatch<WithTransaction<Self, S>, S>, S> {

        let name = data.my_name().to_owned();
        let channel = Channels::entity::<T>(&name);

        let create_permission = Permission::create_entity::<T>();
        let update_permission = Permission::modify_entity::<T>(name);
        let on_duplicate = OnDuplicate::Ignore; //TODO:...

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

impl<T, S> Action<S> for CreateEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = CreateEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_duplicate {
            OnDuplicate::Update => {
                state
                    .get_entity_modifier_function()
                    .upsert(self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("upsert result: {:?}", &res);
                        match res {
                            Upserted::Update { old, new } => ActionRes::new("CreateEntity", CreateEntityResult::Updated { old, new }),
                            Upserted::Create { new } => ActionRes::new("CreateEntity", CreateEntityResult::Created { new }),
                        }
                    })
            },
            OnDuplicate::Ignore => {
                state
                    .get_entity_modifier_function()
                    .create(self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("create result: {:?}", &res);
                        match res {
                            Created::Success { new } => ActionRes::new("CreateEntity", CreateEntityResult::Created { new }),
                            Created::Fail { existing } => ActionRes::new("CreateEntity", CreateEntityResult::AlreadyExists { existing, requested: self.data.clone() } ),
                        }
                    })

            },
            OnDuplicate::Fail => {
                state
                    .get_entity_modifier_function()
                    .create(self.data.clone())
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("create result: {:?}", &res);
                        match res {
                            Created::Success { new } => ActionRes::new("CreateEntity", CreateEntityResult::Created { new }),
                            Created::Fail { .. } => Err(Error::AlreadyExists),
                        }
                    })
            },
        }
    }
}

///update table
#[derive(Debug, Clone)]
pub struct UpdateEntity<T, S = ActionState>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    pub name: String,
    pub data: T,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(S)>,
}

impl<T, S> UpdateEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(name: String, data: T) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channel = Channels::entity::<T>(&name);
        let action = Self {
            name: name.to_owned(),
            data,
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, S> Action<S> for UpdateEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = UpdateEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                state
                    .get_entity_modifier_function()
                    .update((&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("update result: {:?}", &res);
                        match res {
                            Updated::Success { old, new } =>
                                ActionRes::new("UpdateEntity", UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail =>
                                ActionRes::new("UpdateEntity", UpdateEntityResult::NotFound { id: self.name.to_owned(), requested: self.data.clone() }),
                        }
                    })

            },
            OnNotFound::Fail => {
                state
                    .get_entity_modifier_function()
                    .update((&self.name, self.data.clone()))
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("update result: {:?}", &res);
                        match res {
                            Updated::Success { old, new } =>
                                ActionRes::new("UpdateEntity", UpdateEntityResult::Updated { id: self.name.to_owned(), old, new }),
                            Updated::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

///delete table
#[derive(Debug, Clone)]
pub struct DeleteEntity<T, S = ActionState>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    pub name: String,
    pub on_not_found: OnNotFound,
    pub phantom_data: PhantomData<(T, S)>,
}

impl<T, S> DeleteEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(name: String) -> WithPermissionRequired<WithDispatch<WithTransaction<Self, S>, S>, S> {
        let channel = Channels::entity::<T>(&name);
        let action = Self {
            name: name.to_owned(),
            on_not_found: OnNotFound::Ignore,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_dispatch = WithDispatch::new(action_with_transaction, channel);
        let action_with_permission =
            WithPermissionRequired::new(action_with_dispatch, Permission::modify_entity::<T>(name));

        action_with_permission
    }
}

impl<T, S> Action<S> for DeleteEntity<T, S>
    where
        T: RawEntityTypes + UpdateActionFunctions,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = DeleteEntityResult<T>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        match &self.on_not_found {
            OnNotFound::Ignore => {
                state
                    .get_entity_modifier_function()
                    .delete(&self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("delete result: {:?}", &res);
                        match res {
                            Deleted::Success { old } =>
                                ActionRes::new("DeleteEntity", DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => ActionRes::new("DeleteEntity", DeleteEntityResult::NotFound { id: self.name.to_owned() }),
                        }
                    })

            },
            OnNotFound::Fail => {
                state
                    .get_entity_modifier_function()
                    .delete(&self.name)
                    .or_else(|err| Err(Error::Entity(err)))
                    .and_then(|res| {
                        info!("delete result: {:?}", &res);
                        match res {
                            Deleted::Success { old } =>
                                ActionRes::new("DeleteEntity", DeleteEntityResult::Deleted { id: self.name.to_owned(), old } ),
                            Deleted::Fail => Err(Error::NotFound),
                        }
                    })
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::from_value;
    use data;
    use model::actions::results::CreateEntityResult::Created;
    use model::actions::results::DeleteEntityResult::Deleted;
    use test_common::random_identifier;
    use test_common::with_state;
    use test_common::MockState;

    #[test]
    fn test_create_entity() {

        with_state(|state| {
            let name = format!("my_query_{}", random_identifier());
            let new_query: data::Query = from_value(json!({
                "name": name,
                "description": "blah blah blah",
                "statement": "SELECT * FROM a_table"
            })).unwrap();
            let create_action = CreateEntity::<data::Query, MockState>::new(new_query);

            let result = create_action.call(&state);
            let data = result.unwrap().get_data();
            println!("data: {:?}", &data);

            if let Created { new } = data {
                assert_eq!(new.my_name(), name);
                assert_eq!(new.description, "blah blah blah");
                assert_eq!(new.statement, "SELECT * FROM a_table");
            } else {
                panic!("expected a created result");
            }
        });
    }

    #[test]
    fn test_update_entity() {
        with_state(|state| {
            let name = format!("my_query_{}", random_identifier());
            let new_query: data::Query = from_value(json!({
                "name": name,
                "description": "blah blah blah",
                "statement": "SELECT * FROM a_table"
            })).unwrap();
            let create_action = CreateEntity::<data::Query, MockState>::new(new_query);

            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            let read_action = GetEntity::<data::Query, MockState>::new(name.to_owned());
            let result = read_action.call(&state);

            let data = result.unwrap().get_data();
            let GetEntityResult(entity_result) = data;
            assert_eq!(entity_result.my_name(), name);
            assert_eq!(entity_result.description, "blah blah blah");
            assert_eq!(entity_result.statement, "SELECT * FROM a_table");
        });
    }

    #[test]
    fn test_delete_entity() {
        with_state(|state| {
            let name = format!("my_query_{}", random_identifier());
            let new_query: data::Query = from_value(json!({
                "name": name,
                "description": "blah blah blah",
                "statement": "SELECT * FROM a_table"
            })).unwrap();
            let create_action = CreateEntity::<data::Query, MockState>::new(new_query);

            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            let delete_action = DeleteEntity::<data::Query, MockState>::new(name.to_owned());
            let result = delete_action.call(&state);
            let data = result.unwrap().get_data();

            if let Deleted { id, old } = data {
                assert_eq!(id, name);
                assert_eq!(old.my_name(), name);
                assert_eq!(old.description, "blah blah blah");
                assert_eq!(old.statement, "SELECT * FROM a_table");
            } else {
                panic!("expected a deleted result");
            }
        });
    }
}