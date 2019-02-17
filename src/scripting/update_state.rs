use std::path::PathBuf;
use std::fs;
use std::process::Command;

use data;
use data::Named;
use data::permissions::Permission;


use model::entity::error::EntityError;
use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;
use model::entity::update_state::UpdateActionFunctions;
use model::entity::update_state::UpdatePermissionFunctions;
use model::state::user_management::UserManagementOps;

//TODO: there could be different types of script runners
// docker, serverless, or local
// currently we only have local

impl UpdateActionFunctions for data::Script {
    fn create_entity(controller: &EntityModifierController, new: &data::Script) -> Result<(), EntityError> {
        info!("Creating the directory for script {:?}", &new.my_name());
        let script_name = &new.my_name();

        let path_dir = controller.scripting.get_script_home(&script_name);
        let script_path = controller.scripting.get_script_path(&script_name);

        fs::create_dir_all(&path_dir)
            .map_err(|err| EntityError::FileSystemError(format!("Could not create directory: {}", err.to_string())))?;
        info!("created the directory for script {:?} at {:?}", &new.my_name(), &path_dir);


        let script_text = &new.text;
        fs::write(&script_path, script_text.to_owned())
            .map_err(|err| EntityError::FileSystemError(format!("Could not create file: {}", err.to_string())))?;

        info!("created the file for script {:?} at {:?}", &new.my_name(), &script_path);

        //TODO: pip env this

        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {

        data::Script::delete_entity(controller, old)?;
        data::Script::create_entity(controller, new)?;

        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Script) -> Result<(), EntityError> {
        info!("Deleting the directory for script {:?}", &old.my_name());
        let script_name = &old.my_name();

        let path_dir = controller.scripting.get_script_home(&script_name);


        fs::remove_dir_all(&path_dir)
            .map_err(|err| EntityError::FileSystemError(format!("Could not delete directory: {}", err.to_string())))?;

        info!("deleted the file for script {:?} at {:?}", &old.my_name(), &path_dir);

        Ok(())
    }
}

impl UpdatePermissionFunctions for data::Script {
    fn create_permission(controller: &EntityModifierController, new: &data::Script) -> Result<(), EntityError> {
        let permission_list = vec![
            Permission::read_entity::<data::Script>(new.my_name().to_owned()),
            Permission::modify_entity::<data::Script>(new.my_name().to_owned()),
            Permission::run_script(new.my_name().to_owned()),
        ];

        //TODO: assuming that we are going to attach it to the user permission
        match controller.get_role_name() {
            Some(rolename) => for permission in permission_list {
                controller
                    .user_management
                    .attach_permission_for_role(&permission, &rolename);
            },
            None => for permission in permission_list {
                controller
                    .user_management
                    .add_permission(&permission);
            },
        };

        Ok(())
    }

    fn update_permission(controller: &EntityModifierController, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {
        let old_name = old.my_name().to_owned();
        let new_name = new.my_name().to_owned();

        let permission_list = vec![
            (
                Permission::read_entity::<data::Script>(old_name.to_owned()),
                Permission::read_entity::<data::Script>(new_name.to_owned()),
            ),
            (
                Permission::modify_entity::<data::Script>(old_name.to_owned()),
                Permission::modify_entity::<data::Script>(new_name.to_owned()),
            ),
            (
                Permission::run_script(old_name.to_owned()),
                Permission::run_script(new_name.to_owned()),
            )
        ];

        for (old_permission, new_permission) in permission_list {
            controller
                .user_management
                .rename_permission(&old_permission, &new_permission);
        }

        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::Script) -> Result<(), EntityError> {

        let permission_list = vec![
            Permission::read_entity::<data::Script>(old.my_name().to_owned()),
            Permission::modify_entity::<data::Script>(old.my_name().to_owned()),
            Permission::run_script(old.my_name().to_owned()),
        ];

        for permission in permission_list {
            controller
                .user_management
                .remove_permission(&permission);
        }

        Ok(())
    }
}
