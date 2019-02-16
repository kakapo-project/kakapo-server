use model::entity::error::EntityError;
use model::table::error::TableError;
use model::query::error::QueryError;
use model::state::error::UserManagementError;
use model::auth::send_mail::EmailError;

use scripting::error::ScriptError;
use pubsub::error::BroadcastError;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum Error {
    #[fail(display = "{:?}", 0)]
    Entity(EntityError),
    #[fail(display = "{:?}", 0)]
    Table(TableError),
    #[fail(display = "{:?}", 0)]
    Script(ScriptError),
    #[fail(display = "{:?}", 0)]
    Query(QueryError),
    #[fail(display = "{:?}", 0)]
    EmailError(EmailError),
    #[fail(display = "{:?}", 0)]
    UserManagement(UserManagementError),
    #[fail(display = "Not authorized")]
    Unauthorized,
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "{:?}", 0)]
    SerializationError(String),
    #[fail(display = "Could not publish {:?}", 0)]
    PublishError(BroadcastError),
    #[fail(display = "An unknown error occurred")]
    Unknown,
}