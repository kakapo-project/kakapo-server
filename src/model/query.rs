
use std::fmt::Debug;

use data;

use state::StateFunctions;
use state::authorization::AuthorizationOps;
use state::ActionState;
use connection::executor::DomainError;

use plugins::v1::DataQuery;
use plugins::v1::DatastoreError;

pub struct QueryAction<'a> {
    pub conn: &'a Result<Box<DataQuery>, DomainError>,
}

pub trait QueryActionOps {
    fn run_query(&self, query: &data::DataQueryEntity, params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, DatastoreError>;
}


impl<'a> QueryActionOps for QueryAction<'a> {
    fn run_query(&self, query: &data::DataQueryEntity, params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, DatastoreError>  {
        match self.conn {
            Ok(conn) => conn.query(query, params, format),
            Err(err) => Err(err.into())
        }
    }
}