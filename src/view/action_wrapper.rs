
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
use model::actions::{ Action, ActionResult};
use futures::Async;
use data::api;

pub struct ActionWrapper<A: Action + Send> {
    action: A
}

impl<A: Action + Send> ActionWrapper<A> {
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A: Action + Send> Message for ActionWrapper<A>
    where
        <A as Action>::Result: 'static
{
    type Result = A::Result;
}

impl<A: Action + Send> Handler<ActionWrapper<A>> for DatabaseExecutor
    where
        A::Result: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
{
    type Result = A::Result;

    fn handle(&mut self, msg: ActionWrapper<A>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        let result = msg.action.call(&conn);
        result
    }
}