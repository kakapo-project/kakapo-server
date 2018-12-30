use model::entity::error::DBError;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    DB(DBError),
    Unauthorized,
    NotFound,
    AlreadyExists,
    Unknown,
}