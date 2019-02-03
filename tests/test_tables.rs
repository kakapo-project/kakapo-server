extern crate kakapo_api;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate env_logger;
extern crate log;

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

#[derive(Debug)]
struct TestState(KakapoState);
#[derive(Debug)]
struct TestBroadcaster;

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


#[test]
fn test_start_server() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Warn)
        .filter_module("kakapo", LevelFilter::Debug)
        .filter_module("actix_web", LevelFilter::Info)
        .init();

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

    let mut server = server_builder
        .start(|app| {
            app.kakapo_routes();
        });


    let json_request = json!({
        "name": "my_query",
        "description": "blah blah blah",
        "statement": "SELECT * FROM a_table"
    });
    let request = server
        .client(Method::POST, "/manage/createQuery")
        .header(header::AUTHORIZATION, MASTER_KEY_TOKEN)
        .json(json_request)
        .unwrap();
    let response = server.execute(request.send()).unwrap();
    println!("response: {:?}", &response);

    let bytes = server.execute(response.body()).unwrap();
    let body = str::from_utf8(&bytes).unwrap();
    println!("{:?}", &body);

}