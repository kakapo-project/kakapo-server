
pub mod error;

use model::state::ChannelBroadcaster;
use model::state::State;
use data;
use model::query::error::QueryError;
use database::Database;
use database::DatabaseFunctions;
use model::state::GetConnection;

pub struct QueryAction;
pub trait QueryActionFunctions<S> {
    fn run_query(conn: &S, query: &data::Query) -> Result<data::TableData, QueryError> ;
}

impl<B> QueryActionFunctions<State<B>> for QueryAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn run_query(conn: &State<B>, query: &data::Query) -> Result<data::TableData, QueryError>  {

        //TODO: Set session authorization
        Database::exec(conn.get_conn(), &query.statement/*, params*/);
        //TODO: Reset session authorization
        unimplemented!()
    }
}