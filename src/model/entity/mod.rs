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

use self::error::DBError;
use self::results::*;



pub struct Retriever;
pub trait RetrieverFunctions<ET: RawEntityTypes, O>
    where
        O: ConvertRaw<ET::Data>
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

        impl<O> RetrieverFunctions<$EntityType, O> for Retriever
            where
                O: ConvertRaw<<$EntityType as RawEntityTypes>::Data>
        {
            fn get_all(
                conn: &Conn
            ) -> Result<Vec<O>, DBError> {
                internals::$entity::Retriever::get_all::<O>(conn)
            }

            fn get_one(
                conn: &Conn,
                name: &str,
            ) -> Result<Option<O>, DBError> {
                internals::$entity::Retriever::get_one::<O>(conn, name)
            }
        }
    );
}

make_retrievers!(table, data::Table);
make_retrievers!(query, data::Query);
make_retrievers!(script, data::Script);

pub struct Modifier;
pub trait ModifierFunctions<ET: RawEntityTypes, O>
    where
        O: GenerateRaw<ET::NewData>,
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
        impl<O> ModifierFunctions<$EntityType, O> for Modifier
            where
                O: GenerateRaw<<$EntityType as RawEntityTypes>::NewData>,
        {

            fn create(
                conn: &Conn,
                object: O,
            ) -> Result<Created<O>, DBError> {
                internals::$entity::Modifier::create::<O>(conn, object)
            }

            fn create_many(
                conn: &Conn,
                objects: &[O],
            ) -> Result<CreatedSet<O>, DBError> {
                internals::$entity::Modifier::create_many::<O>(conn, objects)
            }

            fn upsert(
                conn: &Conn,
                object: O,
            ) -> Result<Upserted<O>, DBError> {
                internals::$entity::Modifier::upsert::<O>(conn, object)
            }

            fn upsert_many(
                conn: &Conn,
                objects: &[O],
            ) -> Result<UpsertedSet<O>, DBError> {
                internals::$entity::Modifier::upsert_many::<O>(conn, objects)
            }

            fn update(
                conn: &Conn,
                name_object: (&str, O),
            ) -> Result<Updated<O>, DBError> {
                internals::$entity::Modifier::update::<O>(conn, name_object)
            }

            fn update_many(
                conn: &Conn,
                names_objects: &[(&str, O)],
            ) -> Result<UpdatedSet<O>, DBError> {
                internals::$entity::Modifier::update_many::<O>(conn, names_objects)
            }

            fn delete(
                conn: &Conn,
                name: &str,
            ) -> Result<Deleted<O>, DBError> {
                internals::$entity::Modifier::delete::<O>(conn, name)
            }

            fn delete_many(
                conn: &Conn,
                names: &[&str],
            ) -> Result<DeletedSet<O>, DBError> {
                internals::$entity::Modifier::delete_many::<O>(conn, names)
            }
        }
    );
}

make_modifiers!(table, data::Table);
make_modifiers!(query, data::Query);
make_modifiers!(script, data::Script);