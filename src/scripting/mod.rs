#![feature(specialization)]

mod docker_utils;
pub mod error;
pub mod update_state;

use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use scripting::docker_utils as Run;
use scripting::error::ScriptError;

/// Roadmap for scripts
/// - Better permissioning
///     - Every script should have it's own user with it's own user
/// - More run options
///     - Run on docker, serverless
/// - library support (i.e. pip install ..., custom libraries)
/// - Versioning scripts ( + Full git integration)
/// - More languages
/// - Cron support
/// - More efficient updates (i.e. don't upload the entire script all the time)

pub trait ScriptFunctions {
    fn run(&self, script_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ScriptError>;
}

#[derive(Clone, Debug)]
pub struct Scripting {
    script_home: PathBuf,
}

impl Scripting {
    pub fn new(script_home: PathBuf) -> Self {
        Self {
            script_home
        }
    }

    pub fn get_home(&self) -> PathBuf {
        self.script_home.to_owned()
    }
}

impl ScriptFunctions for Scripting {

    fn run(&self, script_name: &str, params: &serde_json::Value) -> Result<serde_json::Value, ScriptError> {
        let script_home = &self.script_home;
        //check if image exists
        //run container >> sudo docker run -v {{tmpfile}}:/var/commfile.txt {{name}}

        let gil = Python::acquire_gil();
        let py = gil.python();
        let sys = py.import("sys")
            .map_err(|_| ScriptError::Unknown)?;
        let version: String = sys.get("version")
            .and_then(|res| res.extract())
            .map_err(|_| ScriptError::Unknown)?;

        let locals = PyDict::new(py);
        let import = py.import("os")
            .map_err(|_| ScriptError::Unknown)?;

        locals
            .set_item("os", import)
            .map_err(|_| ScriptError::Unknown)?;
        let eval = py
            .eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals))
            .map_err(|_| ScriptError::Unknown)?;
        let user: String = eval
            .extract()
            .map_err(|_| ScriptError::Unknown)?;

        println!("Hello {}, I'm Python {}", user, version);

        unimplemented!();

    }
}
