
use model::entity::results::*;
use model::entity::error::EntityError;
use data;
use model::state::ActionState;

use std::error::Error;

use diesel::RunQueryDsl;
use data::DataType;
use std::fmt::Debug;
use model::entity::Controller;
use model::entity::RawEntityTypes;

use model::entity::update_state::UpdateActionFunctions;

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
        DataType::Time { with_tz } => format!("SMALLINT"),
        //DataType::TimeInterval,

        DataType::Boolean => format!("BOOLEAN"),

        DataType::Json => format!("JSON"),
    }
}


///mdodify table in database here
impl UpdateActionFunctions for data::Table {
    fn create_entity(controller: &Controller, new: &data::Table) -> Result<(), EntityError> {

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
        let command = format!("CREATE TABLE \"{}\" ({});", new.name, formatted_columns.join(", "));
        info!("DSL command: `{}`", &command);

        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }

    fn update_entity(controller: &Controller, old: &data::Table, new: &data::Table) -> Result<(), EntityError> {
        unimplemented!();
        let command = format!("ALTER TABLE \"{}\";", old.name);
        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }

    fn delete_entity(controller: &Controller, old: &data::Table) -> Result<(), EntityError> {
        let command = format!("DROP TABLE \"{}\";", old.name);
        diesel::sql_query(command)
            .execute(controller.conn)
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }
}