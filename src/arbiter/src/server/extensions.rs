
use actix::prelude::*;

use actix_web::{
    AsyncResponder, error, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json, Query,
    HttpRequest, HttpResponse, ws,
};


use actix_web::middleware::cors::CorsBuilder;

use server::state::AppState;
use server::session::SessionListener;
use server::session::SessionActor;

use actix_web::dev::JsonConfig;
use std::fmt::Debug;
// use actix_web::dev::QueryConfig; //TODO: for some reason this can't be imported, probably actix_web issue

/// extend actix cors routes to handle Session
pub trait CorsBuilderSessionExt {

    /// Create a websocket session
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `session_builder` - An object extending `SessionBuilder` for building a session
    ///
    fn session<P, SL>(&mut self, path: &str, session_listener: SL) -> &mut CorsBuilder<AppState>
        where
            P: serde::de::DeserializeOwned + 'static,
            SL: SessionListener<P> + Clone + 'static,
            Json<P>: FromRequest<AppState>,
            for<'de> P: serde::Deserialize<'de>;

}

impl CorsBuilderSessionExt for CorsBuilder<AppState> {


    fn session<P, SL>(&mut self, path: &str, session_listener: SL) -> &mut CorsBuilder<AppState>
        where
            P: serde::de::DeserializeOwned + 'static,
            SL: SessionListener<P> + Clone + 'static,
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