
pub mod error;

use model::state::ActionState;
use data;
use model::query::error::QueryError;
use database::DatabaseFunctions;
use model::auth::permissions::GetUserInfo;
use data::Value;
use data::QueryParams;
use std::marker::PhantomData;
use std::fmt::Debug;
use model::state::StateFunctions;

#[derive(Debug, Clone)]
pub struct QueryAction {}

pub trait QueryActionFunctions<S>
    where Self: Send + Debug,
{
    fn run_query(conn: &S, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError>;
}

impl QueryActionFunctions<ActionState> for QueryAction {
    fn run_query(state: &ActionState, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError>  {
        let username = state.get_username();
        let db_params = params.value_list();

        if let Some(db_user) = username.to_owned() {
            state
                .get_database()
                .exec("SET SESSION AUTHORIZATION $1", vec![Value::String(db_user)])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        let result = state
            .get_database()
            .exec(&query.statement, db_params)
            .or_else(|err| Err(QueryError::db_error(err)))?;

        if let Some(db_user) = username {
            state
                .get_database()
                .exec("RESET SESSION AUTHORIZATION", vec![])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        Ok(result)
    }
}