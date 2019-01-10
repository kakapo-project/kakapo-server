
pub mod error;

use model::state::ChannelBroadcaster;
use model::state::State;
use data;
use model::script::error::ScriptError;

use serde_json;

pub struct ScriptAction;
pub trait ScriptActionFunctions<S> {
    fn run_script(conn: &S, script: &data::Script) -> Result<serde_json::Value, ScriptError>;
}

impl<B> ScriptActionFunctions<State<B>> for ScriptAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    /// runs the given script
    /// NOTE: this doesn't have a timeout it probably should
    /// Warning: if you have access to the script you may have access to anything stored in that container, potential vulnerability?
    fn run_script(conn: &State<B>, script: &data::Script) -> Result<serde_json::Value, ScriptError>  {
        //TODO: debug mode

        //let script_runner_process = conn.get_execute_script();
        unimplemented!()
    }
}