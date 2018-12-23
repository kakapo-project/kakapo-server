

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
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
use data::api;


pub struct Session {

}

impl Session {
    pub fn subscribeTo() {

    }

    pub fn unsubscribeFrom() {

    }
}

pub trait SessionListener<P> {
    fn listen(&self, session: Session, param: P);
}


struct SessionActor<P, SL: SessionListener<P> + Clone> {
    session_id: usize,
    listener: SL,
    phantom_p: std::marker::PhantomData<P>, //spooky
}

impl<P, SL: SessionListener<P> + Clone> SessionActor<P, SL>
{
    fn build_session(
        &self,
        /*websocket_context: &mut ws::WebsocketContext<SessionActor<P, A, SL>, AppState>*/
    ) -> Session {
        Session {}
    }
}

impl<
    P: 'static,
    SL: SessionListener<P> + Clone + 'static>
Actor for SessionActor<P, SL> {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl<P, SL: SessionListener<P> + Clone>
SessionActor<P, SL> {
    pub fn setup(listener: &SL) -> Self {
        Self {
            session_id: 0,
            listener: listener.clone(),
            phantom_p: std::marker::PhantomData,
        }
    }
}

impl<
    P: serde::de::DeserializeOwned + 'static,
    SL: SessionListener<P> + Clone + 'static>
StreamHandler<ws::Message, ws::ProtocolError> for SessionActor<P, SL>
    where
        for<'de> P: serde::Deserialize<'de>

{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                // parse json
                serde_json::from_str(&text)
                    .or_else::<serde_json::error::Error, _>(|err| {
                        println!("Error occured while parsing websocket request: {:?}", err);
                        ctx.stop();
                        //TODO: send error message
                        Err(err)
                    })
                    .and_then(|parameter: P| {
                        let session: Session = self.build_session(/*ctx*/);
                        self.listener.listen(session, parameter);
                        Ok(())
                    });

            },
            ws::Message::Close(_) => {
                ctx.stop();
            },
            _ => {}
        };
    }
}

/// extend actix cors routes to handle RPC
pub trait CorsBuilderExt {

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

impl CorsBuilderExt for CorsBuilder<AppState> {


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