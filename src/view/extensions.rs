
use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
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

use model::actions::{ Action, ActionResult};


/// extend actix cors routes to handle RPC
pub trait CorsBuilderProcedureExt {

    /// Create an RPC call
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn procedure<P: 'static, A: Action + Send + 'static, PB: ProcedureBuilder<P, A> + Clone + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            DatabaseExecutor: Handler<ActionWrapper<A>>,
            Json<P>: FromRequest<AppState>,
            <A as Action>::Result: Send;

}

impl CorsBuilderProcedureExt for CorsBuilder<AppState> {
    fn procedure<P: 'static, A: Action + Send + 'static, PB: ProcedureBuilder<P, A> + Clone + 'static>
    (&mut self, path: &str, procedure_builder: PB) -> &mut CorsBuilder<AppState>
        where
            DatabaseExecutor: Handler<ActionWrapper<A>>,
            Json<P>: FromRequest<AppState>,
            <A as Action>::Result: Send,
    {
        self.resource(path, move |r| {
            r.method(http::Method::POST).with(
                move |(req, parameters): (HttpRequest<AppState>, Json<P>)| {
                    let proc = ProcedureHandler::<P, A, PB>::setup(&procedure_builder);
                    handler_function(proc, req, parameters)
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
                    let session = SessionActor::<P, SL>::setup(&session_listener);
                    ws::start(req, session)
                })
        })
    }
}