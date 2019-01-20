use model::entity::error::EntityError;
use model::table::error::TableError;
use model::query::error::QueryError;

use diesel::result::Error as DieselError;
use model::script::error::ScriptError;
use model::auth::error::UserManagementError;

#[derive(Debug, Fail)]
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
    UserManagement(UserManagementError),
    #[fail(display = "Not authorized")]
    Unauthorized,
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Already exists")]
    AlreadyExists,
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

impl From<DieselError> for Error {
    //as far as I can tell, this function will only run if the transaction fails, which wouldn't
    //normally return any specific error, it will return the inner error
    fn from(diesel_error: DieselError) -> Self {
        Error::Unknown
    }
}