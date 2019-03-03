
use std::result::Result::Ok;
use std::marker::PhantomData;

use model::actions::results::*;
use model::actions::error::Error;
use model::query;

use data;
use data::permissions::*;

use model::actions::decorator::*;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::entity::RetrieverFunctions;

use state::StateFunctions;
use state::ActionState;

// Query Action
#[derive(Debug)]
pub struct RunQuery<S = ActionState, QC = query::QueryAction>  {
    pub query_name: String,
    pub params: serde_json::Value,
    pub format: serde_json::Value,
    pub phantom_data: PhantomData<(S, QC)>,
}

impl<S, QC> RunQuery<S, QC>
    where
        QC: query::QueryActionFunctions<S>,
        for<'a> S: StateFunctions<'a>,
{
    pub fn new(query_name: String, params: serde_json::Value) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            query_name: query_name.to_owned(),
            params,
            format: json!({}), //TODO:... example: TableDataFormat::Rows
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_query(query_name));

        action_with_permission
    }
}

impl<S, QC> Action<S> for RunQuery<S, QC>
    where
        QC: query::QueryActionFunctions<S>,
        for<'a> S: StateFunctions<'a>,
{
    type Ret = RunQueryResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one(&self.query_name)
            .map_err(Error::Entity)
            .and_then(|res| match res {
                Some(query) => Ok(query),
                None => Err(Error::NotFound),
            })
            .and_then(|query| {
                QC::run_query(state, &query, &self.params, &self.format)
                    .map_err(Error::Query)
            })
            .and_then(|res| ActionRes::new("RunQuery", RunQueryResult(res)))
    }
}
