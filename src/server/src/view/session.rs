

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json, Query,
    HttpRequest, HttpResponse, ws,
};

use serde_json;

use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use view::state::AppState;
use model::actions;
use model::actions::Action;
use futures::Async;
use view::action_wrapper::ActionWrapper;

use view::serializers::Serializable;
use actix_web::error;
use actix_web::ResponseError;
use view::error::Error;
use std::fmt::Debug;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub type NoQuery = ();

pub struct SessionContext {
    req: HttpRequest<AppState>
}

impl SessionContext {

    pub fn new(req: HttpRequest<AppState>) -> Self {
        Self { req }
    }

    pub fn subscribeTo() -> () {

    }

    pub fn unsubscribeFrom() -> () {

    }

    pub fn dispatch<A>(&mut self, action: A)
        where
            A: Action + 'static,
            Result<A::Ret, actions::error::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
            <A as Action>::Ret: Serializable,
    {
        self.req
            .state()
            .connect()
            .send(ActionWrapper::new(action))
            .wait()
            .or_else(|err| {
                error!("encountered unexpected error: {:?}", &err);
                Err(err)
            })
            .and_then(|res| {
                //TODO: send result
                Ok(res)
            });
    }
}

pub trait SessionListener<JP> {

    fn listen(self, context: SessionContext, param: JP);
}


/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `SessionBuilder` and execute it asynchronously
pub struct SessionHandler<JP, SL>
    where
        SL: SessionListener<JP> + Clone,
        JP: Debug,
        Json<JP>: FromRequest<AppState>,
{
    listener: SL,
    phantom_data: std::marker::PhantomData<JP>,
}


impl<JP, SL> SessionHandler<JP, SL>
    where
        SL: SessionListener<JP> + Clone,
        JP: Debug,
        Json<JP>: FromRequest<AppState>,
{
    /// constructor
    pub fn setup(listener: &SL) -> Self {
        SessionHandler {
            listener: listener.to_owned(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

pub fn session_handler_function<JP, SL>(
    session_handler: SessionHandler<JP, SL>,
    req: HttpRequest<AppState>,
    json_params: Json<JP>,
) -> HttpResponse
    where
        SL: SessionListener<JP> + Clone,
        JP: Debug,
        Json<JP>: FromRequest<AppState>,
{

    debug!("Session action called on {:?} JSON {:?}", req.path(), &json_params);
    let session_context = SessionContext::new(req);
    session_handler.listener.listen(session_context, json_params.into_inner());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&json!({ "success": true }))
            .unwrap_or_default())
}


