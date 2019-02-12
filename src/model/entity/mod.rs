
pub mod error;
pub mod results;

pub mod update_state;

use model::entity::error::EntityError;
use model::entity::results::*;

use serde::Serialize;

use metastore::EntityCrudOps;
use model::entity::update_state::UpdateState;
use model::entity::update_state::UpdateActionFunctions;
use std::fmt::Debug;
use connection::executor::Conn;
use data::claims::AuthClaims;
use scripting::Scripting;
use data::Named;
use model::entity::update_state::UpdatePermissionFunctions;
use metastore::auth_modifier::Auth;

pub trait RawEntityTypes
    where
        Self: Clone + Send + Debug + Serialize,
        Self::Data: ConvertRaw<Self>,
        Self::NewData: GenerateRaw<Self>,
        Self: EntityCrudOps,
        Self: Named,
{
    type Data;
    type NewData;
}

pub trait ConvertRaw<T> {
    fn convert(&self) -> T;
}

pub trait GenerateRaw<T> {
    fn new(data: &T, entity_id: i64, modified_by: i64) -> Self;
    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self;
}

pub struct EntityRetrieverController<'a> {
    pub conn: &'a Conn, //TODO: database specific, dependency inject here
    pub claims: &'a Option<AuthClaims>,
}

pub struct EntityModifierController<'a> {
    pub conn: &'a Conn, //TODO: database specific, dependency inject here
    pub claims: &'a Option<AuthClaims>,
    pub scripting: &'a Scripting,
    pub auth_permissions: Auth<'a>,
}

impl<'a> EntityModifierController<'a> {
    pub fn get_role_name(&self) -> Option<String> {
        self.claims
            .to_owned()
            .and_then(|claim| {
                claim.get_role()
            })
    }
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
        where O: RawEntityTypes + UpdateActionFunctions;

    /// if value is updated, return the old value
    /// if value is created, returns nothing
    fn upsert<O>(&self, object: O) -> Result<Upserted<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions;

    /// if value is updated, return the old value
    /// if name not found, returns nothing, updates nothing
    fn update<O>(&self, name_object: (&str, O)) -> Result<Updated<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions;

    /// if value is deleted, return the old value
    /// if name not found, returns nothing
    fn delete<O>(&self, name: &str) -> Result<Deleted<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions;
}


impl<'a> RetrieverFunctions for EntityRetrieverController<'a> {
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

impl<'a> ModifierFunctions for EntityModifierController<'a> {
    fn create<O>(&self, object: O) -> Result<Created<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions,
    {
        O::create(self, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn upsert<O>(&self, object: O) -> Result<Upserted<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions
    {
        O::upsert(self, object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn update<O>(&self, name_object: (&str, O)) -> Result<Updated<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions
    {
        O::update(self, name_object)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }

    fn delete<O>(&self, name: &str) -> Result<Deleted<O>, EntityError>
        where O: RawEntityTypes + UpdateActionFunctions
    {
        O::delete(self, name)
            .and_then(|res| {
                debug!("result in table, now updating state: {:?}", res);
                res.update_state(self)
            })
    }
}