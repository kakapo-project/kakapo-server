
mod permissions;
mod input;
mod routes;
mod server;
mod action_receiver;

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
use pubsub::server::UnsubscribeMessage;
use pubsub::server::ActionMessage;
use pubsub::server::WsServer;
use actix::Handler;
use actix::SystemService;
use std::collections::HashSet;
use data::channels::Channels;
use pubsub::server::UserSession;

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
    subscriptions: HashSet<Channels>,
    last_beat: DateTime<Utc>,
    auth_header: Option<Vec<u8>>,
    user_id: i64,
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
            subscriptions: HashSet::new(),
            last_beat: Utc::now(),
            auth_header: None,
            user_id: 0,
            phantom_data: PhantomData,
        }
    }

    fn handle_message(&mut self, ctx: &mut ws::WebsocketContext<Self, S>, input: WsInputData) {
        match input {
            WsInputData::Authenticate { token } => {
                info!("Authenticating ws user");
                self.authenticating_user(token, ctx);
            },
            WsInputData::SubscribeTo { channel } => {
                self.send_subscribe_message(channel, ctx);
            },
            WsInputData::UnsubscribeFrom { channel } => {
                self.send_unsubscribe_message(channel, ctx);
            },
            WsInputData::ListSubscribers { channel } => {
                unimplemented!(); //TODO: also need a way to get list of new joins and exits
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


impl<S> WsClientSession<S>
    where S: AppStateLike
{

    fn authenticating_user(&mut self, token: String, ctx: &mut ws::WebsocketContext<Self, S>) {
        let token_secret = ctx.state().get_token_secret();
        let decoded = jsonwebtoken::decode::<AuthClaims>(
            &token,
            token_secret.as_ref(),
            &jsonwebtoken::Validation::default());

        match decoded {
            Ok(x) => {
                let bearer_token = to_bearer_token(token); //need it to be a bearer token for the action wrapper to handle it
                self.auth_header = Some(bearer_token.as_bytes().to_vec());
                self.user_id = x.claims.get_user_id();

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
    }

    fn send_subscribe_message(&mut self, channel: Channels, ctx: &mut ws::WebsocketContext<Self, S>) {
        if self.subscriptions.contains(&channel) {
            warn!("User already subscribed to {:?}", &channel);
            let message = json!({
                        "error": "Already subscribed"
                    });
            let message = serde_json::to_string(&message).unwrap_or_default();
            ctx.text(message);
        } else {
            let user_session = UserSession {
                user_id: self.user_id.to_owned(),
                rec: ctx.address().recipient(),
            };

            let subscribe_msg = SubscribeMessage(channel.to_owned(), self.id.to_owned(), user_session);
            WsServer::from_registry()
                .send(subscribe_msg)
                .into_actor(self)
                .then(|res, actor, ctx| {
                    match res {
                        Ok(id) => {
                            let message = json!("subscribed");
                            let message = serde_json::to_string(&message).unwrap_or_default();
                            actor.subscriptions.insert(channel);
                            ctx.text(message);
                        },
                        Err(err) => {
                            error!("Encountered error subscribing: {}", &err.to_string());
                            let message = json!({
                                        "error": err.to_string()
                                    });
                            let message = serde_json::to_string(&message).unwrap_or_default();
                            ctx.text(message);
                        }
                    }
                    fut::ok(())
                })
                .wait(ctx); //TODO: is spawn better here?
        }
    }

    fn send_unsubscribe_message(&mut self, channel: Channels, ctx: &mut ws::WebsocketContext<Self, S>) {
        if !self.subscriptions.contains(&channel) {
            warn!("User not subscribed to {:?}", &channel);
            let message = json!({
                        "error": "Not subscribed"
                    });
            let message = serde_json::to_string(&message).unwrap_or_default();
            ctx.text(message);
        } else {
            let unsubscribe_msg = UnsubscribeMessage(channel.to_owned(), self.id.to_owned());
            WsServer::from_registry()
                .send(unsubscribe_msg)
                .into_actor(self)
                .then(move |res, actor, ctx| {
                    match res {
                        Ok(id) => {
                            let message = json!("unsubscribed");
                            let message = serde_json::to_string(&message).unwrap_or_default();
                            actor.subscriptions.remove(&channel);
                            ctx.text(message);
                        },
                        Err(err) => {
                            error!("Encountered error unsubscribing: {}", &err.to_string());
                            let message = json!({
                                        "error": err.to_string()
                                    });
                            let message = serde_json::to_string(&message).unwrap_or_default();
                            ctx.text(message);
                        }
                    }
                    fut::ok(())
                })
                .wait(ctx); //TODO: is spawn better here?
        }
    }
}