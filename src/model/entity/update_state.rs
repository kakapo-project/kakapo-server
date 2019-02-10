use model::entity::results::*;
use model::entity::error::EntityError;
use data;
use model::state::ActionState;

use std::error::Error;

use diesel::RunQueryDsl;
use data::DataType;
use std::fmt::Debug;
use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;


pub trait UpdateActionFunctions {
    fn create_entity(controller: &EntityModifierController, new: &Self) -> Result<(), EntityError>;
    fn update_entity(controller: &EntityModifierController, old_table: &Self, new_table: &Self) -> Result<(), EntityError>;
    fn delete_entity(controller: &EntityModifierController, old: &Self) -> Result<(), EntityError>;
}

/// This trait does something action specific after the database updates
/// The name is a little bit confusing because the database store is also modification
/// But this module is responsible for all the other modifications
pub trait UpdateState<T>
    where
        Self: Sized,
        T: Debug + RawEntityTypes,
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError>;
}

//Created
impl<T> UpdateState<T> for Created<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        info!("new: {:?}", &self);
        let res = match &self {
            Created::Success { new } => T::create_entity(&state, &new),
            _ => Ok(()),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

//Upserted
impl<T> UpdateState<T> for Upserted<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Upserted::Update { old, new } => T::update_entity(&state, &old, &new),
            Upserted::Create { new } => T::create_entity(&state, &new),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

//Updated
impl<T> UpdateState<T> for Updated<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Updated::Success { old, new } => T::update_entity(&state, &old, &new),
            _ => Ok(()),
        }?;

        Ok(self)
    }
}

//Deleted
impl<T> UpdateState<T> for Deleted<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Deleted::Success { old } => T::delete_entity(&state, &old),
            _ => Ok(()),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

///Nothing needed here
/// TODO: maybe have stored procedures here
impl UpdateActionFunctions for data::Query {
    fn create_entity(controller: &EntityModifierController, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Query, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }
}
