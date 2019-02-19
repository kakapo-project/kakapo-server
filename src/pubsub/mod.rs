
pub mod error;
mod input;

use std::marker::PhantomData;

use uuid::Uuid;

use actix_web::ws;

use actix::ActorContext;
use actix::StreamHandler;
use actix::Actor;

use pubsub::input::WsInputData;
use view::routes;

use AppStateLike;


impl<S> Actor for WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    type Context = ws::WebsocketContext<Self, S>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession[{:?}] opened ", &self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WsSession[{:?}] closed ", &self.id);
    }
}


impl<S> StreamHandler<ws::Message, ws::ProtocolError> for WsClientSession<S>
    where
        S: AppStateLike + 'static,
{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        debug!("received msg: {:?}", msg);
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
            phantom_data: PhantomData,
        }
    }

    fn handle_message(&mut self, ctx: &mut ws::WebsocketContext<Self, S>, input: WsInputData) {
        debug!("receiving message: {:?}", &input);
        let message = match input {
            //TODO:...
            // - Authenticate
            // - GetSubscribers
            // - Subscribe to
            // - Unsubscribe from
            WsInputData::Call { procedure, params, data } => {
                let state = ctx.state();
                routes::call_procedure(&procedure, state, params, data)
                    .and_then(|res| {
                        info!("action message error");
                        let message = serde_json::to_string(&res).unwrap_or_default();
                        Ok(message)
                    })
                    .or_else::<serde_json::Value, _>(|err| {
                        info!("action message ok");
                        let message = serde_json::to_string(&err).unwrap_or_default();
                        Ok(message)
                    })
                    .unwrap_or_default()
            },
        };

        debug!("sending back message: {:?}", &message);
        ctx.text(message);
    }
}
