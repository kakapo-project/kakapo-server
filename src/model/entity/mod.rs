
pub mod error;
pub mod results;

mod conversion;
mod update_state;
mod internals;

use self::error::EntityError;
use self::results::*;
use model::state::ActionState;
use model::state::GetConnection;

use self::internals::InternalModifierFunctions;
use self::update_state::UpdateState;
use model::entity::update_state::UpdateActionFunctions;
use std::fmt::Debug;
use model::entity::internals::InternalRetrieverFunctions;
use connection::executor::Conn;
use model::state::AuthClaims;

pub use model::entity::conversion::RawEntityTypes;

pub struct Controller<'a> {
    pub conn: &'a Conn,
    pub claims: &'a Option<AuthClaims>,
}

pub trait RetrieverFunctions {
    /// get all values and returns a list of all database values
    fn get_all<O>(&self) -> Result<Vec<O>, EntityError>
        where
            O: RawEntityTypes;

    /// filters the values by the name, and returns the value if it exists
    /// if it doesn't exist it retuns none
    fn get_one<O>(&self, name: &str) -> Result<Option<O>, EntityError>
        where
            O: RawEntityTypes;
}

pub trait ModifierFunctions {
    ///creates the object if creation succeeded
    /// if name conflict, return the old value, creates nothing
    /// if value is created, returns nothing
    fn create<O>(&self, object: O) -> Result<Created<O>, EntityError>
        where O: RawEntityTypes;

    /// if value is updated, return the old value
    /// if value is created, returns nothing
    fn upsert<O>(&self, object: O) -> Result<Upserted<O>, EntityError>
        where O: RawEntityTypes;

    /// if value is updated, return the old value
    /// if name not found, returns nothing, updates nothing
    fn update<O>(&self, name_object: (&str, O)) -> Result<Updated<O>, EntityError>
        where O: RawEntityTypes;

    /// if value is deleted, return the old value
    /// if name not found, returns nothing
    fn delete<O>(&self, name: &str) -> Result<Deleted<O>, EntityError>
        where O: RawEntityTypes;
}


impl<'a> RetrieverFunctions for Controller<'a> {
    fn get_all<O>(&self) -> Result<Vec<O>, EntityError>
        where
            O: RawEntityTypes,
    {
        O::get_all(self)
    }

    fn get_one<O>(&self, name: &str) -> Result<Option<O>, EntityError>
        where
            O: RawEntityTypes,
    {
        O::get_one(self, name)
    }
}

impl<'a> ModifierFunctions for Controller<'a> {
    fn create<O>(&self, object: O) -> Result<Created<O>, EntityError>
        where O: RawEntityTypes,
    {
        O::create(self, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn upsert<O>(&self, object: O) -> Result<Upserted<O>, EntityError>
        where O: RawEntityTypes
    {
        O::upsert(self, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn update<O>(&self, name_object: (&str, O)) -> Result<Updated<O>, EntityError>
        where O: RawEntityTypes
    {
        O::update(self, name_object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn delete<O>(&self, name: &str) -> Result<Deleted<O>, EntityError>
        where O: RawEntityTypes
    {
        O::delete(self, name)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }
}