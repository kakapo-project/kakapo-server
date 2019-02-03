
pub mod error;

use model::state::State;
use data;
use model::script::error::ScriptError;
use std::fmt::Debug;

use serde_json;
use model::state::GetScripting;

#[derive(Debug, Clone)]
pub struct ScriptAction;
pub trait ScriptActionFunctions<S>
    where Self: Send + Debug
{
    fn run_script(conn: &S, script: &data::Script) -> Result<serde_json::Value, ScriptError>;
}

impl ScriptActionFunctions<State> for ScriptAction {
    /// runs the given script
    /// NOTE: this doesn't have a timeout it probably should
    /// Warning: if you have access to the script you may have access to anything stored in that container, potential vulnerability?
    fn run_script(conn: &State, script: &data::Script) -> Result<serde_json::Value, ScriptError>  {
        //TODO: debug mode for state

        let scripting = conn.get_scripting();
        unimplemented!()
    }
}