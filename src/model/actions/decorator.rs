
use std::result::Result::Ok;
use std::marker::PhantomData;
use std::fmt;
use std::collections::HashSet;

use data::channels::Channels;
use data::permissions::*;

use model::actions::error::Error;
use model::actions::Action;
use model::actions::ActionResult;
use model::actions::OkAction;

use state::StateFunctions;
use state::authorization::AuthorizationOps;
use state::PubSubOps;
use state::ActionState;

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct WithPermissionRequired<A, S = ActionState>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    action: A,
    permissions: Requirements,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> WithPermissionRequired<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
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
        for<'a> S: StateFunctions<'a>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if state.get_authorization().is_admin() {
            return self.action.call(state);
        }

        let user_permissions = state
            .get_authorization()
            .permissions()
            .unwrap_or_default();
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
#[derive(Debug, Clone)]
pub struct WithLoginRequired<A, S = ActionState>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    action: A,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> WithLoginRequired<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
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
        for<'a> S: StateFunctions<'a>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if state.get_authorization().is_admin() {
            return self.action.call(state);
        }

        let user_permissions = state
            .get_authorization()
            .permissions();
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
pub struct WithPermissionFor<A, S = ActionState>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    action: A,
    required_permission: Box<Fn(&HashSet<Permission>, &HashSet<Permission>) -> bool + Send>,
    phantom_data: PhantomData<(S)>,
}

impl<A, S> fmt::Debug for WithPermissionFor<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WithPermissionFor({:?})", &self.action)
    }
}

impl<A, S> WithPermissionFor<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
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
        for<'a> S: StateFunctions<'a>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        if state.get_authorization().is_admin() {
            return self.action.call(state);
        }

        let user_permissions = state
            .get_authorization()
            .permissions()
            .unwrap_or_default();

        let all_permissions = state
            .get_authorization()
            .all_permissions();

        let is_permitted = (self.required_permission)(&user_permissions, &all_permissions);

        if is_permitted {
            self.action.call(state)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

///decorator for transactions
#[derive(Clone)]
pub struct WithTransaction<A, S = ActionState>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    action: A,
    phantom_data: PhantomData<S>,
}

impl<A, S> fmt::Debug for WithTransaction<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WithTransaction({:?})", &self.action)
    }
}

impl<A, S> WithTransaction<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
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
        warn!("diesel_error: {:?}", &diesel_error);
        Error::Unknown
    }
}

impl<A, S> Action<S> for WithTransaction<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        debug!("started transaction");

        state.transaction::<OkAction<Self::Ret>, Error, _>(||
            self.action.call(state)
        )

    }
}

///decorator for dispatching to channel
#[derive(Clone)]
pub struct WithDispatch<A, S = ActionState>
    where
        A: Action<S>,
{
    action: A,
    channel: Channels,
    phantom_data: PhantomData<S>,
}

impl<A, S> fmt::Debug for WithDispatch<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WithDispatch({:?})", &self.action)
    }
}

impl<A, S> WithDispatch<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(action: A, channel: Channels) -> Self {
        Self {
            action,
            channel,
            phantom_data: PhantomData,
        }
    }
}

impl<A, S> Action<S> for WithDispatch<A, S>
    where
        A: Action<S>,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = A::Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        debug!("dispatching action");

        let result = self.action.call(state)?;

        let data_ref = serde_json::to_value(result.get_data_ref().clone())
            .map_err(|err| Error::SerializationError(err.to_string()))?;

        state
            .get_pub_sub()
            .publish(
                self.channel.to_owned(),
                result.get_name(),
                &data_ref)
            .map_err(Error::PublishError)?;

        Ok(result)
    }
}
