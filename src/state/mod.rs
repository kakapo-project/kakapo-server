
pub mod error;
pub mod authentication;
pub mod authorization;
pub mod user_management;
pub mod domain_management;

use serde_json;

use std::fmt::Debug;
use std::fmt;
use std::sync::Arc;

use diesel::Connection;
use serde::Serialize;


use model::actions::error::Error;

use connection::executor::Conn;
use connection::executor::Secrets;
use connection::executor::DomainError;
use connection::GetSecrets;

use model::entity::EntityRetrieverController;
use model::entity::EntityModifierController;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::table::DatastoreAction;
use model::table::DatastoreActionOps;
use auth::send_mail::EmailSender;
use auth::send_mail::EmailOps;

use state::authorization::AuthorizationOps;
use state::authentication::AuthenticationOps;
use state::user_management::UserManagementOps;
use state::domain_management::DomainManagementOps;
use state::error::BroadcastError;

use scripting::ScriptFunctions;
use scripting::Scripting;

use data::claims::AuthClaims;
use data::channels::Channels;
use data::channels::Subscription;
use data::auth::User;
use data::Message;
use plugins::v1::Datastore;
use plugins::v1::DataQuery;
use model::query::QueryActionOps;
use model::query::QueryAction;


pub struct ActionState {
    pub database: Conn, //TODO: this should be templated
    pub scripting: Scripting,
    pub claims: Option<AuthClaims>,
    pub secrets: Secrets,
    pub domain_name: Option<String>,
    pub datastore_conn: Result<Box<Datastore>, DomainError>, //TODO: probably use the domains for this
    pub query_conn: Result<Box<DataQuery>, DomainError>,
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
        Self::TableController: DatastoreActionOps,
        Self::QueryController: QueryActionOps,
        Self::Scripting: ScriptFunctions,
        Self::PubSub: PubSubOps,
        Self::EmailSender: EmailOps,
        //TODO: managementstore
        Self::EntityRetrieverFunctions: RetrieverFunctions,
        Self::EntityModifierFunctions: ModifierFunctions,
        //managementstore
        Self::UserManagement: UserManagementOps,
        Self::DomainManagement: DomainManagementOps,
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

    type DomainManagement;
    fn get_domain_management(&'a self) -> Self::DomainManagement;

    // tables management
    type EntityRetrieverFunctions;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions;

    type EntityModifierFunctions;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions;

    // table actions
    type TableController;
    fn get_table_controller(&'a self) -> Self::TableController;

    type QueryController;
    fn get_query_controller(&'a self) -> Self::QueryController;

    type Scripting;
    fn get_script_runner(&'a self) -> Self::Scripting;

    type Database;
    fn get_database(&'a self) -> Self::Database;

    type EmailSender;
    fn get_email_sender(&'a self) -> Self::EmailSender;

    type PubSub;
    fn get_pub_sub(&'a self) -> Self::PubSub;

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

    type DomainManagement = DomainManagement<'a>;
    fn get_domain_management(&'a self) -> Self::DomainManagement {
        DomainManagement {
            conn: &self.database,
        }
    }

    type EntityRetrieverFunctions = EntityRetrieverController<'a>;
    fn get_entity_retreiver_functions(&'a self) -> Self::EntityRetrieverFunctions {
        EntityRetrieverController {
            conn: &self.database,
            claims: &self.claims,
            domain_name: &self.domain_name,
        }
    }

    type EntityModifierFunctions = EntityModifierController<'a>;
    fn get_entity_modifier_function(&'a self) -> Self::EntityModifierFunctions {
        let user_management = self.get_user_management();

        EntityModifierController {
            conn: &self.database,
            domain_conn: &self.datastore_conn, //TODO: should be a separate thing, permissions
            claims: &self.claims,
            scripting: &self.scripting,
            user_management,
            domain_name: &self.domain_name,
        }
    }

    type TableController = DatastoreAction<'a>;
    fn get_table_controller(&'a self) -> Self::TableController { //TODO: rename datastore
        DatastoreAction {
            conn: &self.datastore_conn,
        }
    }

    type QueryController = QueryAction<'a>;
    fn get_query_controller(&'a self) -> Self::QueryController {
        QueryAction {
            conn: &self.query_conn,
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

    type PubSub = PublishCallback<'a>;
    fn get_pub_sub(&'a self) -> Self::PubSub {
        PublishCallback {
            conn: &self.database,
        }
    }

    fn transaction<G, E, F>(&self, f: F) -> Result<G, E> //TODO: should work for all state actions
        where F: FnOnce() -> Result<G, E>, E: From<diesel::result::Error> {
        let conn = &self.database;
        conn.transaction::<G, E, _>(f)
    }
}

impl ActionState {
    //TODO: this has too many parameters
    pub fn new(
        database: Conn,
        scripting: Scripting,
        claims: Option<AuthClaims>,
        secrets: Secrets,
        domain_name: Option<String>,
        datastore_conn: Result<Box<Datastore>, DomainError>,
        query_conn: Result<Box<DataQuery>, DomainError>,
        jwt_issuer: String,
        jwt_duration: i64,
        jwt_refresh_duration: i64,
    ) -> Self {
        Self {
            database,
            scripting,
            claims,
            secrets,
            domain_name,
            datastore_conn,
            query_conn,
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

pub struct DomainManagement<'a> {
    pub conn: &'a Conn,
}

pub struct PublishCallback<'a> {
    pub conn: &'a Conn,
}

pub trait PubSubOps {

    fn publish(&self, channel: Channels, action_name: String, action_result: &serde_json::Value) -> Result<(), BroadcastError>;

    fn subscribe(&self, user_id: i64, channel: Channels) -> Result<Subscription, BroadcastError>;

    fn unsubscribe(&self, user_id: i64, channel: Channels) -> Result<Subscription, BroadcastError>;

    fn unsubscribe_all(&self, user_id: i64) -> Result<(), BroadcastError>;

    fn get_subscribers(&self, channel: Channels) -> Result<Vec<User>, BroadcastError>;

    fn get_messages(
        &self,
        user_id: i64,
        start_time: chrono::NaiveDateTime,
        end_time: chrono::NaiveDateTime,
    ) -> Result<Vec<Message>, BroadcastError>;

    // Some user permissions have been removed so they must be purged
    fn permissions_removed(&self) -> Result<(), BroadcastError>;
}

impl GetSecrets for ActionState {
    fn get_token_secret(&self) -> String {
        self.secrets.token_secret.to_owned()
    }

    fn get_password_secret(&self) -> String {
        self.secrets.password_secret.to_owned()

    }
}
