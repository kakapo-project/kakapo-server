
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


#[test]
fn test_get_table() {
    let mut server = build_server();
    let id = random_identifier();
    let query_name = format!("my_query_{}", id);


    let json_request = json!({
        "name": query_name,
        "description": "blah blah blah",
        "statement": "SELECT * FROM a_table"
    });
    let endpoint = "/manage/createQuery";

    let (response, body) = send_message(&mut server, endpoint, &json_request);

    assert_eq!(body["result"], json!("created"));
    assert_eq!(body["new"]["name"], json!(query_name));

}