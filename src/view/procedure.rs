
use std::fmt::Debug;

use serde::Serialize;
use serde_json;

use actix::prelude::*;
use actix_web::AsyncResponder;
use actix_web::error;
use actix_web::error::JsonPayloadError;
use actix_web::Error as ActixError;
use actix_web::FromRequest;
use actix_web::Json;
use actix_web::Query;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::http::header;

use futures::Future;

use connection::executor::Executor;
use connection::AppStateLike;

use model::actions::Action;
use view::action_wrapper::ActionWrapper;

type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoQuery {}

pub struct ProcedureBuilderContainer<S>
    where S: AppStateLike
{
    f: Box<impl ProcedureBuilder>
}

/// Build `Action` from an http request
pub trait ProcedureBuilder<S, JP, QP, A>
    where
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action + 'static,
        S: AppStateLike,
{
    /// build an Action
    ///
    /// # Arguments
    /// * `req` - HttpRequest
    ///
    /// # Returns
    /// an Action
    fn build(self, json_param: JP, query_params: QP) -> Result<A, serde_json::Error>;

    fn to_container(self) -> ProcedureBuilderContainer<S>;
}

/// can use lambdas instead of procedure builder
impl<S, JP, QP, A, F> ProcedureBuilder<S, JP, QP, A> for F
    where
        F: FnOnce(JP, QP) -> Result<A, serde_json::Error> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action + 'static,
        S: AppStateLike,
{
    fn build(self, json_param: JP, query_params: QP) -> Result<A, serde_json::Error> {
        self(json_param, query_params)
    }

    fn to_container(self) -> ProcedureBuilderContainer<S> {
        ProcedureBuilderContainer {
            f: Box::new(self)
        }
    }
}


/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct ProcedureHandler<S, JP, QP, PB, A>
    where
        Executor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action + 'static,
        S: AppStateLike,
{
    builder: PB,
    phantom_data: std::marker::PhantomData<(S, JP, QP, A)>,
}


impl<S, JP, QP, PB, A> ProcedureHandler<S, JP, QP, PB, A>
    where
        Executor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action,
        S: AppStateLike,
{
    /// constructor
    pub fn setup(builder: &PB) -> Self {
        ProcedureHandler {
            builder: builder.to_owned(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

pub fn procedure_handler_function<S, JP, QP, PB, A>(
    procedure_handler: ProcedureHandler<S, JP, QP, PB, A>,
    req: HttpRequest<S>,
    json_params: Json<JP>,
    query_params: Query<QP>
) -> AsyncResponse
    where
        Executor: Handler<ActionWrapper<A>>,
        PB: ProcedureBuilder<S, JP, QP, A> + Clone,
        JP: Debug,
        QP: Debug,
        Json<JP>: FromRequest<S>,
        Query<QP>: FromRequest<S>,
        A: Action,
        <A as Action>::Ret: Serialize,
        S: AppStateLike,
{

    debug!("Procedure called on {:?} QUERY {:?} JSON {:?}", req.path(), &json_params, &query_params);
    let action = procedure_handler.builder.build(json_params.into_inner(), query_params.into_inner());
    let state = req.state();

    let auth_header = req.headers().get(header::AUTHORIZATION).map(|x| x.as_bytes());

    state
        .connect()
        .send(ActionWrapper::new(auth_header, action))
        .from_err()
        .and_then(|res| match res {
            Ok(ok_res) => {
                let serialized = ok_res.get_data();
                debug!("Responding with message: {:?}", &serialized);
                Ok(HttpResponse::Ok()
                    .json(serialized))
            },
            Err(err) => {
                debug!("Responding with error message: {:?}", &err);
                Ok(HttpResponse::InternalServerError()
                    .json(json!({ "error": err.to_string() })))
            }
        })
        .responder()
}

pub fn procedure_bad_request_handler_function(err: JsonPayloadError) -> actix_web::Error {
    let resp = HttpResponse::BadRequest()
        .json(json!({ "error": err.to_string() }));

    error::InternalError::from_response(err, resp).into()
}

