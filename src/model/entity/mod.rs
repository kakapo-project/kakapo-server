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
mod post_modification;

use self::error::DBError;
use self::results::*;
use model::state::State;
use model::state::GetConnection;


pub struct Controller;

pub trait RetrieverFunctions<O, S>
    where
        O: RawEntityTypes,
        O: ConvertRaw<<O as RawEntityTypes>::Data>,
        S: GetConnection,
{
    /// get all values and returns a list of all database values
    fn get_all(
        conn: &S,
    ) -> Result<Vec<O>, DBError>;

    /// filters the values by the name, and returns the value if it exists
    /// if it doesn't exist it retuns none
    fn get_one(
        conn: &S,
        name: &str,
    ) -> Result<Option<O>, DBError>;
}

pub trait ModifierFunctions<O, S>
    where
        O: RawEntityTypes,
        O: GenerateRaw<<O as RawEntityTypes>::NewData>,
        S: GetConnection,
{
    ///creates the object if creation succeeded
    /// if name conflict, return the old value, creates nothing
    /// if value is created, returns nothing
    fn create(
        conn: &S,
        object: O,
    ) -> Result<Created<O>, DBError>;

    fn create_many(
        conn: &S,
        objects: &[O],
    ) -> Result<CreatedSet<O>, DBError>;

    /// if value is updated, return the old value
    /// if value is created, returns nothing
    fn upsert(
        conn: &S,
        object: O,
    ) -> Result<Upserted<O>, DBError>;
    fn upsert_many(
        conn: &S,
        objects: &[O],
    ) -> Result<UpsertedSet<O>, DBError>;

    /// if value is updated, return the old value
    /// if name not found, returns nothing, updates nothing
    fn update(
        conn: &S,
        name_object: (&str, O),
    ) -> Result<Updated<O>, DBError>;
    fn update_many(
        conn: &S,
        names_objects: &[(&str, O)],
    ) -> Result<UpdatedSet<O>, DBError>;

    /// if value is deleted, return the old value
    /// if name not found, returns nothing
    fn delete(
        conn: &S,
        name: &str,
    ) -> Result<Deleted<O>, DBError>;
    fn delete_many(
        conn: &S,
        names: &[&str],
    ) -> Result<DeletedSet<O>, DBError>;
}

pub struct Retriever;
macro_rules! make_retrievers {
    ($entity:ident, $EntityType:ty) => (

        impl RetrieverFunctions<$EntityType, State> for Retriever {
            fn get_all(
                conn: &State
            ) -> Result<Vec<$EntityType>, DBError> {
                internals::$entity::Retriever::get_all::<$EntityType>(conn.get_conn())
            }

            fn get_one(
                conn: &State,
                name: &str,
            ) -> Result<Option<$EntityType>, DBError> {
                internals::$entity::Retriever::get_one::<$EntityType>(conn.get_conn(), name)
            }
        }
    );
}

make_retrievers!(table, data::Table);
make_retrievers!(query, data::Query);
make_retrievers!(script, data::Script);

pub struct Modifier;
macro_rules! make_modifiers {
    ($entity:ident, $EntityType:ty) => (
        impl ModifierFunctions<$EntityType, State> for Modifier {

            fn create(
                conn: &State,
                object: $EntityType,
            ) -> Result<Created<$EntityType>, DBError> {
                internals::$entity::Modifier::create::<$EntityType>(conn.get_conn(), object)
            }

            fn create_many(
                conn: &State,
                objects: &[$EntityType],
            ) -> Result<CreatedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::create_many::<$EntityType>(conn.get_conn(), objects)
            }

            fn upsert(
                conn: &State,
                object: $EntityType,
            ) -> Result<Upserted<$EntityType>, DBError> {
                internals::$entity::Modifier::upsert::<$EntityType>(conn.get_conn(), object)
            }

            fn upsert_many(
                conn: &State,
                objects: &[$EntityType],
            ) -> Result<UpsertedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::upsert_many::<$EntityType>(conn.get_conn(), objects)
            }

            fn update(
                conn: &State,
                name_object: (&str, $EntityType),
            ) -> Result<Updated<$EntityType>, DBError> {
                internals::$entity::Modifier::update::<$EntityType>(conn.get_conn(), name_object)
            }

            fn update_many(
                conn: &State,
                names_objects: &[(&str, $EntityType)],
            ) -> Result<UpdatedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::update_many::<$EntityType>(conn.get_conn(), names_objects)
            }

            fn delete(
                conn: &State,
                name: &str,
            ) -> Result<Deleted<$EntityType>, DBError> {
                internals::$entity::Modifier::delete::<$EntityType>(conn.get_conn(), name)
            }

            fn delete_many(
                conn: &State,
                names: &[&str],
            ) -> Result<DeletedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::delete_many::<$EntityType>(conn.get_conn(), names)
            }
        }
    );
}

make_modifiers!(table, data::Table);
make_modifiers!(query, data::Query);
make_modifiers!(script, data::Script);