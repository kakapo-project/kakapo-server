
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
use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::Action;
use model::actions::ActionResult;
use model::actions::ActionOk;

///decorator for permission
pub struct WithPermissionRequired<A, S = State, AU = AuthPermissions>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    action: A,
    permission: Permission,
    phantom_data: PhantomData<(S, AU)>,
}

impl<A, S, AU> WithPermissionRequired<A, S, AU>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    pub fn new(action: A, permission: Permission) -> Self {
        Self {
            action,
            permission,
            phantom_data: PhantomData,
        }
    }
}

impl<A, S, AU> Action<S> for WithPermissionRequired<A, S, AU>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if AU::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = AU::get_permissions(state).unwrap_or_default();
        if user_permissions.contains(&self.permission) {
            self.action.call(state)
        } else {
            debug!("Permission denied, required permission: {:?}", &self.permission);
            Err(Error::Unauthorized)
        }

    }
}

///decorator for login
pub struct WithLoginRequired<A, S = State, AU = AuthPermissions>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    action: A,
    phantom_data: PhantomData<(S, AU)>,
}

impl<A, S, AU> WithLoginRequired<A, S, AU>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, S, AU> Action<S> for WithLoginRequired<A, S, AU>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if AU::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = AU::get_permissions(state);
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
pub struct WithPermissionRequiredOnReturn<A, S = State, AU = AuthPermissions>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    action: A,
    initial_permission: Permission,
    required_permission: Box<Fn(&A::Ret) -> Option<Permission> + Send>,
    phantom_data: PhantomData<(S, AU)>,
}

impl<A, S, AU> WithPermissionRequiredOnReturn<A, S, AU>
    where
        A: Action<S>,
        Self: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
{
    pub fn new<F>(action: A, permission: Permission, required_permission: F) -> Self
        where
            F: Send + Fn(&A::Ret) -> Option<Permission> + 'static,
    {
        Self {
            action,
            initial_permission: permission,
            required_permission: Box::new(required_permission),
            phantom_data: PhantomData,
        }
    }
}

impl<A, S, AU> Action<S> for WithPermissionRequiredOnReturn<A, S, AU>
    where
        A: Action<S>,
        S: GetConnection,
        AU: AuthPermissionFunctions<S>,
        ActionOk<<A as Action<S>>::Ret> : Clone,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if AU::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = AU::get_permissions(state).unwrap_or_default();
        if user_permissions.contains(&self.initial_permission) {
            let action_result = self.action.call(state)?;
            let result = &action_result.data;
            match (self.required_permission)(result) {
                None => Ok(action_result.clone()),
                Some(next_permission) => if user_permissions.contains(&self.initial_permission) {
                    Ok(action_result.clone())
                } else {
                    Err(Error::Unauthorized)
                }
            }
        } else {
            debug!("Permission denied, required permission: {:?}", &self.initial_permission);
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

impl<A, S> Action<S> for WithTransaction<A, S>
    where
        A: Action<S>,
        S: GetConnection,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        debug!("started transaction");

        state.transaction::<ActionOk<Self::Ret>, Error, _>(||
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
    pub fn new(action: A, channels: Vec<Channels>) -> Self {
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
        result.channels = self.channels.to_owned();

        Ok(result)
    }
}
