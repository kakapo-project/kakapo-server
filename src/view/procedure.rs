

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    FromRequest, Json, Query,
    HttpRequest, HttpResponse,
};

use serde_json;

use connection::executor::DatabaseExecutor;
use futures::Future;


use connection::AppState;
use model::actions::Action;
use view::action_wrapper::ActionWrapper;

use actix_web::error;
use std::fmt::Debug;
use actix_web::error::JsonPayloadError;
use serde::Serialize;
use connection::GetAppState;
use connection::Auth;
use connection::Broadcaster;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub type NoQuery = ();


/// Build `Action` from an http request
pub trait ProcedureBuilder<S, AU, B, JP, QP, A> {
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
impl<S, AU, B, JP, QP, A, F> ProcedureBuilder<S, AU, B, JP, QP, A> for F
    where
        F: FnOnce(JP, QP) -> A,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        S: GetAppState<AU, B>,
        AU: Auth,
        B: Broadcaster,
{
    fn build(self, json_param: JP, query_params: QP) -> A {
        self(json_param, query_params)
    }
}


/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct ProcedureHandler<S, AU, B, JP, QP, PB, A>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, AU, B, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action + 'static,
        S: GetAppState<AU, B>,
        AU: Auth,
        B: Broadcaster,
{
    builder: PB,
    phantom_data: std::marker::PhantomData<(S, AU, B, JP, QP, A)>,
}


impl<S, AU, B, JP, QP, PB, A> ProcedureHandler<S, AU, B, JP, QP, PB, A>
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, AU, B, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action,
        S: GetAppState<AU, B>,
        AU: Auth,
        B: Broadcaster,
{
    /// constructor
    pub fn setup(builder: &PB) -> Self {
        ProcedureHandler {
            builder: builder.to_owned(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

pub fn procedure_handler_function<S, AU, B, JP, QP, PB, A>(
    procedure_handler: ProcedureHandler<S, AU, B, JP, QP, PB, A>,
    req: HttpRequest<S>,
    json_params: Json<JP>,
    query_params: Query<QP>
) -> AsyncResponse
    where
        DatabaseExecutor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, AU, B, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action,
        <A as Action>::Ret: Serialize,
        S: GetAppState<AU, B>,
        AU: Auth,
        B: Broadcaster,
{

    debug!("Procedure called on {:?} QUERY {:?} JSON {:?}", req.path(), &json_params, &query_params);
    let action = procedure_handler.builder.build(json_params.into_inner(), query_params.into_inner());

    req.state()
        .get_app_state()
        .connect()
        .send(ActionWrapper::new(action))
        .from_err()
        .and_then(|res| {
            match res {
                Ok(ok_res) => {
                    let serialized = ok_res;
                    debug!("Responding with message: {:?}", &serialized);
                    Ok(HttpResponse::Ok()
                        .json(serialized))
                },
                Err(err) => {
                    debug!("Responding with error message: {:?}", &err);
                    Ok(HttpResponse::InternalServerError()
                        .json(json!({ "error": err.to_string() })))
                }
            }
        })
        .responder()
}

pub fn procedure_bad_request_handler_function(err: JsonPayloadError) -> actix_web::Error {
    let resp = HttpResponse::BadRequest()
        .json(json!({ "error": err.to_string() }));

    error::InternalError::from_response(err, resp).into()
}

