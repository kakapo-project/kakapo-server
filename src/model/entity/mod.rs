use data::dbdata::RawEntityTypes;

mod internals;
pub mod error;
pub mod results;
mod update_state;

use self::error::EntityError;
use self::results::*;
use model::state::ActionState;
use model::state::GetConnection;

use self::internals::Retriever;
use self::internals::Modifier;
use self::update_state::UpdateState;
use model::entity::update_state::UpdateAction;
use model::entity::update_state::UpdateActionFunctions;
use std::fmt::Debug;


#[derive(Debug, Clone)]
pub struct Controller; //TODO: controller should be state agnostic (dependency inject)

pub trait RetrieverFunctions<O, S>
    where
        Self: Send + Debug,
        O: RawEntityTypes,
        S: GetConnection,
{
    /// get all values and returns a list of all database values
    fn get_all(
        conn: &S,
    ) -> Result<Vec<O>, EntityError>;

    /// filters the values by the name, and returns the value if it exists
    /// if it doesn't exist it retuns none
    fn get_one(
        conn: &S,
        name: &str,
    ) -> Result<Option<O>, EntityError>;
}

pub trait ModifierFunctions<O, S>
    where
        Self: Send + Debug,
        O: RawEntityTypes,
        S: GetConnection,
{
    ///creates the object if creation succeeded
    /// if name conflict, return the old value, creates nothing
    /// if value is created, returns nothing
    fn create(
        conn: &S,
        object: O,
    ) -> Result<Created<O>, EntityError>;

    /// if value is updated, return the old value
    /// if value is created, returns nothing
    fn upsert(
        conn: &S,
        object: O,
    ) -> Result<Upserted<O>, EntityError>;

    /// if value is updated, return the old value
    /// if name not found, returns nothing, updates nothing
    fn update(
        conn: &S,
        name_object: (&str, O),
    ) -> Result<Updated<O>, EntityError>;

    /// if value is deleted, return the old value
    /// if name not found, returns nothing
    fn delete(
        conn: &S,
        name: &str,
    ) -> Result<Deleted<O>, EntityError>;
}


impl<O> RetrieverFunctions<O, ActionState> for Controller
    where
        O: RawEntityTypes,
        Retriever: RetrieverFunctions<O, ActionState>,
{
    fn get_all(conn: &ActionState) -> Result<Vec<O>, EntityError> {
        Retriever::get_all(conn)
    }

    fn get_one(conn: &ActionState, name: &str) -> Result<Option<O>, EntityError> {
        Retriever::get_one(conn, name)
    }
}

impl<O> ModifierFunctions<O, ActionState> for Controller
    where
        O: RawEntityTypes,
        Created<O>: UpdateState<O>,
        Upserted<O>: UpdateState<O>,
        Updated<O>: UpdateState<O>,
        Deleted<O>: UpdateState<O>,
        UpdateAction: UpdateActionFunctions<O, ActionState>,
        Modifier: ModifierFunctions<O, ActionState>,
{
    fn create(conn: &ActionState, object: O) -> Result<Created<O>, EntityError> {
        Modifier::create(conn, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(conn)
            })
    }

    fn upsert(conn: &ActionState, object: O) -> Result<Upserted<O>, EntityError> {
        Modifier::upsert(conn, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(conn)
            })
    }

    fn update(conn: &ActionState, name_object: (&str, O)) -> Result<Updated<O>, EntityError> {
        Modifier::update(conn, name_object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(conn)
            })
    }

    fn delete(conn: &ActionState, name: &str) -> Result<Deleted<O>, EntityError> {
        Modifier::delete(conn, name)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(conn)
            })
    }
}