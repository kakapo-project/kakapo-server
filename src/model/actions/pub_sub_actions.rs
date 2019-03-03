
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;
use model::query;

use data::permissions::*;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::entity::RetrieverFunctions;
use data::channels::Channels;
use data::channels::Defaults;
use data::channels::Sub;

use state::PubSubOps;
use state::ActionState;
use state::StateFunctions;
use state::authorization::AuthorizationOps;

#[derive(Debug)]
pub struct SubscribeTo<S = ActionState>  {
    pub channel: Channels,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> SubscribeTo<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(channel: Channels) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let permission = channel.required_permission();
        let action = Self {
            channel,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, permission);

        action_with_permission
    }
}

impl<S> Action<S> for SubscribeTo<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = SubscriptionResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {

        let user_id = state
            .get_authorization()
            .user_id()
            .ok_or_else(|| Error::Unauthorized)?;

        state
            .get_pub_sub()
            .subscribe(user_id, self.channel.to_owned())
            .map_err(|err| Error::PublishError(err))
            .and_then(|res| ActionRes::new("UnsubscribeFrom", SubscriptionResult::Subscribed(res)))
    }
}

#[derive(Debug)]
pub struct UnsubscribeFrom<S = ActionState>  {
    pub channel: Channels,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> UnsubscribeFrom<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    pub fn new(channel: Channels) -> WithLoginRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            channel,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithLoginRequired::new(action_with_transaction);

        action_with_permission
    }
}

impl<S> Action<S> for UnsubscribeFrom<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    type Ret = SubscriptionResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {

        let user_id = state
            .get_authorization()
            .user_id()
            .ok_or_else(|| Error::Unauthorized)?;

        state
            .get_pub_sub()
            .unsubscribe(user_id, self.channel.to_owned())
            .map_err(|err| Error::PublishError(err))
            .and_then(|res| ActionRes::new("UnsubscribeFrom", SubscriptionResult::Unsubscribed(res)))
    }
}

#[derive(Debug)]
pub struct GetSubscribers<S = ActionState>  {
    pub channel: Channels,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> GetSubscribers<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    pub fn new(channel: Channels) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let permission = channel.required_permission();
        let action = Self {
            channel,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, permission);

        action_with_permission
    }
}

impl<S> Action<S> for GetSubscribers<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    type Ret = Vec<data::auth::User>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_pub_sub()
            .get_subscribers(self.channel.to_owned())
            .map_err(|err| Error::PublishError(err))
            .and_then(|res| ActionRes::new("GetAllSubscribers", res))
    }
}

#[derive(Debug)]
pub struct GetMessages<S = ActionState>  {
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> GetMessages<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    pub fn new(start_time: chrono::NaiveDateTime, end_time: chrono::NaiveDateTime) -> WithLoginRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            start_time,
            end_time,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission = WithLoginRequired::new(action_with_transaction);

        action_with_permission
    }
}

impl<S> Action<S> for GetMessages<S>
    where
            for<'a> S: StateFunctions<'a>,
{
    type Ret = Vec<data::Message>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {

        let user_id = state
            .get_authorization()
            .user_id()
            .ok_or_else(|| Error::Unauthorized)?;

        state
            .get_pub_sub()
            .get_messages(user_id, self.start_time.to_owned(), self.end_time.to_owned())
            .map_err(|err| Error::PublishError(err))
            .and_then(|res| ActionRes::new("GetMessages", res))
    }
}

impl Channels {
    fn required_permission(&self) -> Permission {
        match self {
            Channels::Defaults(Defaults::Table(name)) => Permission::read_entity::<data::DataStoreEntity>(name.to_owned()), //TODO: not right, this should be the responsiblity of raw types
            Channels::Defaults(Defaults::Query(name)) => Permission::read_entity::<data::DataQueryEntity>(name.to_owned()),
            Channels::Defaults(Defaults::Script(name)) => Permission::read_entity::<data::Script>(name.to_owned()),
            Channels::Defaults(Defaults::View(name)) => Permission::read_entity::<data::View>(name.to_owned()),
            Channels::Defaults(Defaults::TableData(name)) => Permission::get_table_data(name.to_owned()),
            Channels::Subscribers(Sub::Subscribers(channel)) => Channels::Defaults(channel.to_owned()).required_permission(),
        }
    }
}