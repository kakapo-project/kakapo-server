
pub mod error;
mod permissions;
mod input;
mod routes;
mod server;

use std::marker::PhantomData;

use uuid::Uuid;

use futures::Future;

use actix_web::ws;
use actix_web::HttpResponse;

use actix::ActorContext;
use actix::StreamHandler;
use actix::Actor;
use actix::fut;
use actix::WrapFuture;
use actix::ActorFuture;
use actix::ContextFutureSpawner;
use actix::AsyncContext;

use AppStateLike;
use view::action_wrapper::ActionWrapper;
use view::procedure::ProcedureBuilder;
use view::error::Error::TooManyConnections;
use model::actions::Action;

use pubsub::input::WsInputData;
use pubsub::routes::CallAction;
use data::claims::AuthClaims;
use view::bearer_token::to_bearer_token;
use chrono::Utc;
use chrono::DateTime;
use pubsub::routes::CallParams;
use pubsub::server::SubscribeMessage;
use pubsub::server::ActionMessage;
use pubsub::server::WsServer;
use actix::Handler;
use actix::SystemService;

impl<S> Actor for WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    type Context = ws::WebsocketContext<Self, S>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession [{}] opened ", &self.id.to_hyphenated_ref());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WsSession[{}] closed ", &self.id.to_hyphenated_ref());
    }
}

impl<S> Handler<ActionMessage> for WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: ActionMessage, ctx: &mut Self::Context) {
        info!("handling message: {:?}", &msg);
    }
}

impl<S> StreamHandler<ws::Message, ws::ProtocolError> for WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {

        //updating the heartbeat
        self.last_beat = Utc::now();

        match msg {
            ws::Message::Text(text) => {
                let _ = serde_json::from_str(&text)
                    .or_else(|err| {
                        warn!("could not understand incoming message, must be `WsInputData`");
                        let message = json!({
                            "error": "Could not understand message"
                        });
                        let message = serde_json::to_string(&message).unwrap_or_default();
                        ctx.text(message);
                        Err(())
                    })
                    .and_then(move |res: WsInputData| {
                        debug!("handling message");
                        self.handle_message(ctx, res);
                        Ok(())
                    });
            },
            ws::Message::Close(_) => {
                info!("Closing connection");
                ctx.stop();
            },
            ws::Message::Binary(_) => {
                warn!("binary websocket messages not currently supported");
                let message = json!({
                    "error": "Binary format not supported"
                });
                let message = serde_json::to_string(&message).unwrap_or_default();
                ctx.text(message);
            },
            ws::Message::Ping(_) => {
                //TODO....
            },
            ws::Message::Pong(_) => {
                //TODO:...
            },
        }
    }
}


#[derive(Clone, Debug)]
pub struct WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    pub id: Uuid,
    last_beat: DateTime<Utc>,
    auth_header: Option<Vec<u8>>,
    phantom_data: PhantomData<(S)>,
}

impl<S> WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            last_beat: Utc::now(),
            auth_header: None,
            phantom_data: PhantomData,
        }
    }

    fn handle_message(&mut self, ctx: &mut ws::WebsocketContext<Self, S>, input: WsInputData) {
        match input {
            WsInputData::Authenticate { token } => {
                info!("Authenticating ws user");

                let token_secret = ctx.state().get_token_secret();
                let decoded = jsonwebtoken::decode::<AuthClaims>(
                    &token,
                    token_secret.as_ref(),
                    &jsonwebtoken::Validation::default());

                match decoded {
                    Ok(x) => {
                        let bearer_token = to_bearer_token(token); //need it to be a bearer token for the action wrapper to handle it
                        self.auth_header = Some(bearer_token.as_bytes().to_vec());

                        let message = json!("authenticated");
                        let message = serde_json::to_string(&message).unwrap_or_default();
                        ctx.text(message);
                    },
                    Err(err) => {
                        error!("encountered error trying to decode token: {:?}", &err);
                        let message = json!({
                            "error": "Could not authenticate token"
                        });
                        let message = serde_json::to_string(&message).unwrap_or_default();
                        ctx.text(message);
                    }
                }
            },
            WsInputData::SubscribeTo { channel } => {
                let subscribe_msg = SubscribeMessage(channel, self.id.to_owned(), ctx.address().recipient());

                WsServer::from_registry()
                    .send(subscribe_msg)
                    .into_actor(self)
                    .then(|res, actor, ctx| {

                        fut::ok(())
                    })
                    .wait(ctx); //TODO: is spawn better here?
            },
            WsInputData::UnsubscribeFrom { channel } => {

            },
            WsInputData::ListSubscribers { channel } => {

            },
            WsInputData::Call { procedure, params, data } => {
                debug!("calling procedure: {:?}", &procedure);
                let mut call_params = CallParams {
                    data, params, ctx
                };

                let result = routes::call_procedure(&procedure, self, &mut call_params);
                debug!("finished calling procedure {:?}", &result);
            },
        };
    }
}

impl<S> CallAction<S> for WsClientSession<S>
    where S: AppStateLike
{
    /// For use by the websockets
    fn call<'a, PB, A>(&mut self, procedure_builder: PB, call_params: &mut CallParams<'a, S>)
        where
            PB: ProcedureBuilder<S, serde_json::Value, serde_json::Value, A> + Clone + 'static,
            S: AppStateLike + 'static,
            A: Action + 'static,
    {

        let action = procedure_builder
            .build(call_params.data.to_owned(), call_params.params.to_owned());

        let mut action_wrapper = ActionWrapper::new(action);

        if let Some(ref auth) = self.auth_header {
            action_wrapper = action_wrapper.with_auth(&auth);
        }

        call_params
            .ctx
            .state()
            .connect()
            .send(action_wrapper)
            .into_actor(self)
            .then(|res, actor, ctx| {
                match res {
                    Ok(ok_res) => match ok_res {
                        Ok(res) => {
                            info!("action message ok");
                            //TODO: need the action name
                            let message = serde_json::to_string(&res.get_data()).unwrap_or_default();
                            ctx.text(message);
                        },
                        Err(err) => {
                            info!("action message error");
                            let message = serde_json::to_string(&json!({"error": err.to_string()})).unwrap_or_default();
                            ctx.text(message)
                        }
                    },
                    Err(err) => {
                        error!("websocket error occurred with error message: {:?}", &err);
                        let message = serde_json::to_string(&json!({"error": err.to_string()})).unwrap_or_default();
                        ctx.text(message)
                    }
                }

                fut::ok(())
            })
            .wait(&mut call_params.ctx); //TODO: is spawn better here?
    }

    fn error<'a>(&mut self, call_params: &mut CallParams<'a, S>)
        where
            S: AppStateLike + 'static
    {
        unimplemented!()
    }
}