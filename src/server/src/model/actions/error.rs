use model::entity::error::EntityError;
use model::table::error::TableQueryError;

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