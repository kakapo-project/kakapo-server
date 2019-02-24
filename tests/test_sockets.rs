
extern crate actix;
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

use kakapo_api::AppState;
use kakapo_api::AppStateLike;
use actix_web::test::TestServerBuilder;
use actix_web::HttpMessage;
use actix_web::http::Method;
use actix_web::http::header;
use actix_web::client::ClientResponse;

use std::sync::Arc;
use std::str;
use kakapo_api::test_common::*;
use actix_web::ws::Message::Text;
use actix_web::ws;

trait AsText {
    fn as_value(self) -> serde_json::Value;
}

impl AsText for ws::Message {
    fn as_value(self) -> serde_json::Value {
        if let Text(final_msg) = self {
            let final_msg: serde_json::Value = serde_json::from_str(&final_msg).unwrap();
            return final_msg
        } else {
            panic!("Expected a text message");
        }
    }
}


#[test]
fn test_sanity_websocket_connection() {

    let mut server = build_server();
    let id = random_identifier();

    let query_name = format!("my_query_{}", id);
    let json_request = json!({
        "action": "call",
        "procedure": "createQuery",
        "params": {},
        "data": {
            "name": query_name,
            "description": "blah blah blah",
            "statement": "SELECT * FROM a_table"
        }
    });


    let (message, _reader, _writer) = send_new_ws_message(&mut server, &json_request);

    assert_eq!(message.as_value(), json!({"error": "Not authorized"}))
}

#[test]
fn test_websocket_connection_outdated_token() {

    let mut server = build_server();
    let id = random_identifier();

    let json_request = json!({
        "action": "authenticate",
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJ0ZXN0Iiwic3ViIjoxLCJpYXQiOjAsImV4cCI6MTI2MjM0NzIwMCwidXNlcm5hbWUiOiJBZG1pblRlc3QiLCJpc0FkbWluIjp0cnVlLCJyb2xlIjpudWxsfQ.ajRQEvl948E_4UgNd4OMGLzpU38fizzNIfH_8U_micQ",
    }); //The token here is an outdated token, similar permissions as the master token, but expired in 2010


    let (message, _reader, _writer) = send_new_ws_message(&mut server, &json_request);

    assert_eq!(message.as_value(), json!({"error": "Could not authenticate token"}))
}

#[test]
fn test_websocket_connection() {

    let mut server = build_server();
    let id = random_identifier();

    let json_request = json!({
        "action": "authenticate",
        "token": MASTER_KEY_TOKEN_RAW,
    });


    let (message, mut reader, mut writer) = send_new_ws_message(&mut server, &json_request);

    assert_eq!(message.as_value(), json!("authenticated"));


    let query_name = format!("my_query_{}", id);
    let json_request = json!({
        "action": "call",
        "procedure": "createQuery",
        "params": {},
        "data": {
            "name": query_name,
            "description": "blah blah blah",
            "statement": "SELECT * FROM a_table"
        }
    });


    let message = send_ws_message(&mut writer, &mut reader, &mut server, &json_request);

    println!("final mesg: {:?}", &message.as_value());
}

#[test]
fn test_websocket_flow() {
    let mut server = build_server();
    let id = random_identifier();
    let query_name = format!("my_query_{}", id);

    let json_request = json!({
        "action": "authenticate",
        "token": MASTER_KEY_TOKEN_RAW,
    });

    let (message1, mut reader1, mut writer1) = send_new_ws_message(&mut server, &json_request);
    let (message2, mut reader2, mut writer2) = send_new_ws_message(&mut server, &json_request);


    let json_request = json!({
        "action": "call",
        "procedure": "subscribeTo",
        "params": {
            "username": "admin",
        },
        "data": {
            "query": query_name.to_owned()
        }
    });

    let message= send_ws_message(&mut writer1, &mut reader1, &mut server, &json_request);
    assert_eq!(message.as_value()["type"].to_owned(), json!("subscribed"));


    let json_request = json!({
        "action": "call",
        "procedure": "createQuery",
        "params": {},
        "data": {
            "name": query_name,
            "description": "blah blah blah",
            "statement": "SELECT * FROM a_table"
        }
    });

    let message1= send_ws_message(&mut writer2, &mut reader2, &mut server, &json_request);
    println!("msg1: {:?}", &message1);
    let message2 = ws_message_from_reader(&mut reader1, &mut server);
    println!("msg2: {:?}", &message2);
}