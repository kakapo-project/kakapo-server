
pub mod error;

use model::state::ChannelBroadcaster;
use model::state::State;
use data;
use model::script::error::ScriptError;

use serde_json;

pub struct ScriptAction;
pub trait ScriptActionFunctions<S> {
    fn run_script() -> Result<serde_json::Value, ScriptError>;
}

impl<B> ScriptActionFunctions<State<B>> for ScriptAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn run_script() -> Result<serde_json::Value, ScriptError>  {
        unimplemented!()
    }
}