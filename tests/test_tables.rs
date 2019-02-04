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


use actix_web::{http, test, App, HttpRequest, HttpResponse};
use actix_web::test::TestServer;
use kakapo_api::KakapoState;
use kakapo_api::KakapoStateBuilder;
use kakapo_api::GetKakapoState;
use std::sync::Arc;
use kakapo_api::KakapoBroadcaster;
use kakapo_api::Channels;
use kakapo_api::BroadcasterError;
use kakapo_api::KakapoRouter;
use actix_web::test::TestServerBuilder;
use actix_web::HttpMessage;
use std::str;
use actix_web::http::Method;
use log::LevelFilter;
use env_logger::{Builder, Target};
use actix_web::http::header;
use uuid::Uuid;
use actix_web::client::ClientResponse;

fn random_identifier() -> String {
    let uuid = Uuid::new_v4();
    let res = base64::encode_config(&uuid.as_bytes(), base64::STANDARD_NO_PAD);

    res.replace("/", "_").replace("+", "$")

}

fn init_logger() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Warn)
        .filter_module("kakapo", LevelFilter::Debug)
        .filter_module("actix_web", LevelFilter::Info)
        .init();
}

fn build_server() -> TestServer {
    let server_builder: TestServerBuilder<TestState, _> = TestServer::build_with_state(|| {
        let state = KakapoStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .db("test")
            .script_path("./local")
            .token_secret(TEST_KEY)
            .num_threads(1)
            .done();

        TestState(state)
    });

    server_builder
        .start(|app| {
            app.kakapo_routes();
        })
}

fn send_message(server: &mut TestServer, endpoint: &str, json_request: &serde_json::Value) -> (ClientResponse, serde_json::Value) {
    let request = server
        .client(Method::POST, endpoint)
        .header(header::AUTHORIZATION, MASTER_KEY_TOKEN)
        .json(json_request)
        .unwrap();
    let response = server.execute(request.send()).unwrap();
    let bytes = server.execute(response.body()).unwrap();
    let body_str = str::from_utf8(&bytes).unwrap();
    let body: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    //println!("HEADER: {:?} BODY: \n{}", &response, serde_json::to_string_pretty(&body).unwrap());

    (response, body)
}

fn print_response(response: &ClientResponse, body: &serde_json::Value) {
    println!("HEADER: \n{:?}\nRESPONSE: \n{}", response, serde_json::to_string_pretty(body).unwrap());
}

// equivalent to
// {
//    "iss": "test",
//    "sub": 1,
//    "iat": 0,
//    "exp": 9223372036854775807,
//    "username": "AdminTest",
//    "isAdmin": true,
//    "role": null
// }
// with key "TEST_SECRET_TEST_SECRET"

const TEST_KEY: &'static str = "TEST_SECRET_TEST_SECRET";
const MASTER_KEY_TOKEN: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjkyMjMzNzIwMzY4NTQ3NzU4MDcsImlhdCI6MCwiaXNBZG1pbiI6dHJ1ZSwiaXNzIjoidGVzdCIsInJvbGUiOm51bGwsInN1YiI6MSwidXNlcm5hbWUiOiJBZG1pblRlc3QifQ.pgSE-K4RTaWMhVfny2LwUp3f0TEHS6y-vciDcH1c2y8";


#[derive(Debug)]
struct TestState(KakapoState);
#[derive(Debug)]
struct TestBroadcaster;

#[derive(Serialize, Deserialize)]
struct Column {
    name: String
}

impl KakapoBroadcaster for TestBroadcaster {
    fn publish(
        &self,
        channels: Vec<Channels>,
        action_name: String,
        action_result: serde_json::Value,
    ) -> Result<(), BroadcasterError> {
        //do nothing
        Ok(())
    }
}

impl GetKakapoState<TestBroadcaster> for TestState {
    fn get_app_state(&self) -> &KakapoState {
        &self.0
    }

    fn get_broadcaster(&self) -> Arc<KakapoBroadcaster> {
        Arc::new(TestBroadcaster)
    }
}


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