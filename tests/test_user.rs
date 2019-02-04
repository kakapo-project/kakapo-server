
extern crate actix;
extern crate actix_broker;
extern crate actix_web;
extern crate base64;
extern crate bcrypt;
extern crate bigdecimal;
extern crate byteorder;
extern crate bytes;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate json;
extern crate jsonwebtoken;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate openssl;
extern crate tokio;
extern crate tokio_core;
extern crate uuid;

extern crate kakapo_api;

use kakapo_api::KakapoState;
use kakapo_api::KakapoStateBuilder;
use kakapo_api::GetKakapoState;
use kakapo_api::KakapoBroadcaster;
use kakapo_api::Channels;
use kakapo_api::BroadcasterError;
use kakapo_api::KakapoRouter;
use actix_web::test::TestServerBuilder;
use actix_web::HttpMessage;
use actix_web::http::Method;
use actix_web::http::header;
use actix_web::client::ClientResponse;

use std::sync::Arc;
use std::str;
use kakapo_api::test_common::*;


#[test]
fn test_authenticate_username() {
    let username = "AzureDiamon";
    let password = "Hunter2";
}

#[test]
fn test_authenticate_email() {
    let email = "chunkylover53@aol.com";
    let password = "PressAnyKey";
}

#[test]
fn test_authenticate_username_failed() {
    let username = "AzureDiamon";
    let password = "Hunter2";
}

#[test]
fn test_authenticate_email_failed() {
    let email = "chunkylover53@aol.com";
    let password = "PressAnyKey";
}