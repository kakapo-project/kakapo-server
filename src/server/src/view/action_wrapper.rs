
use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
};

use serde_json;

use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use super::state::AppState;
use model::actions;
use model::actions::Action;
use futures::Async;
use model::state::State;
use model::state::ChannelBroadcaster;
use model::state::Channels;
use std::marker::PhantomData;

pub struct Broadcaster;

impl ChannelBroadcaster for Broadcaster {
    fn on_broadcast<T>(&self, channel: Channels, msg: T) {
        unimplemented!()
    }
}

pub struct ActionWrapper<A: Action<Broadcaster> + Send> {
    action: A,
}

impl<A: Action<Broadcaster> + Send> ActionWrapper<A> {
    pub fn new(action: A) -> Self {
        Self {
            action,
        }
    }
}

impl<A: Action<Broadcaster> + Send> Message for ActionWrapper<A>
    where
        A::Ret: 'static,
        Result<A::Ret, actions::error::Error>: 'static,
{
    type Result = Result<A::Ret, actions::error::Error>;
}

impl<A: Action<Broadcaster> + Send> Handler<ActionWrapper<A>> for DatabaseExecutor
    where
        A::Ret: 'static,
        Result<A::Ret, actions::error::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
{
    type Result = Result<A::Ret, actions::error::Error>;

    fn handle(&mut self, msg: ActionWrapper<A>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        let broadcaster = Broadcaster;
        let state = State::new(conn, broadcaster);
        let result = msg.action.call(&state);
        result
    }
}