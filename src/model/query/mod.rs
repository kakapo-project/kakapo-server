
pub mod error;

use model::state::State;
use data;
use model::query::error::QueryError;
use database::Database;
use database::DatabaseFunctions;
use model::state::GetConnection;
use model::auth::permissions::GetUserInfo;
use data::Value;
use data::QueryParams;
use std::marker::PhantomData;

pub struct QueryAction<D = Database> {
    phantom_data: PhantomData<D>,
}

pub trait QueryActionFunctions<S>
    where Self: Send,
{
    fn run_query(conn: &S, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError> ;
}

impl<D> QueryActionFunctions<State> for QueryAction<D>
    where D: DatabaseFunctions,
{
    fn run_query(conn: &State, query: &data::Query, params: &QueryParams) -> Result<data::RawTableData, QueryError>  {
        let db_user = conn.get_db_user();
        let db_params = params.value_list();

        let result = D::exec(conn.get_conn(), "SET SESSION AUTHORIZATION $1", vec![Value::String(db_user)])
            .and_then(|res| {
                D::exec(conn.get_conn(), &query.statement, db_params)
            })
            .or_else(|err| Err(QueryError::db_error(err)));

        D::exec(conn.get_conn(), "RESET SESSION AUTHORIZATION", vec![])
            .or_else(|err| Err(QueryError::db_error(err)))
            .and_then(|res| result)
    }
}