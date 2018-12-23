

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
use view::action_wrapper::ActionWrapper;


type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

/// Build `Action` from an http request
pub trait ProcedureBuilder<P, A: Action> {
    /// build an Action
    ///
    /// # Arguments
    /// * `req` - HttpRequest
    ///
    /// # Returns
    /// an Action
    fn build(self, param: P) -> A;
}

/// can use lambdas instead of procedure builder
impl<P, A, F> ProcedureBuilder<P, A> for F
    where
        A: Action,
        F: FnOnce(P) -> A,
{
    fn build(self, param: P) -> A {
        self(param)
    }
}




/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct
ProcedureHandler<P, A: Action + Send + 'static, PB: ProcedureBuilder<P, A> + Clone>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<P>: FromRequest<AppState>,
{
    builder: PB,
    phantom_p: std::marker::PhantomData<P>,
    phantom_a: std::marker::PhantomData<A>,
}


impl<P, A: Action + Send, PB: ProcedureBuilder<P, A> + Clone>
ProcedureHandler<P, A, PB>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<P>: FromRequest<AppState>,
        <A as Action>::Result: Send,
{
    /// constructor
    pub fn setup(builder: &PB) -> Self {
        ProcedureHandler {
            builder: builder.to_owned(),
            phantom_p: std::marker::PhantomData,
            phantom_a: std::marker::PhantomData,
        }
    }
}

pub fn handler_function<P, A: Action + Send, PB: ProcedureBuilder<P, A> + Clone>
(procedure_handler: ProcedureHandler<P, A, PB>, req: HttpRequest<AppState>, params: Json<P>) -> AsyncResponse
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<P>: FromRequest<AppState>,
        <A as Action>::Result: Send,
{

    let action = procedure_handler.builder.build(params.into_inner());

    req.state()
        .connect(0 /* use master database connector for authentication */)
        .send(ActionWrapper::new(action))
        .from_err()
        .and_then(|res| {
            let fin = HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&json!({ "success": "all is well" }))
                    .unwrap_or_default());

            Ok(fin)
        })
        .responder()
}


