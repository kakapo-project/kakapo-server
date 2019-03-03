pub mod error;
pub mod sql;
mod error_parser;

use kakapo_postgres::database::error::DbError;
use kakapo_postgres::data::RawTableData;
use kakapo_postgres::data::Value;

pub trait DatabaseFunctions {
    fn exec(&self, query: &str, params: Vec<Value>) -> Result<RawTableData, DbError>;
}