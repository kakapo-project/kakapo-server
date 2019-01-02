

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
};

use serde_json;
use std::error::Error;

use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use super::state::AppState;
use model::actions;
use model::actions::Action;
use futures::Async;
use view::action_wrapper::ActionWrapper;
use view::serializers::Serializable;

use view::error;
use std::time::Instant;

pub struct Session<'a, P, SL>
    where
        P: 'static,
        SL: SessionListener<P> + Clone + 'static,
{
    websocket_context: &'a mut ws::WebsocketContext<SessionActor<P, SL>, AppState>,
}

impl<'a, P, SL> Session<'a, P, SL>
    where
        P: 'static,
        SL: SessionListener<P> + Clone + 'static,
{
    pub fn subscribeTo() {

    }

    pub fn unsubscribeFrom() {

    }

    /// dispatch a response from action inside the listener
    pub fn dispatch<A: Action + 'static>(&mut self, action: A)
        where
            Result<A::Ret, actions::error::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
            <A as Action>::Ret: Serializable,
    {
        self.websocket_context
            .state()
            .connect()
            .send(ActionWrapper::new(action))
            .wait()
            .or_else(|err| Err(error::Error::TooManyConnections))
            .and_then(|res| {

                let ok_result = res.or_else(|err| Err(error::Error::TooManyConnections))?;
                let return_val = serde_json::to_string(&json!({
                    "action": <A::Ret as Serializable>::ACTION_NAME,
                    "success": "callback from dispatch"
                })).unwrap();

                self.websocket_context.text(return_val);
                Ok(())
            })
            .or_else(|err| {
                println!("encountered error: {:?}", &err);
                Err(err)
            });
    }
}

pub trait SessionListener<P>: Clone {
    fn listen(&self, session: &mut Session<P, Self>, param: P);
}


pub struct SessionActor<P, SL>
    where
        P: 'static,
        SL: SessionListener<P> + Clone + 'static,
{
    session_id: usize,
    heartbeat: Instant,
    listener: SL,
    phantom_data: std::marker::PhantomData<P>, //spooky
}

impl<P, SL> SessionActor<P, SL>
    where
        P: 'static,
        SL: SessionListener<P> + Clone + 'static,
{
    fn build_session<'a>(
        &self,
        websocket_context: &'a mut ws::WebsocketContext<SessionActor<P, SL>, AppState>
    ) -> Session<'a, P, SL> {
        Session {
            websocket_context
        }
    }
}

impl<P, SL> Actor for SessionActor<P, SL>
    where
        P: 'static,
        SL: SessionListener<P> + Clone + 'static,
{
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl<P, SL> SessionActor<P, SL>
    where
        SL: SessionListener<P> + Clone,
{
    pub fn setup(listener: &SL) -> Self {
        Self {
            session_id: 0,
            heartbeat: Instant::now(),
            listener: listener.clone(),
            phantom_data: std::marker::PhantomData,
        }
    }
}

impl<P, SL> StreamHandler<ws::Message, ws::ProtocolError> for SessionActor<P, SL>
    where
        P: serde::de::DeserializeOwned + 'static,
        SL: SessionListener<P> + Clone + 'static,
        for<'de> P: serde::Deserialize<'de>

{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Text(text) => {
                self.heartbeat = Instant::now();

                serde_json::from_str(&text)
                    .or_else::<serde_json::error::Error, _>(|err| {
                        debug!("could not parse websocket request: {:?}", &text);
                        let error_msg = serde_json::to_string(&json!({
                            "error": err.description()
                        })).unwrap_or_default();

                        ctx.text(error_msg); //send error message
                        ctx.stop();
                        Err(err)
                    })
                    .and_then(|parameter: P| {
                        let mut session = self.build_session(ctx);
                        self.listener.listen(&mut session, parameter);
                        Ok(())
                    });

            },
            ws::Message::Binary(_) => {
                debug!("Received unexpected binary in websocket");
                let error_msg = serde_json::to_string(&json!({
                    "error": "binary requests not currently supported"
                })).unwrap_or_default();
                ctx.text(error_msg);
            },
            ws::Message::Ping(text) => {
                self.heartbeat = Instant::now();
                ctx.pong(&text);
            },
            ws::Message::Pong(text) => {
                self.heartbeat = Instant::now();
            },
            ws::Message::Close(_) => {
                info!("Socket connection closed");
                ctx.stop();
            },
        };
    }
}

