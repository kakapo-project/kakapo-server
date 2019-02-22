
use actix::prelude::*;
use actix::Recipient;
use std::collections::HashMap;
use data::channels::Channels;
use uuid::Uuid;


#[derive(Clone, Message, Debug)]
pub struct ActionMessage(pub serde_json::Value);

#[derive(Clone, Message)]
#[rtype(result = "Uuid")]
pub struct SubscribeMessage(pub Channels, pub Uuid, pub Recipient<ActionMessage>);

impl Handler<SubscribeMessage> for WsServer {
    type Result = MessageResult<SubscribeMessage>;

    fn handle(&mut self, msg: SubscribeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let SubscribeMessage(channels, client_id, client) = msg;

        MessageResult(client_id)
    }
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "Uuid")]
pub struct UnsubscribeMessage(pub Channels, pub Uuid);

impl Handler<UnsubscribeMessage> for WsServer {
    type Result = MessageResult<UnsubscribeMessage>;

    fn handle(&mut self, msg: UnsubscribeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let UnsubscribeMessage(channels, client_id) = msg;

        MessageResult(client_id)
    }
}

type Client = Recipient<ActionMessage>;
type SubscribersCollection = HashMap<Uuid, Client>;
#[derive(Default)]
pub struct WsServer {
    rooms: HashMap<Channels, SubscribersCollection>,
}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started the websocket server");
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}