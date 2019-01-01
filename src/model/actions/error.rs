use model::entity::error::EntityError;
use model::table::error::TableQueryError;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    Entity(EntityError),
    TableQuery(TableQueryError),
    Unauthorized,
    NotFound,
    AlreadyExists,
    Unknown,
}