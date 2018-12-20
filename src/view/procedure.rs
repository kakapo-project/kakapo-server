

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    HttpRequest, HttpResponse,
};

use serde_json;
use std::result::Result::Ok;


use connection::executor::DatabaseExecutor;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use super::state::AppState;
use super::actions::Action;


type AsyncResponse = Box<Future<Item=HttpResponse, Error=ActixError>>;

pub trait ProcedureBuilder<M: Action> {
    fn build(req: &HttpRequest<AppState>) -> M;
}

// handle procedure and turn into http result
pub struct ProcedureHandler
<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M>>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>
{
    procedure_builder: PB,
    ph: std::marker::PhantomData<M>,
}


impl<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static> ProcedureHandler<M, PB>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
{
    pub fn setup(procedure_builder: PB) -> Self {
        ProcedureHandler { procedure_builder, ph: std::marker::PhantomData }
    }
}


impl<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
MsgHandler<AppState> for ProcedureHandler<M, PB>
    where
        M::Result: Send,
        DatabaseExecutor: Handler<M>,
{
    type Result = AsyncResponse;

    fn handle(&self, req: &HttpRequest<AppState>) -> AsyncResponse {

        println!("handling connection: ...");

        req.state()
            .connect(0 /* use master database connector for authentication */)
            .send(PB::build(req))
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
}

// extend actix router
pub trait CorsBuilderExt {
    fn procedure<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>;

}

impl CorsBuilderExt for CorsBuilder<AppState> {
    fn procedure<M: Action + Message + Send + 'static, PB: ProcedureBuilder<M> + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            M::Result: Send,
            DatabaseExecutor: Handler<M>,
    {
        self.resource(path, |r| {
            r.method(http::Method::POST).h(ProcedureHandler::<M, PB>::setup(procedure_builder));
        })
    }
}