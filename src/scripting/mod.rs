#![feature(specialization)]

pub mod error;
pub mod update_state;

use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use std::io::Read;
use std::str::from_utf8;

use tempfile;

use scripting::error::ScriptError;
use data::Script;
use data::Named;



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
    fn run(&self, script: &Script, params: &serde_json::Value) -> Result<ScriptResult, ScriptError>;
}

#[derive(Clone, Debug)]
pub struct Scripting {
    script_home: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptResult {
    pub successful: bool,
    pub stdout: String,
    pub stderr: String,
    pub output: serde_json::Value,
}

const PYTHON: &'static str = "python3";
const SCRIPT_NAME: &'static str = "script.py";

impl Scripting {
    pub fn new(script_home: PathBuf) -> Self {
        Self {
            script_home
        }
    }

    pub fn get_home(&self) -> PathBuf {
        self.script_home.to_owned()
    }

    pub fn get_script_home(&self, script_name: &str) -> PathBuf {
        let mut path = self.get_home();
        path.push(script_name);

        path
    }

    pub fn get_script_path(&self, script_name: &str) -> PathBuf {
        let mut path = self.get_home();
        path.push(script_name);
        path.push(SCRIPT_NAME);

        path
    }
}

impl ScriptFunctions for Scripting {

    fn run(&self, script: &Script, params: &serde_json::Value) -> Result<ScriptResult, ScriptError> {
        let script_home = &self.script_home;
        let path = self.get_script_home(script.my_name());

        env::set_current_dir(path)
            .map_err(|err| ScriptError::IOError(err.to_string()))?;

        let mut temp = tempfile::NamedTempFile::new()
            .map_err(|err| ScriptError::IOError(err.to_string()))?;

        let io_file_path = temp.path().to_str()
            .ok_or_else(|| ScriptError::IOError("Could not convert path to string".to_string()))?
            .to_owned();
        let mut io_file = temp.as_file_mut();

        let params_text = serde_json::to_string(&params)
            .map_err(|err| ScriptError::IOError(err.to_string()))?;
        io_file.write_all(&params_text.as_bytes())
            .map_err(|err| ScriptError::IOError(err.to_string()))?;

        let output = Command::new(PYTHON)
            .arg(SCRIPT_NAME)
            .arg(&io_file_path)
            .output()
            .map_err(|err| ScriptError::ExecuteError(err.to_string()))?;

        let is_successful = output.status.success();

        if is_successful {
            info!("Ran script successfully");
            let output_str = fs::read_to_string(io_file_path).unwrap_or_default();
            let output_value = serde_json::from_str(&output_str).unwrap_or_default();
            debug!("output_value: {:?}", &output_value);

            Ok(ScriptResult {
                successful: is_successful,
                stdout: from_utf8(&output.stdout).unwrap_or_default().to_string(),
                stderr: from_utf8(&output.stderr).unwrap_or_default().to_string(),
                output: output_value,
            })

        } else {
            warn!("Could not run the script successfuly, failed with error");

            Ok(ScriptResult {
                successful: is_successful,
                stdout: from_utf8(&output.stdout).unwrap_or_default().to_string(),
                stderr: from_utf8(&output.stderr).unwrap_or_default().to_string(),
                output: serde_json::Value::default(),
            })
        }
    }
}
