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



pub struct Retriever;
pub trait RetrieverFunctions<O>
    where
        O: RawEntityTypes,
        O: ConvertRaw<<O as RawEntityTypes>::Data>
{
    /// get all values and returns a list of all database values
    fn get_all(
        conn: &Conn,
    ) -> Result<Vec<O>, DBError>;

    /// filters the values by the name, and returns the value if it exists
    /// if it doesn't exist it retuns none
    fn get_one(
        conn: &Conn,
        name: &str,
    ) -> Result<Option<O>, DBError>;
}

macro_rules! make_retrievers {
    ($entity:ident, $EntityType:ty) => (

        impl RetrieverFunctions<$EntityType> for Retriever {
            fn get_all(
                conn: &Conn
            ) -> Result<Vec<$EntityType>, DBError> {
                internals::$entity::Retriever::get_all::<$EntityType>(conn)
            }

            fn get_one(
                conn: &Conn,
                name: &str,
            ) -> Result<Option<$EntityType>, DBError> {
                internals::$entity::Retriever::get_one::<$EntityType>(conn, name)
            }
        }
    );
}

make_retrievers!(table, data::Table);
make_retrievers!(query, data::Query);
make_retrievers!(script, data::Script);

pub struct Modifier;
pub trait ModifierFunctions<O>
    where
        O: RawEntityTypes,
        O: GenerateRaw<<O as RawEntityTypes>::NewData>,
{
    ///creates the object if creation succeeded
    /// if name conflict, return the old value, creates nothing
    /// if value is created, returns nothing
    fn create(
        conn: &Conn,
        object: O,
    ) -> Result<Created<O>, DBError>;

    fn create_many(
        conn: &Conn,
        objects: &[O],
    ) -> Result<CreatedSet<O>, DBError>;

    /// if value is updated, return the old value
    /// if value is created, returns nothing
    fn upsert(
        conn: &Conn,
        object: O,
    ) -> Result<Upserted<O>, DBError>;
    fn upsert_many(
        conn: &Conn,
        objects: &[O],
    ) -> Result<UpsertedSet<O>, DBError>;

    /// if value is updated, return the old value
    /// if name not found, returns nothing, updates nothing
    fn update(
        conn: &Conn,
        name_object: (&str, O),
    ) -> Result<Updated<O>, DBError>;
    fn update_many(
        conn: &Conn,
        names_objects: &[(&str, O)],
    ) -> Result<UpdatedSet<O>, DBError>;

    /// if value is deleted, return the old value
    /// if name not found, returns nothing
    fn delete(
        conn: &Conn,
        name: &str,
    ) -> Result<Deleted<O>, DBError>;
    fn delete_many(
        conn: &Conn,
        names: &[&str],
    ) -> Result<DeletedSet<O>, DBError>;
}


macro_rules! make_modifiers {
    ($entity:ident, $EntityType:ty) => (
        impl ModifierFunctions<$EntityType> for Modifier {

            fn create(
                conn: &Conn,
                object: $EntityType,
            ) -> Result<Created<$EntityType>, DBError> {
                internals::$entity::Modifier::create::<$EntityType>(conn, object)
            }

            fn create_many(
                conn: &Conn,
                objects: &[$EntityType],
            ) -> Result<CreatedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::create_many::<$EntityType>(conn, objects)
            }

            fn upsert(
                conn: &Conn,
                object: $EntityType,
            ) -> Result<Upserted<$EntityType>, DBError> {
                internals::$entity::Modifier::upsert::<$EntityType>(conn, object)
            }

            fn upsert_many(
                conn: &Conn,
                objects: &[$EntityType],
            ) -> Result<UpsertedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::upsert_many::<$EntityType>(conn, objects)
            }

            fn update(
                conn: &Conn,
                name_object: (&str, $EntityType),
            ) -> Result<Updated<$EntityType>, DBError> {
                internals::$entity::Modifier::update::<$EntityType>(conn, name_object)
            }

            fn update_many(
                conn: &Conn,
                names_objects: &[(&str, $EntityType)],
            ) -> Result<UpdatedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::update_many::<$EntityType>(conn, names_objects)
            }

            fn delete(
                conn: &Conn,
                name: &str,
            ) -> Result<Deleted<$EntityType>, DBError> {
                internals::$entity::Modifier::delete::<$EntityType>(conn, name)
            }

            fn delete_many(
                conn: &Conn,
                names: &[&str],
            ) -> Result<DeletedSet<$EntityType>, DBError> {
                internals::$entity::Modifier::delete_many::<$EntityType>(conn, names)
            }
        }
    );
}

make_modifiers!(table, data::Table);
make_modifiers!(query, data::Query);
make_modifiers!(script, data::Script);