
pub mod error;
pub mod authentication;
pub mod authorization;
pub mod user_management;

use serde_json;

use std::fmt::Debug;
use std::fmt;
use std::sync::Arc;

use diesel::Connection;
use serde::Serialize;


use model::actions::error::Error;

use connection::executor::Conn;
use connection::executor::Secrets;

use model::entity::EntityRetrieverController;
use model::entity::EntityModifierController;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::table::TableAction;
use model::table::TableActionFunctions;
use auth::send_mail::EmailSender;
use auth::send_mail::EmailOps;
use model::state::authorization::AuthorizationOps;
use model::state::authentication::AuthenticationOps;
use model::state::user_management::UserManagementOps;

use scripting::ScriptFunctions;
use scripting::Scripting;

use data::claims::AuthClaims;
use data::channels::Channels;
use connection::GetSecrets;
use model::state::error::BroadcastError;

pub struct ActionState {
    pub database: Conn, //TODO: this should be templated
    pub scripting: Scripting,
    pub claims: Option<AuthClaims>,
    pub secrets: Secrets,
    pub pub_sub: Arc<PubSubOps>,
    pub jwt_issuer: String,
    pub jwt_duration: i64,
    pub jwt_refresh_duration: i64,
}

impl fmt::Debug for ActionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionState")
    }
}

pub trait StateFunctions<'a>
    where
        Self: Debug + Send,
        Self::TableController: TableActionFunctions,
        Self::Scripting: ScriptFunctions,
        Self::EmailSender: EmailOps,
        //TODO: managementstore
        Self::EntityRetrieverFunctions: RetrieverFunctions,
        Self::EntityModifierFunctions: ModifierFunctions,
        //managementstore
        Self::UserManagement: UserManagementOps,
        Self::Authorization: AuthorizationOps,
        Self::Authentication: AuthenticationOps,
{
    // user managment
    type Authentication; //Jwt maanager and session management
    fn get_authentication(&'a self) -> Self::Authentication;

    type Authorization; //Read only user stuff
    fn get_authorization(&'a self) -> Self::Authorization;

    type UserManagement; //write user stuff
    fn get_user_management(&'a self) -> Self::UserManagement;

    // tables management
    type EntityRetrieverFunctions;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions;

    type EntityModifierFunctions;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions;

    // table actions
    type TableController;
    fn get_table_controller(&'a self) -> Self::TableController;

    type Scripting;
    fn get_script_runner(&'a self) -> Self::Scripting;

    type Database;
    fn get_database(&'a self) -> Self::Database;

    type EmailSender;
    fn get_email_sender(&'a self) -> Self::EmailSender;

    fn get_pub_sub(&'a self) -> &Arc<PubSubOps>;

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E> //TODO: why is it a diesel::result::Error?
        where F: FnOnce() -> Result<G, E>, E: From<diesel::result::Error>;
}


impl<'a> StateFunctions<'a> for ActionState {
    type Authentication = Authentication<'a>;
    fn get_authentication(&'a self) -> Self::Authentication {
        Authentication {
            conn: &self.database,
            password_secret: self.get_password_secret().to_owned(),
            jwt_secret: self.get_token_secret().to_owned(),
            jwt_duration: self.jwt_duration,
            jwt_refresh_duration: self.jwt_refresh_duration,
            jwt_issuer: self.jwt_issuer.to_owned(),
        }
    }

    type Authorization = Authorization<'a>;
    fn get_authorization(&'a self) -> Self::Authorization {
        Authorization {
            conn: &self.database,
            claims: &self.claims,
        }
    }

    type UserManagement = UserManagement<'a>;
    fn get_user_management(&'a self) -> Self::UserManagement {
        let authentication = self.get_authentication();
        UserManagement {
            conn: &self.database,
            authentication,
        }
    }

    type EntityRetrieverFunctions = EntityRetrieverController<'a>;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions {
        EntityRetrieverController {
            conn: &self.database,
            claims: &self.claims,
        }
    }

    type EntityModifierFunctions = EntityModifierController<'a>;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions {
        let user_management = self.get_user_management();

        EntityModifierController {
            conn: &self.database,
            claims: &self.claims,
            scripting: &self.scripting,
            user_management,
        }
    }

    type TableController = TableAction<'a>;
    fn get_table_controller(&'a self) -> Self::TableController {
        TableAction {
            conn: &self.database,
        }
    }

    type Scripting = Scripting;
    fn get_script_runner(&'a self) -> Self::Scripting {
        self.scripting.clone()
    }

    type Database = &'a Conn;
    fn get_database(&'a self) -> Self::Database {
        &self.database
    }

    type EmailSender = EmailSender;
    fn get_email_sender(&'a self) -> Self::EmailSender {
        EmailSender {}
    }

    fn get_pub_sub(&'a self) -> &Arc<PubSubOps> {
        &self.pub_sub
    }

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E> //TODO: should work for all state actions
        where F: FnOnce() -> Result<G, E>, E: From<diesel::result::Error> {
        let conn = &self.database;
        conn.transaction::<G, E, _>(f)
    }
}

impl ActionState {
    //TODO: this has too many parameters
    pub fn new<PS: PubSubOps + 'static>(
        database: Conn,
        scripting: Scripting,
        claims: Option<AuthClaims>,
        secrets: Secrets,
        pub_sub: PS,
        jwt_issuer: String,
        jwt_duration: i64,
        jwt_refresh_duration: i64,
    ) -> Self {
        Self {
            database,
            scripting,
            claims,
            secrets,
            pub_sub: Arc::new(pub_sub),
            jwt_issuer, //TODO: put these in config
            jwt_duration,
            jwt_refresh_duration,
        }
    }
}

pub struct Authentication<'a> {
    pub conn: &'a Conn,
    pub password_secret: String,
    pub jwt_secret: String,
    pub jwt_duration: i64,
    pub jwt_refresh_duration: i64,
    pub jwt_issuer: String,
}

pub struct Authorization<'a> {
    pub conn: &'a Conn,
    pub claims: &'a Option<AuthClaims>,
}

pub struct UserManagement<'a> {
    pub conn: &'a Conn,
    pub authentication: Authentication<'a>
}

pub trait PubSubOps
    where Self: Send + Sync
{
    fn publish(&self, channels: Vec<Channels>, action_name: String, action_result: &serde_json::Value) -> Result<(), BroadcastError>;

    fn subscribe(&self, channels: Vec<Channels>) -> Result<(), BroadcastError>;
}

impl GetSecrets for ActionState {
    fn get_token_secret(&self) -> String {
        self.secrets.token_secret.to_owned()
    }

    fn get_password_secret(&self) -> String {
        self.secrets.password_secret.to_owned()

    }
}
