use model::entity::error::EntityError;
use model::table::error::TableQueryError;

use diesel::result::Error as DieselError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{:?}", 0)]
    Entity(EntityError),
    #[fail(display = "{:?}", 0)]
    TableQuery(TableQueryError),
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