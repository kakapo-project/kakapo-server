use data;
use database::error::DbError;

pub mod update_state;
pub mod error;
pub mod sql;

pub trait DatabaseFunctions {
    fn exec(&self, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError>;
}