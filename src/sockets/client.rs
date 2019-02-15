
use actix::prelude::*;

use actix_web::{
    App, AsyncResponder, Error, dev::JsonConfig,
    http, http::NormalizePath, http::Method,
    HttpMessage, middleware, HttpRequest, HttpResponse,
    fs, fs::NamedFile,
    ResponseError, State, ws,
};

use actix_web::middleware::cors::Cors;
use actix_web::middleware::Logger;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use chrono::Duration;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::path::Path as fsPath;

use state::AppState;
use actix_web::Path;
use actix_web::Responder;

use futures::Future;
use actix_web::client;

use actix_web::Json;

use actix_broker::BrokerIssue;

use sockets::server::WsServer;
use sockets::server::GetChannelSubscribers;
use sockets::server::LeaveChannel;
use sockets::server::SendMsg;
use sockets::server::SendErrorMsg;

use state::error;
use state::api::ApiResult;
use state::api::Channel;

use uuid::Uuid;
use state::api::UserData;

use jsonwebtoken as jwt;

use state::error::Error::TooManyConnections;
use sockets::Notification;
use state::JwtConfig;

use state::GetEndpoint;

#[derive(Clone, Debug)]
pub struct WsClientSession {
    id: Uuid,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
enum WsInputData {
    #[serde(rename_all = "camelCase")]
    GetSubscribers {
        channel: String,
    },
    #[serde(rename_all = "camelCase")]
    Unsubscribe {
        channel: String,
    },
    #[serde(rename_all = "camelCase")]
    Call {
        // WARNING: while calling functions require authentication,
        // If the role changes while the user is connected
        // everything that has already been subscribed remains
        // This is a security issue and needs to be solved
        auth: String,
        function: String,
        params: serde_json::Value,
        data: serde_json::Value,
        #[serde(default)]
        subscribe_after: bool,
    },
}

impl WsClientSession {

    pub fn new() -> Self {
        let id = Uuid::new_v4();
        Self { id }
    }

    fn get_subscribers(
        &mut self,
        ctx: &mut ws::WebsocketContext<Self, AppState>,
        channel_name: String,
    ) {
        let get_subscribers = GetChannelSubscribers::new(
            self.id,
            channel_name,
            ctx.address().recipient(),
        );

        WsServer::from_registry()
            .send(get_subscribers.to_owned())
            .into_actor(self)
            .then(|res, act, _ctx| {
                info!("Got server from registry");
                fut::ok(())
            }).spawn(ctx);

        self.issue_sync(get_subscribers, ctx);

    }

    fn unsubscribe_from_channel(
        &mut self,
        ctx: &mut ws::WebsocketContext<Self, AppState>,
        channel_name: String,
    ) {
        let leave = LeaveChannel::new(
            self.id,
            channel_name,
        );

        WsServer::from_registry()
            .send(leave.to_owned())
            .into_actor(self)
            .then(|res, act, _ctx| {
                info!("Got server from registry");
                fut::ok(())
            }).spawn(ctx);

        self.issue_sync(leave, ctx);
    }

    fn process_procedure_result(
        &mut self,
        ctx: &mut ws::WebsocketContext<Self, AppState>,
        user: UserData,
        raw_bytes: &[u8],
    ) -> Result<(), error::Error> {
        let api_result = ApiResult::parse_result(raw_bytes)
            .or_else(|err| {
                Err(err)
            })?;

        match api_result {
            ApiResult::Ok(res) => {
                debug!("received ok message \"{:?}\"", &res);
                let recipient = ctx.address().recipient();
                let send_msg = SendMsg::new(self.id, user, res, recipient);

                WsServer::from_registry()
                    .send(send_msg.to_owned())
                    .into_actor(self)
                    .then(|res, act, _ctx| {
                        info!("Got server from registry");
                        fut::ok(())
                    }).spawn(ctx);
                self.issue_sync(send_msg, ctx);
            },
            ApiResult::Err(err) => {
                debug!("received err message \"{:?}\"", &err);
                let recipient = ctx.address().recipient();
                let send_msg = SendErrorMsg::new(self.id, err, recipient);

                WsServer::from_registry()
                    .send(send_msg.to_owned())
                    .into_actor(self)
                    .then(|res, act, _ctx| {
                        info!("Got server from registry");
                        fut::ok(())
                    }).spawn(ctx);
                self.issue_sync(send_msg, ctx);
            },
        }

        Ok(())
    }

    fn call_procedure(
        &mut self,
        ctx: &mut ws::WebsocketContext<Self, AppState>,
        auth: String,
        function: String,
        params: serde_json::Value,
        data: serde_json::Value,
    ) -> Result<(), String> {
        let secret = ctx.state().get_secret_key();
        let user = jwt::decode::<UserData>(&auth, secret.as_ref(), &jwt::Validation::default())
            .and_then(|token_data| Ok(token_data.claims))
            .or_else(|err| {
                warn!("Could not parse token: {:?}", &err);
                Err("Could not parse token data".to_string())
            })?;

        let endpoint = ctx.state().get_endpoint();
        let function_endpoint = format!("{}/{}", endpoint, function);
        debug!("calling endpoint: {:?}", &function_endpoint);

        let _ = client::ClientRequest::post(function_endpoint) //TODO: params, auth
            .json(data)
            .unwrap_or_default()
            .send()
            .wait()
            .or_else(|err| Err(TooManyConnections))
            .and_then(|resp| {
                debug!("msg response: {:?}", &resp);

                resp.body()
                    .and_then(|body| {
                        self.process_procedure_result(ctx, user, &body)
                            .or_else(|err| {
                                debug!("encountered error: {:?}", &err);
                                Ok(()) //the error is handled in the process function
                            })
                    })
                    .wait();

                Ok(())
            });
        Ok(())

    }

    fn handle_message(&mut self, ctx: &mut ws::WebsocketContext<Self, AppState>, input: WsInputData) {
        match input {
            WsInputData::GetSubscribers { channel } => {
                self.get_subscribers(ctx, channel);
            },
            WsInputData::Unsubscribe { channel } => {
                self.unsubscribe_from_channel(ctx, channel);
            },
            WsInputData::Call { auth, function, params, data } => {
                self.call_procedure(ctx, auth, function, params, data);
            },
        }
    }
}


impl Actor for WsClientSession {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsSession[{:?}] opened ", &self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("WsSession[{:?}] closed ", &self.id);
    }
}


impl Handler<Notification> for WsClientSession {
    type Result = ();

    fn handle(&mut self, notification: Notification, ctx: &mut Self::Context) {
        let data = notification.get_data();
        let _ = serde_json::to_string(&data)
            .and_then(|res| {
                ctx.text(res);
                Ok(())
            })
            .or_else(|err| {
                error!("Could not parse message for notifications: {:?}", &err);
                Err(err)
            });

    }
}


impl StreamHandler<ws::Message, ws::ProtocolError> for WsClientSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        debug!("received msg \"{:?}\"", msg);
        match msg {
            ws::Message::Text(text) => {
                let _ = serde_json::from_str(&text)
                    .or_else(|err| {
                        debug!("could not understand incoming message, must be `WsInputData`");
                        Err(())
                    })
                    .and_then(move |res: WsInputData| {
                        self.handle_message(ctx, res);
                        Ok(())
                    });

            },
            ws::Message::Close(_) => {
                ctx.stop();
            },
            ws::Message::Binary(_) => {
                info!("binary websocket messages not currently supported");
            },
            ws::Message::Ping(_) => {

            },
            ws::Message::Pong(_) => {

            },
        }
    }
}
