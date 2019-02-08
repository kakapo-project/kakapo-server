
pub mod error;

use model::state::ActionState;
use data;
use model::query::error::QueryError;
use database::Database;
use database::DatabaseFunctions;
use model::state::GetConnection;
use model::auth::permissions::GetUserInfo;
use data::Value;
use data::QueryParams;
use std::marker::PhantomData;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueryAction<D = Database> {
    phantom_data: PhantomData<D>,
}

pub trait QueryActionFunctions<S>
    where Self: Send + Debug,
{
    fn run_query(conn: &S, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError> ;
}

impl<D> QueryActionFunctions<ActionState> for QueryAction<D>
    where D: DatabaseFunctions,
{
    fn run_query(conn: &ActionState, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError>  {
        let username = conn.get_username();
        let db_params = params.value_list();

        if let Some(db_user) = username.to_owned() {
            D::exec(conn.get_conn(), "SET SESSION AUTHORIZATION $1", vec![Value::String(db_user)])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        let result = D::exec(conn.get_conn(), &query.statement, db_params)
            .or_else(|err| Err(QueryError::db_error(err)))?;

        if let Some(db_user) = username {
            D::exec(conn.get_conn(), "RESET SESSION AUTHORIZATION", vec![])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        Ok(result)
    }
}