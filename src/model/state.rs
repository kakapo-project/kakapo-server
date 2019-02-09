
use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use connection::executor::Conn;
use diesel::Connection;
use model::entity::RawEntityTypes;
use scripting::Scripting;
use connection::Broadcaster;
use std::sync::Arc;
use serde::Serialize;
use model::actions::error::Error;
use std::fmt::Debug;
use std::fmt;
use connection::executor::Secrets;
use model::auth::auth_modifier::AuthFunctions;
use model::auth::auth_modifier::Auth;
use model::entity::Controller;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::table::TableAction;
use model::table::TableActionFunctions;
use std::marker::PhantomData;

pub struct ActionState {
    pub database: Conn, //TODO: this should be templated
    pub scripting: Scripting,
    pub claims: Option<AuthClaims>,
    pub broadcaster: Arc<Broadcaster>,
    pub secrets: Secrets,
}

pub trait StateFunctions<'a>
    where
        Self: Debug,
        Self::AuthFunctions: AuthFunctions,
        Self::EntityRetrieverFunctions: RetrieverFunctions,
        Self::EntityModifierFunctions: ModifierFunctions,
        Self::TableController: TableActionFunctions,
{
    type AuthFunctions;
    fn get_auth_functions(&'a self) -> Self::AuthFunctions;

    type EntityRetrieverFunctions;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions;

    type EntityModifierFunctions;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions;

    type TableController;
    fn get_table_controller(&'a self) -> Self::TableController;
}

impl<'a> StateFunctions<'a> for ActionState {
    type AuthFunctions = Auth<'a>;
    fn get_auth_functions(&'a self) -> Auth<'a> {
        let password_secret = self.get_password_secret();
        Auth::new(
            self.get_conn(),
            password_secret.to_owned(),
        )
    }

    type EntityRetrieverFunctions = Controller<'a>;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions {
        Controller {
            conn: &self.database,
            claims: &self.claims,
        }
    }

    type EntityModifierFunctions = Controller<'a>;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions {
        Controller {
            conn: &self.database,
            claims: &self.claims,
        }
    }

    type TableController = TableAction<'a>;
    fn get_table_controller(&'a self) -> Self::TableController {
        TableAction {
            conn: &self.database,
        }
    }
}


pub trait GetConnection
    where Self: Send + Debug
{
    type Connection;
    fn get_conn(&self) -> &Self::Connection;

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where F: FnOnce() -> Result<G, E>, E: From<diesel::result::Error>;
}

impl GetConnection for ActionState {
    type Connection = Conn;
    fn get_conn(&self) -> &Conn {
        &self.database
    }

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
        where F: FnOnce() -> Result<G, E>, E: From<diesel::result::Error> {
        let conn = self.get_conn();
        conn.transaction::<G, E, _>(f)
    }
}



/// OLD

#[derive(Debug, Clone, Serialize)]
pub enum Channels {
    AllTables,
    AllQueries,
    AllScripts,
    Table(String),
    Query(String),
    Script(String),
    TableData(String),
}

impl Channels {
    pub fn all_entities<T>() -> Self
        where T: RawEntityTypes,
    {
        Channels::AllTables
    }

    pub fn entity<T>(name: &str) -> Self
        where T: RawEntityTypes,
    {
        Channels::Table(name.to_string())
    }

    pub fn table(table_name: &str) -> Self {
        Channels::TableData(table_name.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthClaims {
    iss: String,
    sub: i64, // == user_id
    iat: i64,
    exp: i64,
    username: String,
    is_admin: bool,
    role: Option<String>, //the default role that the user is interacting with
}

impl AuthClaims {
    pub fn get_user_id(&self) -> i64 {
        self.sub
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }

    pub fn is_user_admin(&self) -> bool {
        self.is_admin
    }
}

impl fmt::Debug for ActionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State")
    }
}

impl ActionState {
    //TODO: this has too many parameters
    pub fn new(
        database: Conn,
        scripting: Scripting,
        claims: Option<AuthClaims>,
        broadcaster: Arc<Broadcaster>,
        secrets: Secrets,
    ) -> Self {
        Self {
            database,
            scripting,
            claims,
            broadcaster,
            secrets,
        }
    }
}

pub trait GetScripting
    where Self: Send + Debug
{
    fn get_scripting(&self) -> &Scripting;
}

impl GetScripting for ActionState {
    fn get_scripting(&self) -> &Scripting {
        &self.scripting
    }
}

pub trait GetBroadcaster
    where Self: Send + Debug
{
    fn publish<R>(&self, channels: Vec<Channels>, action_name: String, action_result: &R) -> Result<(), Error>
        where R: Serialize;
}

impl GetBroadcaster for ActionState {
    fn publish<R>(&self, channels: Vec<Channels>, action_name: String, action_result: &R) -> Result<(), Error>
        where R: Serialize
    {
        let payload = serde_json::to_value(action_result)
            .or_else(|err| Err(Error::SerializationError(err.to_string())))?;


        self.broadcaster.publish(channels, action_name, payload)
            .or_else(|err| Err(Error::PublishError(err)))?;

        Ok(())
    }
}

pub trait GetSecrets
    where Self: Send + Debug
{
    fn get_token_secret(&self) -> String;
    fn get_password_secret(&self) -> String;
}

impl GetSecrets for ActionState {
    fn get_token_secret(&self) -> String {
        self.secrets.token_secret.to_owned()
    }

    fn get_password_secret(&self) -> String {
        self.secrets.password_secret.to_owned()

    }
}
