use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::EntityError;

use data::schema;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::conversion;
use data::dbdata::RawEntityTypes;

use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use data::utils::TableDataFormat;

use model::table;
use model::table::TableActionFunctions;
use model::query;
use model::query::QueryActionFunctions;
use model::script;
use model::script::ScriptActionFunctions;

use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::decorator::*;
use std::fmt::Debug;

use model::auth::Auth;
use model::auth::AuthFunctions;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::state::GetUserInfo;


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
        S: GetConnection + GetUserInfo,
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
        S: GetConnection + GetUserInfo,
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
            .and_then(|res| ActionRes::new(RunQueryResult(res)))
    }
}
