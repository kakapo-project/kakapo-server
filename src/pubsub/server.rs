
use actix::prelude::*;
use actix::Recipient;
use std::collections::HashMap;
use data::channels::Channels;
use uuid::Uuid;

type SessionId = Uuid;

#[derive(Clone, Message, Debug)]
pub struct ActionMessage(pub serde_json::Value);

#[derive(Clone, Message)]
#[rtype(result = "Uuid")]
pub struct SubscribeMessage(pub Channels, pub SessionId, pub UserSession);

impl Handler<SubscribeMessage> for WsServer {
    type Result = MessageResult<SubscribeMessage>;

    fn handle(&mut self, msg: SubscribeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let SubscribeMessage(channel, session_id, user_session) = msg;

        info!("Adding session [{:?}] to channel: {:?}", &session_id, &channel);
        self.channel_map.entry(channel)
            .or_insert_with(|| HashMap::new())
            .insert(session_id, user_session);

        MessageResult(session_id)
    }
}

#[derive(Clone, Message, Debug)]
#[rtype(result = "Uuid")]
pub struct UnsubscribeMessage(pub Channels, pub SessionId);

impl Handler<UnsubscribeMessage> for WsServer {
    type Result = MessageResult<UnsubscribeMessage>;

    fn handle(&mut self, msg: UnsubscribeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let UnsubscribeMessage(channel, session_id) = msg;

        if let Some(channel_state) = self.channel_map.get_mut(&channel) {
            info!("Removing session [{:?}] from channel {:?}", &session_id, &channel);
            channel_state.remove(&session_id);
        } else {
            warn!("Could not find channel {:?} to unsubscribe from", &channel);
        }

        MessageResult(session_id)
    }
}

#[derive(Clone)]
pub struct UserSession {
    pub user_id: i64,
    pub rec: Recipient<ActionMessage>,
}

type SubscribersCollection = HashMap<SessionId, UserSession>;
#[derive(Default)]
pub struct WsServer {
    channel_map: HashMap<Channels, SubscribersCollection>,
}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("Started the websocket server");
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}