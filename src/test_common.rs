
use actix_web::test::TestServer;

use std::sync::Arc;
use std::path::PathBuf;

use actix_web::test::TestServerBuilder;
use actix_web::HttpMessage;
use std::str;
use actix_web::http::Method;
use log::LevelFilter;
use env_logger::{Builder, Target};
use actix_web::http::header;
use uuid::Uuid;
use actix_web::client::ClientResponse;

use super::AppState as KakapoState;
use super::AppStateBuilder as KakapoStateBuilder;
use state::ActionState;
use state::StateFunctions;
use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use data::claims::AuthClaims;
use connection::executor::Secrets;
use scripting::Scripting;
use serde::Serialize;
use data::auth::InvitationToken;
use data::auth::Invitation;
use auth::send_mail::EmailError;
use diesel::r2d2::Pool;
use model::actions;
use diesel::Connection;
use auth::send_mail::EmailOps;
use connection::AppStateLike;
use actix::Addr;
use connection::executor::Executor;
use state::PubSubOps;
use data::channels::Channels;
use view::extensions::ProcedureExt;
use actix_web::ws::ClientReader;
use actix_web::ws::ClientWriter;
use futures::Stream;
use actix_web::ws::Message;
use connection::GetSecrets;
use state::error::BroadcastError;
use connection::executor::DomainError;


pub fn random_identifier() -> String {
    let uuid = Uuid::new_v4();
    let res = base64::encode_config(&uuid.as_bytes(), base64::STANDARD_NO_PAD);

    res.replace("/", "_").replace("+", "$")

}

// integration tests

pub fn init_logger() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Warn)
        .filter_module("kakapo", LevelFilter::Debug)
        .filter_module("actix_web", LevelFilter::Info)
        .init();
}

pub fn build_server() -> TestServer {

    let server_builder: TestServerBuilder<TestState, _> = TestServer::build_with_state(move || {

        let mut state = KakapoStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .db("test")
            .script_path("./local")
            .token_secret(TEST_KEY)
            .password_secret(TEST_KEY)
            .issuer("THE_ISSUER")
            .token_duration(600)
            .refresh_token_duration(60 * 60 * 24 * 7)
            .num_threads(1)
            .done();

        TestState(Box::new(state))
    });

    server_builder
        .start(move |app| {
            app.add_routes();
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

pub fn send_new_ws_message(server: &mut TestServer, json_request: &serde_json::Value) -> (Message, ClientReader, ClientWriter) {
    let (reader, mut writer) = server
        .ws_at("/listen")
        .unwrap();
    let json_str: String = serde_json::to_string(json_request).unwrap();
    writer.text(json_str);

    let fut = reader.into_future();
    let res = server.execute(fut).unwrap();

    (res.0.unwrap(), res.1, writer)
}

pub fn send_ws_message(writer: &mut ClientWriter, reader: &mut ClientReader, server: &mut TestServer, json_request: &serde_json::Value) -> Message {

    let json_str: String = serde_json::to_string(json_request).unwrap();
    writer.text(json_str);

    let fut = reader.into_future();
    let res = server.execute(fut).unwrap();

    res.0.unwrap()
}

pub fn ws_message_from_reader(reader: &mut ClientReader, server: &mut TestServer) -> Message {
    let fut = reader.into_future();
    let res = server.execute(fut).unwrap();

    res.0.unwrap()
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
pub const MASTER_KEY_TOKEN_RAW: &'static str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjkyMjMzNzIwMzY4NTQ3NzU4MDcsImlhdCI6MCwiaXNBZG1pbiI6dHJ1ZSwiaXNzIjoidGVzdCIsInJvbGUiOm51bGwsInN1YiI6MSwidXNlcm5hbWUiOiJBZG1pblRlc3QifQ.pgSE-K4RTaWMhVfny2LwUp3f0TEHS6y-vciDcH1c2y8";
pub const MASTER_KEY_TOKEN: &'static str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjkyMjMzNzIwMzY4NTQ3NzU4MDcsImlhdCI6MCwiaXNBZG1pbiI6dHJ1ZSwiaXNzIjoidGVzdCIsInJvbGUiOm51bGwsInN1YiI6MSwidXNlcm5hbWUiOiJBZG1pblRlc3QifQ.pgSE-K4RTaWMhVfny2LwUp3f0TEHS6y-vciDcH1c2y8";


#[derive(Clone, Debug)]
pub struct TestState(Box<KakapoState>);


impl AppStateLike for TestState {
    fn connect(&self) -> &Addr<Executor> {
        self.0.connect()
    }
}

impl GetSecrets for TestState {
    fn get_token_secret(&self) -> String {
        self.0.get_token_secret()
    }

    fn get_password_secret(&self) -> String {
        self.0.get_password_secret()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String
}

// unit tests

pub struct MockMailer;

impl EmailOps for MockMailer {
    fn send_email(&self, invitation_token: InvitationToken) -> Result<Invitation, EmailError> {

        Ok(Invitation {
            email: invitation_token.email,
            expires_at: invitation_token.expires_at,
        })
    }
}

#[derive(Debug)]
pub struct MockState(pub ActionState);
impl<'a> StateFunctions<'a> for MockState {
    type Authentication = <ActionState as StateFunctions<'a>>::Authentication;
    fn get_authentication(&'a self) -> <Self as StateFunctions<'a>>::Authentication {
        self.0.get_authentication()
    }

    type Authorization = <ActionState as StateFunctions<'a>>::Authorization;
    fn get_authorization(&'a self) -> <Self as StateFunctions<'a>>::Authorization {
        self.0.get_authorization()
    }

    type UserManagement = <ActionState as StateFunctions<'a>>::UserManagement;
    fn get_user_management(&'a self) -> <Self as StateFunctions<'a>>::UserManagement {
        self.0.get_user_management()
    }

    type DomainManagement = <ActionState as StateFunctions<'a>>::DomainManagement;
    fn get_domain_management(&'a self) -> <Self as StateFunctions<'a>>::DomainManagement {
        self.0.get_domain_management()
    }


    type EntityRetrieverFunctions = <ActionState as StateFunctions<'a>>::EntityRetrieverFunctions;
    fn get_entity_retreiver_functions(&'a self) -> <Self as StateFunctions<'a>>::EntityRetrieverFunctions {
        self.0.get_entity_retreiver_functions()
    }

    type EntityModifierFunctions = <ActionState as StateFunctions<'a>>::EntityModifierFunctions;
    fn get_entity_modifier_function(&'a self) -> <Self as StateFunctions<'a>>::EntityModifierFunctions {
        self.0.get_entity_modifier_function()
    }

    type TableController = <ActionState as StateFunctions<'a>>::TableController;
    fn get_table_controller(&'a self) -> <Self as StateFunctions<'a>>::TableController {
        self.0.get_table_controller()
    }

    type QueryController = <ActionState as StateFunctions<'a>>::QueryController;
    fn get_query_controller(&'a self) -> <Self as StateFunctions<'a>>::QueryController {
        self.0.get_query_controller()
    }

    type Scripting = <ActionState as StateFunctions<'a>>::Scripting;
    fn get_script_runner(&'a self) -> <Self as StateFunctions<'a>>::Scripting {
        self.0.get_script_runner()
    }

    type Database = <ActionState as StateFunctions<'a>>::Database;
    fn get_database(&'a self) -> <Self as StateFunctions<'a>>::Database {
        self.0.get_database()
    }

    type EmailSender = MockMailer;
    fn get_email_sender(&'a self) -> MockMailer {
        MockMailer
    }

    type PubSub = <ActionState as StateFunctions<'a>>::PubSub;
    fn get_pub_sub(&'a self) -> Self::PubSub {
        self.0.get_pub_sub()
    }

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where
            F: FnOnce() -> Result<G, E>,
            E: From<diesel::result::Error>
    {
        self.0.transaction(f)
    }
}

impl GetSecrets for MockState {
    fn get_token_secret(&self) -> String { self.0.get_token_secret() }

    fn get_password_secret(&self) -> String { self.0.get_password_secret() }
}

pub fn with_state<F>(f: F)
    where F: FnOnce(&MockState) -> ()
{
    let script_path = PathBuf::from("./target/path/to/scripts");
    let conn_url = "postgres://test:password@localhost:5432/test".to_string();
    let conn_manager: ConnectionManager<PgConnection> = ConnectionManager::new(conn_url);
    let pool = Pool::new(conn_manager).unwrap();
    let pooled_conn = pool.get().unwrap();

    let claims_json = json!({ "iss": "https://doesntmatter.com", "sub": 1, "iat": 0, "exp": -1, "username": "Admin", "isAdmin": true, "role": null });
    let claims: AuthClaims = serde_json::from_value(claims_json).unwrap();
    let secrets = Secrets {
        token_secret: "A".to_string(),
        password_secret: "B".to_string(),
    };

    let state = ActionState::new(
        pooled_conn,
        Scripting::new(script_path),
        Some(claims),
        secrets,
        None,
        Err(DomainError::Unknown),
        Err(DomainError::Unknown),
        "THE_ISSUER".to_string(),
        500, // 10 minutes
        60 * 60 * 24 * 7,
    );

    let mock_state = MockState(state);
    let conn = &mock_state.0.database;

    conn.test_transaction::<(), diesel::result::Error, _>(|| {
        f(&mock_state);

        Ok(())
    });
}

pub fn with_state_no_transaction<F>(f: F)
    where F: FnOnce(&MockState) -> ()
{
    let script_path = PathBuf::from("./target/path/to/scripts");
    let conn_url = "postgres://test:password@localhost:5432/test".to_string();
    let conn_manager: ConnectionManager<PgConnection> = ConnectionManager::new(conn_url);
    let pool = Pool::new(conn_manager).unwrap();
    let pooled_conn = pool.get().unwrap();

    let claims_json = json!({ "iss": "https://doesntmatter.com", "sub": 1, "iat": 0, "exp": -1, "username": "Admin", "isAdmin": true, "role": null });
    let claims: AuthClaims = serde_json::from_value(claims_json).unwrap();
    let secrets = Secrets {
        token_secret: "A".to_string(),
        password_secret: "B".to_string(),
    };

    let state = ActionState::new(
        pooled_conn,
        Scripting::new(script_path),
        Some(claims),
        secrets,
        None,
        Err(DomainError::Unknown),
        Err(DomainError::Unknown),
        "THE_ISSUER".to_string(),
        500,
        60 * 60 * 24 * 7,
    );

    let mock_state = MockState(state);

    f(&mock_state);
}