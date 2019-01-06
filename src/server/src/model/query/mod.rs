
pub mod error;

use model::state::ChannelBroadcaster;
use model::state::State;
use data;
use model::query::error::QueryError;

pub struct QueryAction;
pub trait QueryActionFunctions<S> {
    fn run_query() -> Result<data::TableData, QueryError> ;
}

impl<B> QueryActionFunctions<State<B>> for QueryAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn run_query() -> Result<data::TableData, QueryError>  {
        unimplemented!()
    }
}