

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
};

use serde_json;

use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use server::state::AppState;
use server::actions;
use server::actions::Action;
use futures::Async;
use server::action_wrapper::ActionWrapper;
use server::actions::Serializable;

use std::time::Instant;
use actix_broker::BrokerSubscribe;

use server::action_wrapper::DatabaseExecutor;

#[derive(Debug, Fail, Serialize)]
enum Error {
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

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
    pub fn dispatch<A>(&mut self, action: A)
        where
            A: Action + 'static,
            Result<A::Ret, actions::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
            <A as Action>::Ret: Serializable,
    {
        self.websocket_context
            .state()
            .connect()
            .send(ActionWrapper::new(action))
            .wait()
            .or_else(|err| {
                error!("encountered unexpected error: {:?}", &err);
                Err(err)
            })
            .and_then(|res| {
                res.or_else(|err| {
                    debug!("Responding with error message: {:?}", &err);
                    let return_val = serde_json::to_string(&json!({
                        "error": err
                    })).unwrap_or_default();

                    self.websocket_context.text(return_val.clone());

                    Err(return_val)
                }).and_then(|ok_res| {
                    let serialized = ok_res.into_serialize();
                    debug!("Responding with message: {:?}", &serialized);

                    let return_val = serde_json::to_string(&json!({
                        "action": <A::Ret as Serializable>::ACTION_NAME,
                        "data": serialized
                    })).unwrap_or_default();

                    self.websocket_context.text(return_val);

                    Ok(())
                });

                Ok(())
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
    fn get_session<'a>(
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
                            "error": format!("{:?}", err) //TODO: the format of the error messages are inconsistent from server errors
                        })).unwrap_or_default();

                        ctx.text(error_msg); //send error message
                        ctx.stop();
                        Err(err)
                    })
                    .and_then(|parameter: P| {
                        let mut session = self.get_session(ctx);
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

