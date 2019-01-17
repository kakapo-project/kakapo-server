

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
use model::actions;
use futures::Async;
use view::action_wrapper::ActionWrapper;

use view::serializers::Serializable;
use actix_web::error;
use actix_web::ResponseError;
use view::error::Error;
use std::fmt::Debug;
use model::state::ChannelBroadcaster;
use view::action_wrapper::Broadcaster;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub type NoQuery = ();


/// Build `Action` from an http request
pub trait ProcedureBuilder<JP, QP, A>
    where
        A: Action<Broadcaster>,
{
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
impl<JP, QP, A, F> ProcedureBuilder<JP, QP, A> for F
    where
        A: Action<Broadcaster>,
        F: FnOnce(JP, QP) -> A,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
{
    fn build(self, json_param: JP, query_params: QP) -> A {
        self(json_param, query_params)
    }
}


/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct ProcedureHandler<JP, QP, A, PB>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        A: Action<Broadcaster> + 'static,
        PB: ProcedureBuilder<JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
{
    builder: PB,
    phantom_data: std::marker::PhantomData<(JP, QP, A)>,
}


impl<JP, QP, A, PB> ProcedureHandler<JP, QP, A, PB>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<JP, QP, A> + Clone,
        A: Action<Broadcaster>,
        JP: Debug,
        QP: Debug,
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

pub fn procedure_handler_function<JP, QP, A, PB>(
    procedure_handler: ProcedureHandler<JP, QP, A, PB>,
    req: HttpRequest<AppState>,
    json_params: Json<JP>,
    query_params: Query<QP>
) -> AsyncResponse
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<JP, QP, A> + Clone,
        A: Action<Broadcaster>,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<AppState>,
        Query<QP>: FromRequest<AppState>,
        <A as Action<Broadcaster>>::Ret: Serializable,
{

    debug!("Procedure called on {:?} QUERY {:?} JSON {:?}", req.path(), &json_params, &query_params);
    let action = procedure_handler.builder.build(json_params.into_inner(), query_params.into_inner());

    req.state()
        .connect()
        .send(ActionWrapper::new(action))
        .from_err()
        .and_then(|res| {
            match res {
                Ok(ok_res) => {
                    let serialized = ok_res.into_serialize();
                    debug!("Responding with message: {:?}", &serialized);
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&serialized)
                            .unwrap_or_default()))
                },
                Err(err) => {
                    debug!("Responding with error message: {:?}", &err);
                    Ok(HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body(serde_json::to_string(&json!({ "error": err.to_string() }))
                            .unwrap_or_default()))
                }
            }
        })
        .responder()
}


