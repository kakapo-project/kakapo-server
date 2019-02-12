use model::entity::results::*;
use model::entity::error::EntityError;
use data;

use std::fmt::Debug;
use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;
use data::permissions::Permission;
use data::Named;
use model::state::auth::AuthFunctions;


pub trait UpdateActionFunctions
    where Self: UpdatePermissionFunctions
{
    fn create_entity(controller: &EntityModifierController, new: &Self) -> Result<(), EntityError>;
    fn update_entity(controller: &EntityModifierController, old_table: &Self, new_table: &Self) -> Result<(), EntityError>;
    fn delete_entity(controller: &EntityModifierController, old: &Self) -> Result<(), EntityError>;
}

pub trait UpdatePermissionFunctions {
    fn create_permission(controller: &EntityModifierController, new: &Self) -> Result<(), EntityError>;
    fn update_permission(controller: &EntityModifierController, old_table: &Self, new_table: &Self) -> Result<(), EntityError>;
    fn delete_permission(controller: &EntityModifierController, old: &Self) -> Result<(), EntityError>;
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
            Created::Success { new } => {
                T::create_entity(&state, &new)?;
                T::create_permission(&state, &new)?;
            },
            _ => (),
        };

        Ok(self)
    }
}

//Upserted
impl<T> UpdateState<T> for Upserted<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Upserted::Update { old, new } => {
                T::update_entity(&state, &old, &new)?;
                T::update_permission(&state, &old, &new)?;
            },
            Upserted::Create { new } => {
                T::create_entity(&state, &new)?;
                T::create_permission(&state, &new)?;
            },
        };

        Ok(self)
    }
}

//Updated
impl<T> UpdateState<T> for Updated<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Updated::Success { old, new } => {
                T::update_entity(&state, &old, &new)?;
                T::update_permission(&state, &old, &new)?;
            },
            _ => (),
        };

        Ok(self)
    }
}

//Deleted
impl<T> UpdateState<T> for Deleted<T>
    where T: Debug + RawEntityTypes + UpdateActionFunctions
{
    fn update_state(self, state: &EntityModifierController) -> Result<Self, EntityError> {
        let res = match &self {
            Deleted::Success { old } => {
                T::delete_entity(&state, &old)?;
                T::delete_permission(&state, &old)?;
            },
            _ => (),
        };

        Ok(self)
    }
}

///Nothing needed here
/// TODO: maybe have stored procedures here
impl UpdateActionFunctions for data::Query {
    fn create_entity(controller: &EntityModifierController, new: &data::Query) -> Result<(), EntityError> {
        //TODO: add create query delete query run query permissions
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Query, new: &data::Query) -> Result<(), EntityError> {
        //TODO: update create query delete query run query permissions
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Query) -> Result<(), EntityError> {
        //TODO: delete create query delete query run query permissions
        Ok(())
    }
}

///Nothing needed here
impl UpdateActionFunctions for data::View {
    fn create_entity(controller: &EntityModifierController, new: &data::View) -> Result<(), EntityError> {
        //TODO: add create query delete query run query permissions
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::View, new: &data::View) -> Result<(), EntityError> {
        //TODO: update create query delete query run query permissions
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::View) -> Result<(), EntityError> {
        //TODO: delete create query delete query run query permissions
        Ok(())
    }
}

impl UpdatePermissionFunctions for data::Query {
    fn create_permission(controller: &EntityModifierController, new: &data::Query) -> Result<(), EntityError> {
        let permission_list = vec![
            Permission::read_entity::<data::Query>(new.my_name().to_owned()),
            Permission::modify_entity::<data::Query>(new.my_name().to_owned()),
            Permission::run_query(new.my_name().to_owned()),
        ];

        //TODO: assuming that we are going to attach it to the user permission
        match controller.get_role_name() {
            Some(rolename) => for permission in permission_list {
                controller
                    .auth_permissions
                    .attach_permission_for_role(&permission, &rolename);
            },
            None => for permission in permission_list {
                controller
                    .auth_permissions
                    .add_permission(&permission);
            },
        };

        Ok(())
    }

    fn update_permission(controller: &EntityModifierController, old: &data::Query, new: &data::Query) -> Result<(), EntityError> {
        let old_name = old.my_name().to_owned();
        let new_name = new.my_name().to_owned();

        let permission_list = vec![
            (
                Permission::read_entity::<data::Query>(old_name.to_owned()),
                Permission::read_entity::<data::Query>(new_name.to_owned()),
            ),
            (
                Permission::modify_entity::<data::Query>(old_name.to_owned()),
                Permission::modify_entity::<data::Query>(new_name.to_owned()),
            ),
            (
                Permission::run_query(old_name.to_owned()),
                Permission::run_query(new_name.to_owned()),
            )
        ];

        for (old_permission, new_permission) in permission_list {
            controller
                .auth_permissions
                .rename_permission(&old_permission, &new_permission);
        }

        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::Query) -> Result<(), EntityError> {

        let permission_list = vec![
            Permission::read_entity::<data::Query>(old.my_name().to_owned()),
            Permission::modify_entity::<data::Query>(old.my_name().to_owned()),
            Permission::run_query(old.my_name().to_owned()),
        ];

        for permission in permission_list {
            controller
                .auth_permissions
                .remove_permission(&permission);
        }

        Ok(())
    }
}


///Nothing needed here
impl UpdatePermissionFunctions for data::View {
    fn create_permission(controller: &EntityModifierController, new: &data::View) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_permission(controller: &EntityModifierController, old: &data::View, new: &data::View) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::View) -> Result<(), EntityError> {
        Ok(())
    }
}

