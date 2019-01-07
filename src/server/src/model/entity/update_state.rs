use model::entity::results::*;
use model::entity::error::EntityError;
use data;
use model::state::State;
use model::state::ChannelBroadcaster;

use std::error::Error;
use model::table::error::TableError;

use diesel::RunQueryDsl;
use model::state::GetConnection;

/// This trait does something action specific after the database updates
/// The name is a little bit confusing because the database store is also modification
/// But this module is responsible for all the other modifications
pub trait UpdateState<T>
    where
        Self: Sized,
{
    fn update_state<B>(self, state: &State<B>) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, State<B>>,
            B: ChannelBroadcaster + Send + 'static;
}

//Created
impl<T> UpdateState<T> for Created<T> {
    fn update_state<B>(self, state: &State<B>) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, State<B>>,
            B: ChannelBroadcaster + Send + 'static,
    {
        let res = match &self {
            Created::Success { new } => UpdateAction::create_table(&state, &new),
            _ => Ok(()),
        }?;

        //add proper permissions
        Ok(self)
    }
}

//Upserted
impl<T> UpdateState<T> for Upserted<T> {
    fn update_state<B>(self, state: &State<B>) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, State<B>>,
            B: ChannelBroadcaster + Send + 'static,
    {
        let res = match &self {
            Upserted::Update { old, new } => UpdateAction::update_table(&state, &old, &new),
            Upserted::Create { new } => UpdateAction::create_table(&state, &new),
        }?;

        //add proper permissions
        Ok(self)
    }
}

//Updated
impl<T> UpdateState<T> for Updated<T> {
    fn update_state<B>(self, state: &State<B>) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, State<B>>,
            B: ChannelBroadcaster + Send + 'static,
    {
        let res = match &self {
            Updated::Success { old, new } => UpdateAction::update_table(&state, &old, &new),
            _ => Ok(()),
        }?;

        //add proper permissions
        Ok(self)
    }
}

//Deleted
impl<T> UpdateState<T> for Deleted<T> {
    fn update_state<B>(self, state: &State<B>) -> Result<Self, EntityError>
        where
            UpdateAction: UpdateActionFunctions<T, State<B>>,
            B: ChannelBroadcaster + Send + 'static,
    {
        let res = match &self {
            Deleted::Success { old } => UpdateAction::delete_table(&state, &old),
            _ => Ok(()),
        }?;

        //add proper permissions
        Ok(self)
    }
}

pub struct UpdateAction;
pub trait UpdateActionFunctions<T, S> {
    fn create_table(conn: &S, new: &T) -> Result<(), EntityError>;
    fn update_table(conn: &S, old_table: &T, new_table: &T) -> Result<(), EntityError>;
    fn delete_table(conn: &S, old: &T) -> Result<(), EntityError>;
}

impl<B> UpdateActionFunctions<data::Table, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_table(conn: &State<B>, new: &data::Table) -> Result<(), EntityError> {
        let formatted_columns = vec!["test TEXT", "test2 TEXT"];
        let command = format!("CREATE TABLE {} ({});", new.name, formatted_columns.join(", "));
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))
            .and_then(|res| Ok(()))
    }

    fn update_table(conn: &State<B>, old_table: &data::Table, new_table: &data::Table) -> Result<(), EntityError> {

        Ok(())
    }

    fn delete_table(conn: &State<B>, old: &data::Table) -> Result<(), EntityError> {
        let command = format!("DROP TABLE {};", old.name);
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))
            .and_then(|res| Ok(()))
    }
}


impl<B> UpdateActionFunctions<data::Query, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_table(conn: &State<B>, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_table(conn: &State<B>, old_table: &data::Query, new_table: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_table(conn: &State<B>, old: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }
}


impl<B> UpdateActionFunctions<data::Script, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_table(conn: &State<B>, new: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_table(conn: &State<B>, old_table: &data::Script, new_table: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_table(conn: &State<B>, old: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }
}