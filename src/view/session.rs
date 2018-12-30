

use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
};

use serde_json;

use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use super::state::AppState;
use model::actions;
use model::actions::Action;
use futures::Async;
use data::api;
use view::action_wrapper::ActionWrapper;
use model::actions::results::NamedActionResult;

use view::error;

pub struct Session<'a, P: 'static, SL: SessionListener<P> + Clone + 'static> {
    websocket_context: &'a mut ws::WebsocketContext<SessionActor<P, SL>, AppState>,
}

impl<'a, P: 'static, SL: SessionListener<P> + Clone + 'static> Session<'a, P, SL> {
    pub fn subscribeTo() {

    }

    pub fn unsubscribeFrom() {

    }

    /// dispatch a response from action inside the listener
    pub fn dispatch<A: Action + 'static>(&mut self, action: A)
        where
            Result<A::Ret, actions::error::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
    {
        self.websocket_context
            .state()
            .connect(0)
            .send(ActionWrapper::new(action))
            .wait()
            .or_else(|err| Err(error::Error::TooManyConnections))
            .and_then(|res| {

                let ok_result = res.or_else(|err| Err(error::Error::TooManyConnections))?;
                let return_val = serde_json::to_string(&json!({
                    "action": <A::Ret as NamedActionResult>::ACTION_NAME,
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


pub struct SessionActor<P: 'static, SL: SessionListener<P> + Clone + 'static> {
    session_id: usize,
    listener: SL,
    phantom_p: std::marker::PhantomData<P>, //spooky
}

impl<P: 'static, SL: SessionListener<P> + Clone + 'static> SessionActor<P, SL>
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
                        let mut session = self.build_session(ctx);
                        self.listener.listen(&mut session, parameter);
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

