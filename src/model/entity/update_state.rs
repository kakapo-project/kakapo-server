use model::entity::results::*;
use model::entity::error::EntityError;
use data;
use model::state::ActionState;

use std::error::Error;

use diesel::RunQueryDsl;
use model::state::GetConnection;
use model::state::GetScripting;
use data::DataType;
use std::fmt::Debug;

/// This trait does something action specific after the database updates
/// The name is a little bit confusing because the database store is also modification
/// But this module is responsible for all the other modifications
pub trait UpdateState<T>
    where
        Self: Sized,
        T: Debug,
{
    fn update_state(self, state: &ActionState) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, ActionState>;
}

//Created
impl<T> UpdateState<T> for Created<T>
    where T: Debug
{
    fn update_state(self, state: &ActionState) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, ActionState>,
    {
        info!("new: {:?}", &self);
        let res = match &self {
            Created::Success { new } => UpdateAction::create_entity(&state, &new),
            _ => Ok(()),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

//Upserted
impl<T> UpdateState<T> for Upserted<T>
    where T: Debug
{
    fn update_state(self, state: &ActionState) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, ActionState>,
    {
        let res = match &self {
            Upserted::Update { old, new } => UpdateAction::update_entity(&state, &old, &new),
            Upserted::Create { new } => UpdateAction::create_entity(&state, &new),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

//Updated
impl<T> UpdateState<T> for Updated<T>
    where T: Debug
{
    fn update_state(self, state: &ActionState) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, ActionState>,
    {
        let res = match &self {
            Updated::Success { old, new } => UpdateAction::update_entity(&state, &old, &new),
            _ => Ok(()),
        }?;

        Ok(self)
    }
}

//Deleted
impl<T> UpdateState<T> for Deleted<T>
    where T: Debug
{
    fn update_state(self, state: &ActionState) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, ActionState>,
    {
        let res = match &self {
            Deleted::Success { old } => UpdateAction::delete_entity(&state, &old),
            _ => Ok(()),
        }?;

        //TODO: add proper permissions in the db table
        //TODO: add database permissions
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct UpdateAction;
pub trait UpdateActionFunctions<T, S> {
    fn create_entity(conn: &S, new: &T) -> Result<(), EntityError>;
    fn update_entity(conn: &S, old_table: &T, new_table: &T) -> Result<(), EntityError>;
    fn delete_entity(conn: &S, old: &T) -> Result<(), EntityError>;
}

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
impl UpdateActionFunctions<data::Table, ActionState> for UpdateAction {
    fn create_entity(conn: &ActionState, new: &data::Table) -> Result<(), EntityError> {

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
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }

    fn update_entity(conn: &ActionState, old: &data::Table, new: &data::Table) -> Result<(), EntityError> {
        unimplemented!();
        let command = format!("ALTER TABLE \"{}\";", old.name);
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }

    fn delete_entity(conn: &ActionState, old: &data::Table) -> Result<(), EntityError> {
        let command = format!("DROP TABLE \"{}\";", old.name);
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))?;

        Ok(())
    }
}

///Nothing needed here
/// TODO: maybe have stored procedures here
impl UpdateActionFunctions<data::Query, ActionState> for UpdateAction {
    fn create_entity(conn: &ActionState, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(conn: &ActionState, old: &data::Query, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(conn: &ActionState, old: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }
}

///Nothing needed here
impl UpdateActionFunctions<data::Script, ActionState> for UpdateAction {
    fn create_entity(conn: &ActionState, new: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        let scripting = conn.get_scripting();
        //Scripting::build();
        Ok(())
    }

    fn update_entity(conn: &ActionState, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        let scripting = conn.get_scripting();
        //TODO: this should be debounced so that docker doesn't get called all the time
        //Scripting::build();
        Ok(())
    }

    fn delete_entity(conn: &ActionState, old: &data::Script) -> Result<(), EntityError> {
        unimplemented!();
        let scripting = conn.get_scripting();
        //Scripting::delete();
        Ok(())
    }
}