
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;
use model::entity;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::TableDataFormat;
use model::query;

use model::state::State;
use model::state::GetConnection;
use model::auth::permissions::*;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::auth::permissions::GetUserInfo;
use model::state::GetBroadcaster;


// Query Action
#[derive(Debug)]
pub struct RunQuery<S = State, ER = entity::Controller, QC = query::QueryAction>  {
    pub query_name: String,
    pub params: data::QueryParams,
    pub format: TableDataFormat,
    pub phantom_data: PhantomData<(S, ER, QC)>,
}

impl<S, ER, QC> RunQuery<S, ER, QC>
    where
        ER: entity::RetrieverFunctions<data::Query, S>,
        QC: query::QueryActionFunctions<S>,
        S: GetConnection + GetUserInfo + GetBroadcaster,
{
    pub fn new(query_name: String, params: data::QueryParams) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            query_name: query_name.to_owned(),
            params,
            format: TableDataFormat::Rows,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_query(query_name));

        action_with_permission
    }
}

impl<S, ER, QC> Action<S> for RunQuery<S, ER, QC>
    where
        ER: entity::RetrieverFunctions<data::Query, S> + Send,
        QC: query::QueryActionFunctions<S> + Send,
        S: GetConnection + GetUserInfo + GetBroadcaster,
{
    type Ret = RunQueryResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.query_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Query>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|query| {
                QC::run_query(state, &query, &self.params)
                    .or_else(|err| Err(Error::Query(err)))
            })
            .and_then(|table_data| {
                Ok(table_data.format_with(&self.format))
            })
            .and_then(|res| ActionRes::new("RunQuery", RunQueryResult(res)))
    }
}
