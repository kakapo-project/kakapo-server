extern crate kakapo_api;
extern crate actix_web;

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


struct TestState(KakapoState);
struct TestBroadcaster;

impl KakapoBroadcaster for TestBroadcaster {
    fn publish(
        &self,
        channels: Vec<Channels>,
        action_name: String,
        action_result: serde_json::Value,
    ) -> Result<(), BroadcasterError> {
        unimplemented!()
    }
}

impl GetKakapoState<TestBroadcaster> for TestState {
    fn get_app_state(&self) -> &KakapoState {
        unimplemented!()
    }

    fn get_broadcaster(&self) -> Arc<KakapoBroadcaster> {
        unimplemented!()
    }
}

#[test]
fn test_start_server() {
    let server_builder: TestServerBuilder<TestState, _> = TestServer::build_with_state(|| {
        let state = KakapoStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .db("test")
            .script_path("./local")
            .token_secret("TEST_SECRET_TEST_SECRET")
            .num_threads(1)
            .done();

        TestState(state)
    });

    let mut server = server_builder
        .start(|app| {
            app.kakapo_routes();
        });

}