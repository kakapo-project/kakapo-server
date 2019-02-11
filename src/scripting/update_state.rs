use std::path::PathBuf;
use std::fs;

use model::entity::error::EntityError;
use data;

use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;
use model::entity::update_state::UpdateActionFunctions;
use std::process::Command;


//TODO: there could be different types of script runners
// docker, serverless, or local
// currently we only have local

const SCRIPT_FILE_NAME: &'static str = "script.py";

impl UpdateActionFunctions for data::Script {
    fn create_entity(controller: &EntityModifierController, new: &data::Script) -> Result<(), EntityError> {
        info!("Creating the directory for script {:?}", &new.name);
        let script_name = &new.name;
        let script_home = controller.scripting.get_home();

        let mut path_dir = PathBuf::from(script_home);
        path_dir.push(script_name);

        let mut script_path = path_dir.to_owned();
        let path = path_dir.to_str()
            .ok_or_else(|| EntityError::FileSystemError(format!("Could not create path")))?;

        fs::create_dir_all(path.to_owned())
            .map_err(|err| EntityError::FileSystemError(format!("Could not create directory: {}", err.to_string())))?;
        info!("created the directory for script {:?} at {}", &new.name, &path);

        script_path.push(SCRIPT_FILE_NAME);
        let path = script_path.to_str()
            .ok_or_else(|| EntityError::FileSystemError(format!("Could not create path")))?;

        let script_text = &new.text;
        fs::write(path, script_text)
            .map_err(|err| EntityError::FileSystemError(format!("Could not create file: {}", err.to_string())))?;

        info!("created the file for script {:?} at {}", &new.name, &path);

        //TODO: pip env this

        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {
        data::Script::delete_entity(controller, old)?;
        data::Script::create_entity(controller, new)?;

        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Script) -> Result<(), EntityError> {
        info!("Deleting the directory for script {:?}", &old.name);
        let script_name = &old.name;
        let script_home = controller.scripting.get_home();

        let mut path_dir = PathBuf::from(script_home);
        path_dir.push(script_name);

        let mut script_path = path_dir.to_owned();
        let path = path_dir.to_str()
            .ok_or_else(|| EntityError::FileSystemError(format!("Could not create path")))?;

        fs::remove_dir_all(path.to_owned())
            .map_err(|err| EntityError::FileSystemError(format!("Could not delete directory: {}", err.to_string())))?;

        info!("deleted the file for script {:?} at {}", &old.name, &path);

        Ok(())
    }
}
