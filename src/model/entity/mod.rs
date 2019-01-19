use diesel::prelude::*;

use diesel::query_source::Table;
use model::dbdata;
use data;

use connection::executor::Conn;

use model::entity::conversion::*;
use model::dbdata::RawEntityTypes;

mod internals;
pub mod error;
pub mod results;
pub mod conversion;
mod update_state;

use self::error::EntityError;
use self::results::*;
use model::state::State;
use model::state::GetConnection;

use self::internals::Retriever;
use self::internals::Modifier;
use self::update_state::UpdateState;
use model::entity::update_state::UpdateAction;
use model::entity::update_state::UpdateActionFunctions;


pub struct Controller; //TODO: controller should be state agnostic (dependency inject)

pub trait RetrieverFunctions<O, S>
    where
        Self: Send,
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
        Self: Send,
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


impl<O> RetrieverFunctions<O, State> for Controller
    where
        O: RawEntityTypes,
        Retriever: RetrieverFunctions<O, State>,
{
    fn get_all(conn: &State) -> Result<Vec<O>, EntityError> {
        Retriever::get_all(conn)
    }

    fn get_one(conn: &State, name: &str) -> Result<Option<O>, EntityError> {
        Retriever::get_one(conn, name)
    }
}

impl<O> ModifierFunctions<O, State> for Controller
    where
        O: RawEntityTypes,
        Created<O>: UpdateState<O>,
        Upserted<O>: UpdateState<O>,
        Updated<O>: UpdateState<O>,
        Deleted<O>: UpdateState<O>,
        UpdateAction: UpdateActionFunctions<O, State>,
        Modifier: ModifierFunctions<O, State>,
{
    fn create(conn: &State, object: O) -> Result<Created<O>, EntityError> {
        Modifier::create(conn, object)
            .and_then(|res| res.update_state(conn))
    }

    fn upsert(conn: &State, object: O) -> Result<Upserted<O>, EntityError> {
        Modifier::upsert(conn, object)
            .and_then(|res| res.update_state(conn))
    }

    fn update(conn: &State, name_object: (&str, O)) -> Result<Updated<O>, EntityError> {
        Modifier::update(conn, name_object)
            .and_then(|res| res.update_state(conn))
    }

    fn delete(conn: &State, name: &str) -> Result<Deleted<O>, EntityError> {
        Modifier::delete(conn, name)
            .and_then(|res| res.update_state(conn))
    }
}