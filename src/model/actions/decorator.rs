
use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

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
use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::Action;
use model::actions::ActionResult;
use std::collections::HashSet;
use model::auth::permissions::GetUserInfo;
use model::auth::auth_store::AuthStore;
use model::auth::auth_store::AuthStoreFunctions;


#[derive(Debug)]
enum Requirements {
    AllOf(Vec<Permission>),
    AnyOf(Vec<Permission>),
}

impl Requirements {
    fn is_permitted(&self, user_permissions: &HashSet<Permission>) -> bool {
        let mut is_permitted = true;
        match self {
            Requirements::AllOf(required_permissions) => {
                is_permitted = true;
                for required_permission in required_permissions {
                    if !user_permissions.contains(required_permission) {
                        is_permitted = false;
                    }
                }
            },
            Requirements::AnyOf(required_permissions) => {
                is_permitted = false;
                for required_permission in required_permissions {
                    if user_permissions.contains(required_permission) {
                        is_permitted = true;
                    }
                }
            }
        };

        is_permitted
    }
}

///decorator for permission
pub struct WithPermissionRequired<A, S = State>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    action: A,
    permissions: Requirements,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> WithPermissionRequired<A, S>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    pub fn new(action: A, permission: Permission) -> Self {
        Self {
            action,
            permissions: Requirements::AnyOf(vec![permission]),
            phantom_data: PhantomData,
        }
    }

    pub fn new_any_of(action: A, permissions: Vec<Permission>) -> Self {
        Self {
            action,
            permissions: Requirements::AnyOf(permissions),
            phantom_data: PhantomData,
        }
    }

    pub fn new_all_of(action: A, permissions: Vec<Permission>) -> Self {
        Self {
            action,
            permissions: Requirements::AllOf(permissions),
            phantom_data: PhantomData,
        }
    }
}

impl<A, S> Action<S> for WithPermissionRequired<A, S>
    where
        A: Action<S>,
        AuthStore: AuthStoreFunctions<S>,
        S: GetConnection + GetUserInfo,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if S::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = S::get_permissions::<AuthStore>(state).unwrap_or_default();
        let is_permitted = self.permissions.is_permitted(&user_permissions);

        if is_permitted {
            self.action.call(state)
        } else {
            debug!("Permission denied, required permission: {:?}", &self.permissions);
            Err(Error::Unauthorized)
        }

    }
}

///decorator for login
pub struct WithLoginRequired<A, S = State>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    action: A,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> WithLoginRequired<A, S>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, S> Action<S> for WithLoginRequired<A, S>
    where
        A: Action<S>,
        AuthStore: AuthStoreFunctions<S>,
        S: GetConnection + GetUserInfo,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if S::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = S::get_permissions::<AuthStore>(state);
        match user_permissions {
            None => {
                debug!("Permission denied, required login");
                Err(Error::Unauthorized)
            },
            Some(_) => self.action.call(state)
        }
    }
}

///decorator for permission after the value is returned
/// Warning: this should always be wrapped in a transaction decorator, otherwise, you will modify the state
pub struct WithPermissionFor<A, S = State>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    action: A,
    required_permission: Box<Fn(&HashSet<Permission>, &HashSet<Permission>) -> bool + Send>,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> WithPermissionFor<A, S>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
{
    pub fn new<F>(action: A, required_permission: F) -> Self
        where
            F: Fn(&HashSet<Permission>, &HashSet<Permission>) -> bool + Send + 'static,
    {
        Self {
            action,
            required_permission: Box::new(required_permission),
            phantom_data: PhantomData,
        }
    }
}

impl<A, S> Action<S> for WithPermissionFor<A, S>
    where
        A: Action<S>,
        S: GetConnection + GetUserInfo,
        AuthStore: AuthStoreFunctions<S>,
        <A as Action<S>>::Ret : Clone,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if S::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = S::get_permissions::<AuthStore>(state).unwrap_or_default();
        let all_permissions = S::get_all_permissions::<AuthStore>(state);

        let is_permitted = (self.required_permission)(&user_permissions, &all_permissions);

        if is_permitted {
            self.action.call(state)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

///decorator for transactions
#[derive(Debug, Clone)]
pub struct WithTransaction<A, S = State>
    where
        A: Action<S>,
        S: GetConnection,
{
    action: A,
    phantom_data: PhantomData<S>,
}

impl<A, S> WithTransaction<A, S>
    where
        A: Action<S>,
        S: GetConnection,
        Self: Action<S>,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl From<diesel::result::Error> for Error {
    // as far as I can tell, this function will only run if the transaction fails, which wouldn't
    // normally return any specific error, it will return the inner error
    // this is needed for the transaction part below
    fn from(diesel_error: diesel::result::Error) -> Self {
        Error::Unknown
    }
}

impl<A, S> Action<S> for WithTransaction<A, S>
    where
        A: Action<S>,
        S: GetConnection,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        debug!("started transaction");

        state.transaction::<Self::Ret, Error, _>(||
            self.action.call(state)
        )

    }
}

///decorator for dispatching to channel
pub struct WithDispatch<A, S = State>
    where
        A: Action<S>,
{
    action: A,
    channels: Vec<Channels>,
    phantom_data: PhantomData<S>,
}

impl<A, S> WithDispatch<A, S>
    where
        A: Action<S>,
        S: GetConnection,
{
    pub fn new(action: A, channel: Channels) -> Self {
        Self {
            action,
            channels: vec![channel],
            phantom_data: PhantomData,
        }
    }

    pub fn new_multi(action: A, channels: Vec<Channels>) -> Self {
        Self {
            action,
            channels,
            phantom_data: PhantomData,
        }
    }
}

impl<A, S> Action<S> for WithDispatch<A, S>
    where
        A: Action<S>,
        S: GetConnection,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        debug!("dispatching action");

        let mut result = self.action.call(state)?;

        unimplemented!(); //need to send to broadcaster

        Ok(result)
    }
}
