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
use model::auth::permissions::GetUserInfo;


// Query Action
#[derive(Debug)]
pub struct RunScript<S = State, ER = entity::Controller, SC = script::ScriptAction>  {
    pub script_name: String,
    pub param: data::ScriptParam,
    pub phantom_data: PhantomData<(S, ER, SC)>,
}

impl<S, ER, SC> RunScript<S, ER, SC>
    where
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection + GetUserInfo,
{
    pub fn new(script_name: String, param: data::ScriptParam) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            script_name: script_name.to_owned(),
            param,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::run_script(script_name));

        action_with_permission
    }
}

impl<S, ER, SC> Action<S> for RunScript<S, ER, SC>
    where
        ER: entity::RetrieverFunctions<data::Script, S> + Send,
        SC: script::ScriptActionFunctions<S> + Send,
        S: GetConnection + GetUserInfo,
{
    type Ret = RunScriptResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        ER::get_one(state, &self.script_name)
            .or_else(|err| Err(Error::Entity(err)))
            .and_then(|res: Option<data::Script>| {
                match res {
                    Some(query) => Ok(query),
                    None => Err(Error::NotFound),
                }
            })
            .and_then(|script| {
                SC::run_script(state, &script)
                    .or_else(|err| Err(Error::Script(err)))
            })
            .and_then(|res| ActionRes::new(RunScriptResult(res)))
    }
}
