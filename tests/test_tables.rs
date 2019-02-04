
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
fn test_create_query() {

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

#[test]
fn test_create_table() {

    let mut server = build_server();
    let id = random_identifier();
    let table_name = format!("my_table_{}", id);


    let json_request = json!({
        "name": table_name,
        "description": "table description",
        "schema": {
            "columns": [
                {
                    "name": "col_a",
                    "dataType": "integer"
                },
                {
                    "name": "col_b",
                    "dataType": "integer"
                }
            ],
            "constraint": [
            ]
        }
    });

    // make a new table
    let endpoint = "/manage/createTable";

    let (response, body) = send_message(&mut server, endpoint, &json_request);

    println!("HEADER: {:?} BODY: \n{}", &response, serde_json::to_string_pretty(&body).unwrap());

    assert_eq!(body["result"], json!("created"));
    assert_eq!(body["new"]["name"], json!(table_name));

    let columns: Vec<Column> = serde_json::from_value(body["new"]["schema"]["columns"].to_owned()).unwrap();
    let column_names: Vec<String> = columns.iter().map(|x| x.name.to_owned()).collect();
    assert!(column_names.contains(&"col_a".to_string()));
    assert!(column_names.contains(&"col_b".to_string()));


    // now delete the table
    let endpoint = format!("/manage/deleteTable?name={}", table_name);

    let (response, body) = send_message(&mut server, &endpoint, &json!({}));

    assert_eq!(body["result"], json!("deleted"));
    assert_eq!(body["old"]["name"], json!(table_name));

}

#[test]
fn test_create_table_and_read() {

}

#[test]
fn test_create_table_and_update() {

}

#[test]
fn test_create_table_twice() {

}

#[test]
fn test_update_nonexistant_table() {

}

#[test]
fn test_delete_nonexistant_table() {

}

#[test]
fn test_create_badly_formatted_tables_should_fail() {

}