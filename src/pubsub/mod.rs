
pub mod error;
mod input;

use std::marker::PhantomData;

use uuid::Uuid;

use futures::Future;

use actix_web::ws;

use actix::ActorContext;
use actix::StreamHandler;
use actix::Actor;

use pubsub::input::WsInputData;
use view::routes;

use AppStateLike;
use view::action_wrapper::ActionWrapper;
use view::routes::CallAction;
use view::procedure::ProcedureBuilder;
use model::actions::Action;

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
        match input {
            //TODO:...
            // - Authenticate
            // - GetSubscribers
            // - Subscribe to
            // - Unsubscribe from
            WsInputData::Call { procedure, params, data } => {
                let state = ctx.state();
                let callback = WsCallback { state, data, params, };
                routes::call_procedure(&procedure, callback);
            },
        };
    }
}

struct WsCallback<'a, S>
    where S: AppStateLike
{
    state: &'a S,
    data: serde_json::Value,
    params: serde_json::Value,
}

impl<'a, S> CallAction<S> for WsCallback<'a, S>
    where S: AppStateLike
{
    /// For use by the websockets
    fn call<PB, A>(&self, procedure_builder: PB) -> ()
        where
            PB: ProcedureBuilder<S, serde_json::Value, serde_json::Value, A> + Clone + 'static,
            S: AppStateLike + 'static,
            A: Action + 'static,
    {

        let action = procedure_builder
            .build(self.data.to_owned(), self.params.to_owned());

        //TODO: auth

        debug!("calling action asynchronously");
        self.state
            .connect()
            //FIXME: ... it deadlocks here
            .send(ActionWrapper::new(None, action))
            .and_then(|res| match res {
                Ok(ok_res) => {
                    //let serialized = ok_res.get_data();
                    debug!("Responding with message: {:?}", &ok_res);
                    Ok(())
                },
                Err(err) => {
                    debug!("Responding with error message: {:?}", &err);
                    Ok(())
                }
            });

        /*
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
        */
    }

    fn error(&self) -> () {
        unimplemented!()
    }
}