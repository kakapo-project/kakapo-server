
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
use model::state::ChannelBroadcaster;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::Action;

///decorator for permission
pub struct WithPermissionRequired<A, B, S = State<B>, AU = AuthPermissions>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    action: A,
    permission: Permission,
    phantom_data: PhantomData<(S, B, AU)>,
}

impl<A, B, S, AU> WithPermissionRequired<A, B, S, AU>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    pub fn new(action: A, permission: Permission) -> Self {
        Self {
            action,
            permission,
            phantom_data: PhantomData,
        }
    }
}

impl<A, B, AU> Action<B, State<B>> for WithPermissionRequired<A, B, State<B>, AU>
    where
        A: Action<B, State<B>>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    type Ret = A::Ret;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
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
pub struct WithLoginRequired<A, B, S = State<B>, AU = AuthPermissions>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    action: A,
    phantom_data: PhantomData<(S, B, AU)>,
}

impl<A, B, S, AU> WithLoginRequired<A, B, S, AU>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, B, AU> Action<B, State<B>> for WithLoginRequired<A, B, State<B>, AU>
    where
        A: Action<B, State<B>>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    type Ret = A::Ret;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
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
pub struct WithPermissionRequiredOnReturn<A, B, S = State<B>, AU = AuthPermissions>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    action: A,
    initial_permission: Permission,
    required_permission: Box<Fn(&A::Ret) -> Option<Permission> + Send>,
    phantom_data: PhantomData<(S, B, AU)>,
}

impl<A, B, S, AU> WithPermissionRequiredOnReturn<A, B, S, AU>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        Self: Action<B, S>,
        S: GetConnection + Send,
        AU: AuthPermissionFunctions<B> + Send,
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

impl<A, B, AU> Action<B, State<B>> for WithPermissionRequiredOnReturn<A, B, State<B>, AU>
    where
        A: Action<B, State<B>>,
        B: ChannelBroadcaster + Send + 'static,
        AU: AuthPermissionFunctions<B> + Send,
{
    type Ret = A::Ret;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
        if AU::is_admin(state) {
            return self.action.call(state);
        }

        let user_permissions = AU::get_permissions(state).unwrap_or_default();
        if user_permissions.contains(&self.initial_permission) {
            let result = self.action.call(state)?;
            match (self.required_permission)(&result) {
                None => Ok(result),
                Some(next_permission) => if user_permissions.contains(&self.initial_permission) {
                    Ok(result)
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
pub struct WithTransaction<A, B, S = State<B>>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    action: A,
    phantom_data: PhantomData<(S, B)>,
}

impl<A, B, S> WithTransaction<A, B, S>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
        Self: Action<B, S>,
{
    pub fn new(action: A) -> Self {
        Self {
            action,
            phantom_data: PhantomData,
        }
    }
}

impl<A, B> Action<B, State<B>> for WithTransaction<A, B, State<B>>
    where
        A: Action<B, State<B>>,
        B: ChannelBroadcaster + Send + 'static,
{
    type Ret = A::Ret;
    fn call(&self, state: &State<B>) -> Result<Self::Ret, Error> {
        debug!("started transaction");
        let conn = state.get_conn();
        conn.transaction::<Self::Ret, Error, _>(||
            self.action.call(state)
        )

    }
}

///decorator for dispatching to channel
pub struct WithDispatch<A, B, S = State<B>>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
{
    action: A,
    channel: Channels,
    phantom_data: PhantomData<(S, B)>,
}

impl<A, B, S> WithDispatch<A, B, S>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    pub fn new(action: A, channel: Channels) -> Self {
        Self {
            action,
            channel,
            phantom_data: PhantomData,
        }
    }
}

impl<A, B, S> Action<B, S> for WithDispatch<A, B, S>
    where
        A: Action<B, S>,
        B: ChannelBroadcaster + Send + 'static,
        S: GetConnection + Send,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> Result<Self::Ret, Error> {
        debug!("dispatching action");

        let result = self.action.call(state)?;
        B::on_broadcast(&self.channel, &result);

        Ok(result)
    }
}
