
use actix_web::{http, test, App, HttpRequest, HttpResponse};
use actix_web::test::TestServer;

use std::sync::Arc;

use actix_web::test::TestServerBuilder;
use actix_web::HttpMessage;
use std::str;
use actix_web::http::Method;
use log::LevelFilter;
use env_logger::{Builder, Target};
use actix_web::http::header;
use uuid::Uuid;
use actix_web::client::ClientResponse;

use super::KakapoState;
use super::KakapoStateBuilder;
use super::GetKakapoState;
use super::KakapoBroadcaster;
use super::Channels;
use super::BroadcasterError;
use super::KakapoRouter;

pub fn random_identifier() -> String {
    let uuid = Uuid::new_v4();
    let res = base64::encode_config(&uuid.as_bytes(), base64::STANDARD_NO_PAD);

    res.replace("/", "_").replace("+", "$")

}

pub fn init_logger() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Warn)
        .filter_module("kakapo", LevelFilter::Debug)
        .filter_module("actix_web", LevelFilter::Info)
        .init();
}

pub fn build_server() -> TestServer {
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

pub fn send_message(server: &mut TestServer, endpoint: &str, json_request: &serde_json::Value) -> (ClientResponse, serde_json::Value) {
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

pub fn print_response(response: &ClientResponse, body: &serde_json::Value) {
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

pub const TEST_KEY: &'static str = "TEST_SECRET_TEST_SECRET";
pub const MASTER_KEY_TOKEN: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjkyMjMzNzIwMzY4NTQ3NzU4MDcsImlhdCI6MCwiaXNBZG1pbiI6dHJ1ZSwiaXNzIjoidGVzdCIsInJvbGUiOm51bGwsInN1YiI6MSwidXNlcm5hbWUiOiJBZG1pblRlc3QifQ.pgSE-K4RTaWMhVfny2LwUp3f0TEHS6y-vciDcH1c2y8";


#[derive(Debug)]
pub struct TestState(KakapoState);
#[derive(Debug)]
pub struct TestBroadcaster;

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String
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