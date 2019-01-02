
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
use super::session::SessionListener;
use super::session::SessionActor;
use super::procedure::ProcedureBuilder;
use super::procedure::ProcedureHandler;
use super::procedure::handler_function;

use model::actions::Action;
use view::serializers::Serializable;
use actix_web::dev::JsonConfig;
use std::fmt::Debug;
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
            <A as Action>::Ret: Send + Serializable;

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
            <A as Action>::Ret: Send + Serializable,
    {
        self.resource(path, move |r| {
            r.method(http::Method::POST).with_config(
                move |(req, json_params, query_params): (HttpRequest<AppState>, Json<JP>, Query<QP>)| {
                    let proc = ProcedureHandler::<JP, QP, A, PB>::setup(&procedure_builder);
                    handler_function(proc, req, json_params, query_params)
                },
                |((_, json_cfg, query_cfg),)| {
                    json_cfg
                        .error_handler(|err, req| {
                            let resp = HttpResponse::BadRequest()
                                .content_type("application/json")
                                .body(serde_json::to_string(&json!({ "error": err.to_string() }))
                                    .unwrap_or_default());

                            error::InternalError::from_response(err, resp).into()
                        });
                }
            );
        })
    }
}

/// extend actix cors routes to handle RPC
pub trait CorsBuilderSessionExt {

    /// Create a websocket session
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `session_builder` - An object extending `SessionBuilder` for building a session
    ///
    fn session<
        P: serde::de::DeserializeOwned + 'static,
        SL: SessionListener<P> + Clone + 'static>
    (&mut self, path: &str, session_listener: SL) -> &mut CorsBuilder<AppState>
        where
            Json<P>: FromRequest<AppState>,
            for<'de> P: serde::Deserialize<'de>;

}

impl CorsBuilderSessionExt for CorsBuilder<AppState> {


    fn session<
        P: serde::de::DeserializeOwned + 'static,
        SL: SessionListener<P> + Clone + 'static>
    (&mut self, path: &str, session_listener: SL) -> &mut CorsBuilder<AppState>
        where
            Json<P>: FromRequest<AppState>,
            for<'de> P: serde::Deserialize<'de>,
    {
        self.resource(path, move |r| {
            r.method(http::Method::GET).f(
                move |req: &HttpRequest<AppState>| {
                    debug!(
                        "websocket connection established on {:?} FROM {:?} \"{:?}\"",
                        &req.path(),
                        &req.headers().get(http::header::HOST),
                        &req.headers().get(http::header::USER_AGENT),
                    );
                    let session = SessionActor::<P, SL>::setup(&session_listener);
                    ws::start(req, session)
                })
        })
    }
}