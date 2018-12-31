use model::entity::error::DBError;
use model::table::error::TableQueryError;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    DB(DBError),
    TableQuery(TableQueryError),
    Unauthorized,
    NotFound,
    AlreadyExists,
    Unknown,
}