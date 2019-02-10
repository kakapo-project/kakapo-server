use model::entity::error::EntityError;
use data;

use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;
use model::entity::update_state::UpdateActionFunctions;

//TODO: there could be different types of script runners
// docker, serverless, or local
// currently we only have local


impl UpdateActionFunctions for data::Script {
    fn create_entity(controller: &EntityModifierController, new: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        //let scripting = controller.scripts;
        //Scripting::build();
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        //let scripting = controller.scripts;
        //TODO: this should be debounced so that image doesn't get rebuild all the time
        //Scripting::build();
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        //let scripting = controller.scripts;
        //Scripting::delete();
        Ok(())
    }
}
