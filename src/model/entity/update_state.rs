use std::fmt::Debug;

use model::entity::results::*;
use model::entity::error::EntityError;
use model::entity::EntityModifierController;
use model::entity::RawEntityTypes;

use data;
use data::permissions::Permission;
use data::Named;

use state::user_management::UserManagementOps;

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
///maybe have stored procedures here for some speedup
impl UpdateActionFunctions for data::DataQueryEntity {
    fn create_entity(controller: &EntityModifierController, new: &data::DataQueryEntity) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::DataQueryEntity, new: &data::DataQueryEntity) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::DataQueryEntity) -> Result<(), EntityError> {
        Ok(())
    }
}

///Nothing needed here
impl UpdateActionFunctions for data::View {
    fn create_entity(controller: &EntityModifierController, new: &data::View) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::View, new: &data::View) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::View) -> Result<(), EntityError> {
        Ok(())
    }
}

//TODO: brind some othe the stuff from table here
///Nothing needed here
impl UpdateActionFunctions for data::DataStoreEntity {
    fn create_entity(controller: &EntityModifierController, new: &data::DataStoreEntity) -> Result<(), EntityError> {
        match controller.domain_conn {
            Ok(conn) => {
                conn.on_datastore_created(new)
                    .map_err(|err| EntityError::InternalError(err.to_string()))?;
            },
            Err(err) => {
                warn!("Could not get the controller for updating the state: {:?}", &err);
            }
        }

        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::DataStoreEntity, new: &data::DataStoreEntity) -> Result<(), EntityError> {
        match controller.domain_conn {
            Ok(conn) => {
                conn.on_datastore_updated(old, new)
                    .map_err(|err| EntityError::InternalError(err.to_string()))?;
            },
            Err(err) => {
                warn!("Could not get the controller for updating the state: {:?}", &err);
            }
        }

        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::DataStoreEntity) -> Result<(), EntityError> {
        match controller.domain_conn {
            Ok(conn) => {
                conn.on_datastore_deleted(old)
                    .map_err(|err| EntityError::InternalError(err.to_string()))?;
            },
            Err(err) => {
                warn!("Could not get the controller for updating the state: {:?}", &err);
            }
        }

        Ok(())
    }
}

impl UpdatePermissionFunctions for data::DataQueryEntity {
    fn create_permission(controller: &EntityModifierController, new: &data::DataQueryEntity) -> Result<(), EntityError> {
        /* TODO:...
        let permission_list = vec![
            Permission::read_entity::<Query>(new.my_name().to_owned()),
            Permission::modify_entity::<Query>(new.my_name().to_owned()),
            Permission::run_query(new.my_name().to_owned()),
        ];

        //TODO: assuming that we are going to attach it to the current user rolo
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
        */
        Ok(())
    }

    fn update_permission(controller: &EntityModifierController, old: &data::DataQueryEntity, new: &data::DataQueryEntity) -> Result<(), EntityError> {
        /* TODO:...
        let old_name = old.my_name().to_owned();
        let new_name = new.my_name().to_owned();

        let permission_list = vec![
           (
               Permission::read_entity::<Query>(old_name.to_owned()),
               Permission::read_entity::<Query>(new_name.to_owned()),
           ),
           (
               Permission::modify_entity::<Query>(old_name.to_owned()),
               Permission::modify_entity::<Query>(new_name.to_owned()),
           ),
           (
               Permission::run_query(old_name.to_owned()),
               Permission::run_query(new_name.to_owned()),
           )
        ];

        for (old_permission, new_permission) in permission_list {
           controller
               .user_management
               .rename_permission(&old_permission, &new_permission);
        }
       */
        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::DataQueryEntity) -> Result<(), EntityError> {
        /* TODO:...
        let permission_list = vec![
            Permission::read_entity::<Query>(old.my_name().to_owned()),
            Permission::modify_entity::<Query>(old.my_name().to_owned()),
            Permission::run_query(old.my_name().to_owned()),
        ];

        for permission in permission_list {
            controller
                .user_management
                .remove_permission(&permission);
        }
        */
        Ok(())
    }
}

///mdodify table permissions in database here
impl UpdatePermissionFunctions for data::DataStoreEntity {
    fn create_permission(controller: &EntityModifierController, new: &data::DataStoreEntity) -> Result<(), EntityError> {
        /* TODO:...
        let permission_list = vec![
            Permission::read_entity::<Table>(new.my_name().to_owned()),
            Permission::modify_entity::<Table>(new.my_name().to_owned()),
            Permission::get_table_data(new.my_name().to_owned()),
            Permission::modify_table_data(new.my_name().to_owned()),
        ];

        //TODO: assuming that we are going to attach it to the user permission, that should go inside the kakapo_postgres
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
        */
        Ok(())
    }

    fn update_permission(controller: &EntityModifierController, old: &data::DataStoreEntity, new: &data::DataStoreEntity) -> Result<(), EntityError> {
        /* TODO:...
        let old_name = old.my_name().to_owned();
        let new_name = new.my_name().to_owned();

        let permission_list = vec![
            (
                Permission::read_entity::<Table>(old_name.to_owned()),
                Permission::read_entity::<Table>(new_name.to_owned()),
            ),
            (
                Permission::modify_entity::<Table>(old_name.to_owned()),
                Permission::modify_entity::<Table>(new_name.to_owned()),
            ),
            (
                Permission::get_table_data(old_name.to_owned()),
                Permission::get_table_data(new_name.to_owned()),
            ),
            (
                Permission::modify_table_data(old_name.to_owned()),
                Permission::modify_table_data(new_name.to_owned()),
            )
        ];

        for (old_permission, new_permission) in permission_list {
            controller
                .user_management
                .rename_permission(&old_permission, &new_permission);
        }
        */
        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::DataStoreEntity) -> Result<(), EntityError> {
        /* TODO:...
        let permission_list = vec![
            Permission::read_entity::<Table>(old.my_name().to_owned()),
            Permission::modify_entity::<Table>(old.my_name().to_owned()),
            Permission::get_table_data(old.my_name().to_owned()),
            Permission::modify_table_data(old.my_name().to_owned()),
        ];

        for permission in permission_list {
            controller
                .user_management
                .remove_permission(&permission);
        }
        */
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

