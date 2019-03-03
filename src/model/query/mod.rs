
pub mod error;

use std::fmt::Debug;

use data;

use model::query::error::QueryError;

use state::StateFunctions;
use state::authorization::AuthorizationOps;
use state::ActionState;

#[derive(Debug, Clone)]
pub struct QueryAction {}

pub trait QueryActionFunctions<S>
    where Self: Send + Debug,
{
    fn run_query(conn: &S, query: &data::DataQueryEntity, params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, QueryError>;
}


impl QueryActionFunctions<ActionState> for QueryAction {
    fn run_query(state: &ActionState, query: &data::DataQueryEntity, params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, QueryError>  {
        /* TODO:...
        */
        unimplemented!()
    }
}