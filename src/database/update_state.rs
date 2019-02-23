
use diesel::RunQueryDsl;

use data;
use data::Named;
use data::DataType;

use model::entity::EntityModifierController;
use model::entity::error::EntityError;
use model::entity::update_state::UpdateActionFunctions;
use model::entity::update_state::UpdatePermissionFunctions;
use data::permissions::Permission;

use state::user_management::UserManagementOps;

fn get_sql_data_type(data_type: &DataType) -> String {
    match data_type {
        DataType::SmallInteger => format!("SMALLINT"),
        DataType::Integer => format!("INTEGER"),
        DataType::BigInteger => format!("BIGINT"),
        //DataType::Decimal { precision: u32, scale: u32 },
        DataType::Float => format!("REAL"),
        DataType::DoubleFloat => format!("DOUBLE PRECISION"),

        DataType::String => format!("TEXT"),
        DataType::VarChar { length } => format!("VARCHAR({})", length),

        DataType::Byte => format!("BYTEA"),

        DataType::Timestamp { with_tz } => match with_tz {
            true => format!("TIMESTAMP WITH TIME ZONE"),
            false => format!("TIMESTAMP"),
        },
        DataType::Date => format!("SMALLINT"),
        DataType::Time { with_tz } => format!("SMALLINT"), //TODO: with_tz
        //DataType::TimeInterval,

        DataType::Boolean => format!("BOOLEAN"),

        DataType::Json => format!("JSON"),
    }
}


///mdodify table in database here
impl UpdateActionFunctions for data::Table {
    fn create_entity(controller: &EntityModifierController, new: &data::Table) -> Result<(), EntityError> {

        let schema = &new.schema;
        let columns = &schema.columns;

        if columns.len() == 0 {
            Err(EntityError::NoColumns)?;
        }

        let formatted_columns: Vec<String> = columns.iter().map(|column| {
            let col_name = &column.name;
            let col_type = get_sql_data_type(&column.data_type);
            //TODO: nullable + default + serial
            format!("\"{}\" {}", col_name, col_type)
        }).collect();
        let command = format!("CREATE TABLE \"{}\" ({});", new.my_name(), formatted_columns.join(", "));
        info!("DSL command: `{}`", &command);

        //TODO: constraints...

        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.to_string())))?;

        //TODO: run DSL command to add permission to role

        Ok(())
    }

    fn update_entity(controller: &EntityModifierController, old: &data::Table, new: &data::Table) -> Result<(), EntityError> {
        unimplemented!();
        let command = format!("ALTER TABLE \"{}\";", old.my_name());
        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.to_string())))?;

        Ok(())
    }

    fn delete_entity(controller: &EntityModifierController, old: &data::Table) -> Result<(), EntityError> {
        let command = format!("DROP TABLE \"{}\";", old.my_name());
        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.to_string())))?;

        Ok(())
    }
}

///mdodify table permissions in database here
impl UpdatePermissionFunctions for data::Table {
    fn create_permission(controller: &EntityModifierController, new: &data::Table) -> Result<(), EntityError> {
        let permission_list = vec![
            Permission::read_entity::<data::Table>(new.my_name().to_owned()),
            Permission::modify_entity::<data::Table>(new.my_name().to_owned()),
            Permission::get_table_data(new.my_name().to_owned()),
            Permission::modify_table_data(new.my_name().to_owned()),
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

    fn update_permission(controller: &EntityModifierController, old: &data::Table, new: &data::Table) -> Result<(), EntityError> {
        let old_name = old.my_name().to_owned();
        let new_name = new.my_name().to_owned();

        let permission_list = vec![
            (
                Permission::read_entity::<data::Table>(old_name.to_owned()),
                Permission::read_entity::<data::Table>(new_name.to_owned()),
            ),
            (
                Permission::modify_entity::<data::Table>(old_name.to_owned()),
                Permission::modify_entity::<data::Table>(new_name.to_owned()),
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

        Ok(())
    }

    fn delete_permission(controller: &EntityModifierController, old: &data::Table) -> Result<(), EntityError> {

        let permission_list = vec![
            Permission::read_entity::<data::Table>(old.my_name().to_owned()),
            Permission::modify_entity::<data::Table>(old.my_name().to_owned()),
            Permission::get_table_data(old.my_name().to_owned()),
            Permission::modify_table_data(old.my_name().to_owned()),
        ];

        for permission in permission_list {
            controller
                .user_management
                .remove_permission(&permission);
        }

        Ok(())
    }
}