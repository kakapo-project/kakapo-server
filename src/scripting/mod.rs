
mod docker_utils;
pub mod error;
pub mod update_state;

use scripting::docker_utils as Run;
use scripting::error::ScriptError;


pub trait ScriptFunctions {
    fn run(&self, script_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ScriptError>;
}

#[derive(Clone, Debug)]
pub struct Scripting {
    script_home: String,
}

impl Scripting {
    pub fn new(script_home: Strgiuting) -> Self {
        Self {
            script_home
        }
    }

    pub fn get_home(&self) -> String {
        self.script_home.to_owned()
    }
}

impl ScriptFunctions for Scripting {

    fn run(&self, script_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ScriptError> {
        let script_home = &self.script_home;
        //check if image exists
        //run container >> sudo docker run -v {{tmpfile}}:/var/commfile.txt {{name}}

        Run::run_container(script_name);
        unimplemented!();

    }
}
