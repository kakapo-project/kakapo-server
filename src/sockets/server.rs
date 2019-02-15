use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use state::api::ApiOkResponse;
use state::api::ApiErrorResponse;
use state::api::Channel;
use std::iter::FromIterator;

use sockets::Notification;

use uuid::Uuid;
use state::api::UserData;

type Client = Recipient<Notification>;


#[derive(Clone)]
struct UserSession {
    client: Client,
    user: UserData,
}

impl UserSession {
    pub fn new(client: Client, user: UserData) -> Self {
        Self { client, user }
    }
}

type ChannelState = HashMap<Uuid, UserSession>;
#[derive(Default)]
pub struct WsServer {
    channels: HashMap<String, ChannelState>,
}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<GetChannelSubscribers>(ctx);
        self.subscribe_async::<LeaveChannel>(ctx);

        self.subscribe_async::<SendMsg>(ctx);
        self.subscribe_async::<SendErrorMsg>(ctx);
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}

#[derive(Clone, Message)]
pub struct GetChannelSubscribers {
    id: Uuid,
    channel: String,
    client: Client,
}

impl GetChannelSubscribers {
    pub fn new(id: Uuid, channel: String, client: Client) -> Self {
        Self { id, channel, client }
    }
}

impl Handler<GetChannelSubscribers> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: GetChannelSubscribers, _ctx: &mut Self::Context) -> Self::Result {
        debug!("received request for joining channel");

        let error_msg = json!({
            "channel": msg.channel,
            "error": "User is not subscribed to this channel",
        });
        let publish_msg = match self.channels.get(&msg.channel) {
            None => { error_msg },
            Some(channel_state) => {
                let users: Vec<_> = channel_state
                    .values()
                    .map(|x| x.user.get_username())
                    .collect();

                match channel_state.get(&msg.id) {
                    None => { error_msg },
                    Some(current_user_state) => {
                        json!({
                            "channel": msg.channel,
                            "users": users
                        })
                    },
                }
            },
        };

        let client = msg.client;
        client.do_send(Notification::new(publish_msg)).is_ok();
    }
}

#[derive(Clone, Message)]
pub struct LeaveChannel {
    id: Uuid,
    channel: String,
}

impl LeaveChannel {
    pub fn new(id: Uuid, channel: String) -> Self {
        Self { id, channel }
    }
}

impl Handler<LeaveChannel> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveChannel, _ctx: &mut Self::Context) -> Self::Result {
        let channel_name = msg.channel;
        let id = msg.id;

        match self.channels.get_mut(&channel_name) {
            Some(channel_state) => {
                info!("Removing user [{:?}] from channel {:?}", &id, &channel_name);
                channel_state.remove(&id);
            },
            None => {
                warn!("Could not find channel: {:?}", &channel_name);
            }
        }
    }
}

#[derive(Clone, Message)]
pub struct SendErrorMsg {
    id: Uuid,
    error_result: ApiErrorResponse,
    client: Client,
}

impl SendErrorMsg {
    pub fn new(id: Uuid, error_result: ApiErrorResponse, client: Client) -> Self {
        Self { id, error_result, client }
    }
}

impl Handler<SendErrorMsg> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: SendErrorMsg, _ctx: &mut Self::Context) -> Self::Result {
        let client = msg.client;
        serde_json::to_value(msg.error_result)
            .and_then(|publish_msg| {
                warn!("publishing error msg: {:?}", &publish_msg);
                client.do_send(Notification::new(publish_msg)).is_ok();

                Ok(())
            });
    }
}

#[derive(Clone, Message)]
pub struct SendMsg {
    id: Uuid,
    user: UserData,
    api_result: ApiOkResponse,
    client: Client,
}

impl SendMsg {
    pub fn new(id: Uuid, user: UserData, api_result: ApiOkResponse, client: Client) -> Self {
        Self { id, user, api_result, client }
    }
}

fn manage_new_subscriptions(server: &mut WsServer, subscribe_to: Vec<String>, id: &Uuid, client: &Client, user: &UserData) {
    for channel_name in subscribe_to {
        let user_session = UserSession::new(client.to_owned(), user.to_owned());
        info!("Adding user to channel: \"{:?}\" user {:?}", &channel_name, &id);
        server.channels.entry(channel_name)
            .or_insert_with(|| HashMap::new())
            .insert(id.to_owned(), user_session);
    }
}

fn get_relevant_clients_for_publishing(server: &mut WsServer, publish_to: Vec<String>, id: &Uuid, client: &Client) -> HashMap<Uuid, Client> {
    let mut relevant_clients = HashMap::new();
    relevant_clients.insert(id.to_owned(), client.to_owned());

    for channel_name in publish_to {
        match server.channels.get(&channel_name) {
            Some(channel_state) => {
                for (id, user_session) in channel_state {
                    relevant_clients.insert(id.to_owned(), user_session.client.to_owned());
                }
            },
            None => {}
        }
    }

    relevant_clients
}

fn handle_pub_sub_for_server(server: &mut WsServer, msg: SendMsg) -> HashMap<Uuid, Client> {
    let subscribe_to = msg.api_result.get_channels_to_subscribe_to();
    let publish_to = msg.api_result.get_channels_to_publish_to();
    let client = msg.client;
    let user = msg.user;

    // subscribe user to these channels
    manage_new_subscriptions(server, subscribe_to, &msg.id, &client, &user);

    // publish data to these channels
    get_relevant_clients_for_publishing(server, publish_to, &msg.id, &client)
}

impl Handler<SendMsg> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: SendMsg, _ctx: &mut Self::Context) -> Self::Result {
        let publish_msg = json!({
            "action": msg.api_result.get_action(),
            "data": msg.api_result.get_data(),
        });

        let relevant_clients = handle_pub_sub_for_server(self, msg);

        for (id, client) in relevant_clients {
            client.do_send(Notification::new(publish_msg.to_owned())).is_ok();
        }
    }
}

