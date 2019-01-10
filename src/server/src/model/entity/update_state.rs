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
            Created::Success { new } => UpdateAction::create_entity(&state, &new),
            _ => Ok(()),
        }?;

        //TODO: add proper permissions
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
            Upserted::Update { old, new } => UpdateAction::update_entity(&state, &old, &new),
            Upserted::Create { new } => UpdateAction::create_entity(&state, &new),
        }?;

        //TODO: add proper permissions
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
            Updated::Success { old, new } => UpdateAction::update_entity(&state, &old, &new),
            _ => Ok(()),
        }?;

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
            Deleted::Success { old } => UpdateAction::delete_entity(&state, &old),
            _ => Ok(()),
        }?;

        //TODO: remove permissions
        Ok(self)
    }
}

pub struct UpdateAction;
pub trait UpdateActionFunctions<T, S> {
    fn create_entity(conn: &S, new: &T) -> Result<(), EntityError>;
    fn update_entity(conn: &S, old_table: &T, new_table: &T) -> Result<(), EntityError>;
    fn delete_entity(conn: &S, old: &T) -> Result<(), EntityError>;
}

///mdodify table in database here
impl<B> UpdateActionFunctions<data::Table, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_entity(conn: &State<B>, new: &data::Table) -> Result<(), EntityError> {
        unimplemented!();
        let formatted_columns: Vec<String> = vec![];
        let command = format!("CREATE TABLE {} ({});", new.name, formatted_columns.join(", "));
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))
            .and_then(|res| Ok(()))
    }

    fn update_entity(conn: &State<B>, old: &data::Table, new: &data::Table) -> Result<(), EntityError> {
        unimplemented!();
        let command = format!("ALTER TABLE {};", old.name);
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))
            .and_then(|res| Ok(()))
    }

    fn delete_entity(conn: &State<B>, old: &data::Table) -> Result<(), EntityError> {
        let command = format!("DROP TABLE {};", old.name);
        diesel::sql_query(command)
            .execute(conn.get_conn())
            .or_else(|err|
                Err(EntityError::InternalError(err.description().to_string())))
            .and_then(|res| Ok(()))
    }
}

///Nothing needed here
/// TODO: maybe have stored procedures here
impl<B> UpdateActionFunctions<data::Query, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_entity(conn: &State<B>, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(conn: &State<B>, old: &data::Query, new: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(conn: &State<B>, old: &data::Query) -> Result<(), EntityError> {
        Ok(())
    }
}

///Nothing needed here
impl<B> UpdateActionFunctions<data::Script, State<B>> for UpdateAction
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn create_entity(conn: &State<B>, new: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }

    fn update_entity(conn: &State<B>, old: &data::Script, new: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }

    fn delete_entity(conn: &State<B>, old: &data::Script) -> Result<(), EntityError> {
        Ok(())
    }
}