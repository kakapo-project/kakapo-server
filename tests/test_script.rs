
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


#[test]
fn test_create_and_read_script() {

    init_logger();

    let mut server = build_server();
    let id = random_identifier();
    let script_name = format!("my_script_{}", id);


    let json_request = json!({
        "name": script_name,
        "description": "blah blah blah",
        "text": "print('hey world')"
    });
    let endpoint = "/manage/createScript";

    let (response, body) = send_message(&mut server, endpoint, &json_request);

    assert_eq!(body["result"], json!("created"));
    assert_eq!(body["new"]["name"], json!(script_name));


    let endpoint = "/manage/getAllScripts";
    let (response, body) = send_message(&mut server, endpoint, &json!({}));
    let result_set = body.as_array().unwrap();

    let fin_res = result_set.iter().find(|x| x["name"].as_str().unwrap() == &script_name).unwrap();

    assert_eq!(fin_res["name"], json!(script_name));
    assert_eq!(fin_res["description"], json!("blah blah blah"));
    assert_eq!(fin_res["text"], json!("print('hey world')"));


}
