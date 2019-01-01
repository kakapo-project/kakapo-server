

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


use super::state::AppState;
use model::actions::Action;
use futures::Async;
use view::action_wrapper::ActionWrapper;

use view::serializers::Serializable;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub type NoQuery = ();


/// Build `Action` from an http request
pub trait ProcedureBuilder<JP, QP, A: Action> {
    /// build an Action
    ///
    /// # Arguments
    /// * `req` - HttpRequest
    ///
    /// # Returns
    /// an Action
    fn build(self, json_param: JP, query_params: QP) -> A;
}

/// can use lambdas instead of procedure builder
impl<JP, QP, A: Action, F: FnOnce(JP, QP) -> A>
ProcedureBuilder<JP, QP, A> for F
    where
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
{
    fn build(self, json_param: JP, query_params: QP) -> A {
        self(json_param, query_params)
    }
}


/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct
ProcedureHandler<JP, QP, A: Action + 'static, PB: ProcedureBuilder<JP, QP, A> + Clone>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
{
    builder: PB,
    phantom_data: std::marker::PhantomData<(JP, QP, A)>,
}


impl<JP, QP, A: Action, PB: ProcedureBuilder<JP, QP, A> + Clone>
ProcedureHandler<JP, QP, A, PB>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
{
    /// constructor
    pub fn setup(builder: &PB) -> Self {
        ProcedureHandler {
            builder: builder.to_owned(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

pub fn handler_function<JP, QP, A: Action, PB: ProcedureBuilder<JP, QP, A> + Clone>
(procedure_handler: ProcedureHandler<JP, QP, A, PB>, req: HttpRequest<AppState>, json_params: Json<JP>, query_params: Query<QP>) -> AsyncResponse
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
        <A as Action>::Ret: Serializable,
{

    let action = procedure_handler.builder.build(json_params.into_inner(), query_params.into_inner());

    req.state()
        .connect()
        .send(ActionWrapper::new(action))
        .from_err()
        .and_then(|res| {

            res.and_then(|ok_res| {
                let response = HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&ok_res.into_serialize())
                        .unwrap_or_default());

                Ok(response)
            }).or_else(|err| {
                let response = HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&json!({}))
                        .unwrap_or_default());

                Ok(response)
            })
        })
        .responder()
}


