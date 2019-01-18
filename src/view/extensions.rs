
use actix::prelude::*;

use actix_web::{
    AsyncResponder, error, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json, Query,
    HttpRequest, HttpResponse, ws,
};

use connection::executor::DatabaseExecutor;

use actix_web::middleware::cors::CorsBuilder;


use super::state::AppState;
use super::action_wrapper::ActionWrapper;

use super::procedure::ProcedureBuilder;
use super::procedure::ProcedureHandler;
use super::procedure::procedure_handler_function;
use super::procedure::procedure_bad_request_handler_function;

use model::actions::Action;
use actix_web::dev::JsonConfig;
use std::fmt::Debug;
use serde::Serialize;
// use actix_web::dev::QueryConfig; //TODO: for some reason this can't be imported, probably actix_web issue


/// extend actix cors routes to handle RPC
pub trait CorsBuilderProcedureExt {

    /// Create an RPC call
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn procedure<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            DatabaseExecutor: Handler<ActionWrapper<A>>,
            JP: Debug + 'static,
            QP: Debug + 'static,
            A: Action + Send + 'static,
            PB: ProcedureBuilder<JP, QP, A> + Clone + 'static,
            Json<JP>: FromRequest<AppState, Config = JsonConfig<AppState>>,
            Query<QP>: FromRequest<AppState>,
            <A as Action>::Ret: Send + Serialize;

}


impl CorsBuilderProcedureExt for CorsBuilder<AppState> {
    fn procedure<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            DatabaseExecutor: Handler<ActionWrapper<A>>,
            A: Action + Send + 'static,
            PB: ProcedureBuilder<JP, QP, A> + Clone + 'static,
            JP: Debug + 'static,
            QP: Debug + 'static,
            Json<JP>: FromRequest<AppState, Config = JsonConfig<AppState>>,
            Query<QP>: FromRequest<AppState>,
            <A as Action>::Ret: Send + Serialize,
    {
        self.resource(path, move |r| {
            r.method(http::Method::POST).with_config(
                move |(req, json_params, query_params): (HttpRequest<AppState>, Json<JP>, Query<QP>)| {
                    let proc = ProcedureHandler::<JP, QP, PB, A>::setup(&procedure_builder);
                    procedure_handler_function(proc, req, json_params, query_params)
                },
                |((_, json_cfg, query_cfg),)| {
                    json_cfg
                        .error_handler(|err, req| {
                            procedure_bad_request_handler_function(err)
                        });
                }
            );
        })
    }
}

