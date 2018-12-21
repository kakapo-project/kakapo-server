

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse,
};

use serde_json;
use std::result::Result::Ok;


use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use super::state::AppState;
use model::actions::Action;
use futures::Async;


type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub trait Parameters {
    fn temp();
}

/// Build `Action` from an http request
pub trait ProcedureBuilder<P: Parameters, M: Action> {
    /// build an Action
    ///
    /// # Arguments
    /// * `req` - HttpRequest
    ///
    /// # Returns
    /// an Action
    fn build(self, param: P) -> M;
}

/// can use lambdas instead of procedure builder
impl<P, M, F> ProcedureBuilder<P, M> for F
    where
        P: Parameters,
        M: Action,
        F: FnOnce(P) -> M,
{
    fn build(self, param: P) -> M {
        self(param)
    }
}

impl<M: Action + Message + Send + 'static>
Handler<M> for DatabaseExecutor
    where
        M::Result: Send,
        <M as Action>::Return: MessageResponse<DatabaseExecutor, M>,
{
    type Result = <M as Action>::Return;

    fn handle(&mut self, msg: M, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        let result = msg.call(&conn);
        result
    }
}

/// Container struct for implemeting the `dev::Handler<AppState>` trait
/// This will extract the `ProcedureBuilder` and execute it asynchronously
pub struct
ProcedureHandler<P: Parameters + 'static, M: Action + Message + Send + 'static, PB: ProcedureBuilder<P, M> + Clone + 'static>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
        Json<P>: FromRequest<AppState>,
{
    builder: PB,
    p: std::marker::PhantomData<P>,
    ph: std::marker::PhantomData<M>,
}


impl<P: Parameters + 'static, M: Action + Message + Send + 'static, PB: ProcedureBuilder<P, M> + Clone + 'static>
ProcedureHandler<P, M, PB>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
        Json<P>: FromRequest<AppState>,
{
    /// constructor
    pub fn setup(builder: &PB) -> Self {
        ProcedureHandler {
            builder: builder.clone(),
            p: std::marker::PhantomData,
            ph: std::marker::PhantomData,
        }
    }
}

fn handler_function<P: Parameters + 'static, M: Action + Message + Send + 'static, PB: ProcedureBuilder<P, M> + Clone + 'static>
(procedure_handler: ProcedureHandler<P, M, PB>, req: HttpRequest<AppState>, params: Json<P>) -> AsyncResponse
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
        Json<P>: FromRequest<AppState>,
{

    let message = procedure_handler.builder.build(params.into_inner());

    req.state()
        .connect(0 /* use master database connector for authentication */)
        .send(message)
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


/// extend actix cors routes to handle RPC
pub trait CorsBuilderExt {

    /// Create an RPC call
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn procedure<P: Parameters + 'static, M: Action + Message + Send + 'static, PB: ProcedureBuilder<P, M> + Clone + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>,
            Json<P>: FromRequest<AppState>;

}

impl CorsBuilderExt for CorsBuilder<AppState> {
    fn procedure<P: Parameters + 'static, M: Action + Message + Send + 'static, PB: ProcedureBuilder<P, M> + Clone + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>,
            Json<P>: FromRequest<AppState>,
    {
        self.resource(path, move |r| {
            r.method(http::Method::POST).with(
                move |(req, parameters): (HttpRequest<AppState>, Json<P>)| {
                    let proc = ProcedureHandler::<P, M, PB>::setup(&procedure_builder);
                    handler_function(proc, req, parameters)
                }
            );
        })
    }
}