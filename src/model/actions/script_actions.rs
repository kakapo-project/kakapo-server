use std::result::Result::Ok;
use std::marker::PhantomData;

use data;
use model::actions::results::*;
use model::actions::error::Error;
use model::script;

use model::state::ActionState;
use data::permissions::Permission;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::state::GetBroadcaster;
use model::state::StateFunctions;
use model::entity::RetrieverFunctions;

// Query Action
#[derive(Debug)]
pub struct RunScript<S = ActionState, SC = script::ScriptAction>  {
    pub script_name: String,
    pub param: data::ScriptParam,
    pub phantom_data: PhantomData<(S, SC)>,
}

impl<S, SC> RunScript<S, SC>
    where
        SC: script::ScriptActionFunctions<S>,
        for<'a> S: GetBroadcaster + StateFunctions<'a>,
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

impl<S, SC> Action<S> for RunScript<S, SC>
    where
        SC: script::ScriptActionFunctions<S>,
        for<'a> S: GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RunScriptResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one(&self.script_name)
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
            .and_then(|res| ActionRes::new("RunScript", RunScriptResult(res)))
    }
}



#[cfg(test)]
mod test {
    use super::*;

    use test_common::random_identifier;
    use serde_json::from_value;
    use test_common::*;
    use model::actions::entity_actions;

    #[test]
    fn test_run_script() {
        with_state(|state| {
            let script_name = format!("my_table{}", random_identifier());
            let script: data::Script = from_value(json!({
                "name": script_name,
                "description": "table description",
                "text": "print('Hello World')"
            })).unwrap();

            let create_action = entity_actions::CreateEntity::<data::Script, MockState>::new(script);
            let result = create_action.call(&state);

            println!("{:?}", &result);
        });
    }
}