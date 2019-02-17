use std::result::Result::Ok;
use std::marker::PhantomData;

use data;
use data::Named;
use model::actions::results::*;
use model::actions::error::Error;

use model::state::ActionState;
use data::permissions::Permission;

use model::actions::decorator::*;

use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::state::StateFunctions;
use model::entity::RetrieverFunctions;
use scripting::ScriptFunctions;
use scripting::ScriptResult;

// Script Action
#[derive(Debug)]
pub struct RunScript<S = ActionState>  {
    pub script_name: String,
    pub param: data::ScriptParam,
    pub phantom_data: PhantomData<(S)>,
}

impl<S> RunScript<S>
    where
        for<'a> S: StateFunctions<'a>,
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

impl<S> Action<S> for RunScript<S>
    where
        for<'a> S: StateFunctions<'a>,
{
    type Ret = ScriptResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_entity_retreiver_functions()
            .get_one::<data::Script>(&self.script_name)
            .map_err(Error::Entity)
            .and_then(|res| match res {
                Some(query) => Ok(query),
                None => Err(Error::NotFound),
            })
            .and_then(|script| {
                state
                    .get_script_runner()
                    .run(&script, &self.param)
                    .map_err(Error::Script)
            })
            .and_then(|res| ActionRes::new("RunScript", res))
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
                "name": script_name.to_owned(),
                "description": "table description",
                "text": r#"
import sys
import os

print('Hello World')
print('Bye World', file=sys.stderr)

filename = sys.argv[1]
with open(filename, 'r') as f:
    print(f.read())
with open(filename, 'w') as f:
    f.write('{"bye": "world"}')
                "#
            })).unwrap();

            let create_action = entity_actions::CreateEntity::<data::Script, MockState>::new(script);
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            let params = json!({"Hello": "World"});
            let create_action = RunScript::<MockState>::new(script_name, params);
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();
            assert_eq!(data.successful, true);
            assert_eq!(data.stdout, "Hello World\n{\"Hello\":\"World\"}\n");
            assert_eq!(data.stderr, "Bye World\n");
            assert_eq!(data.output, json!({"bye": "world"}));
        });
    }
}