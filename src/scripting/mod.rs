
mod docker_utils;
pub mod error;
pub mod update_state;

use scripting::docker_utils as Run;
use scripting::error::ScriptError;


pub trait ScriptFunctions {
    fn build(&self, script_name: &str, script_text: &str) -> Result<(), ScriptError>;
    fn delete(&self, script_name: &str) -> Result<(), ScriptError>;
    fn run(&self, script_name: &str) -> Result<(), ScriptError>;
}

#[derive(Clone, Debug)]
pub struct Scripting {
    script_home: String,
}

impl Scripting {
    pub fn new(script_home: String) -> Self {
        Self {
            script_home
        }
    }
}

impl ScriptFunctions for Scripting {

    fn build(&self, script_name: &str, script_text: &str) -> Result<(), ScriptError> {
        let script_home = &self.script_home;
        debug!("building the image");
        //delete image if exists
        let _delete_image_result = Run::delete_image("the_room");

        //delete directory
        let _delete_dir_result = Run::delete_directory(script_home, "the_room");

        //make dockerfile
        Run::build_directory(script_home, &script_name, &script_text)
            .and_then(|res| {
                //docker build >> sudo docker build . --tag={{name}}
                Run::build_image(script_home, &script_name)
            })

    }

    fn delete(&self, script_name: &str) -> Result<(), ScriptError> {
        let script_home = &self.script_home;

        //delete image if exists
        Run::delete_image(script_name);

        //delete directory
        Run::delete_directory(script_home,script_name);
        unimplemented!();
    }

    fn run(&self, script_name: &str) -> Result<(), ScriptError> {
        let script_home = &self.script_home;
        //check if image exists
        //run container >> sudo docker run -v {{tmpfile}}:/var/commfile.txt {{name}}

        Run::run_container(script_name);
        unimplemented!();

    }
}
