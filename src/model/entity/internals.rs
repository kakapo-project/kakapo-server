use diesel::prelude::*;

use model::schema;
use diesel::query_source::Table;
use model::dbdata;
use data;

use connection::executor::Conn;

use model::entity::conversion::*;
use model::dbdata::RawEntityTypes;

use model::entity::error::EntityError;
use model::entity::results::*;

use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::state::State;

use model::state::GetConnection;

pub struct Retriever;
macro_rules! make_retrievers {
    ($entity:ident, $EntityType:ty) => (

        impl RetrieverFunctions<$EntityType, State> for Retriever {
            fn get_all(
                conn: &State
            ) -> Result<Vec<$EntityType>, EntityError> {
                $entity::Retriever::get_all::<$EntityType>(conn.get_conn())
            }

            fn get_one(
                conn: &State,
                name: &str,
            ) -> Result<Option<$EntityType>, EntityError> {
                $entity::Retriever::get_one::<$EntityType>(conn.get_conn(), name)
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
            ) -> Result<Created<$EntityType>, EntityError> {
                $entity::Modifier::create::<$EntityType>(conn.get_conn(), object)
            }

            fn create_many(
                conn: &State,
                objects: &[$EntityType],
            ) -> Result<CreatedSet<$EntityType>, EntityError> {
                $entity::Modifier::create_many::<$EntityType>(conn.get_conn(), objects)
            }

            fn upsert(
                conn: &State,
                object: $EntityType,
            ) -> Result<Upserted<$EntityType>, EntityError> {
                $entity::Modifier::upsert::<$EntityType>(conn.get_conn(), object)
            }

            fn upsert_many(
                conn: &State,
                objects: &[$EntityType],
            ) -> Result<UpsertedSet<$EntityType>, EntityError> {
                $entity::Modifier::upsert_many::<$EntityType>(conn.get_conn(), objects)
            }

            fn update(
                conn: &State,
                name_object: (&str, $EntityType),
            ) -> Result<Updated<$EntityType>, EntityError> {
                $entity::Modifier::update::<$EntityType>(conn.get_conn(), name_object)
            }

            fn update_many(
                conn: &State,
                names_objects: &[(&str, $EntityType)],
            ) -> Result<UpdatedSet<$EntityType>, EntityError> {
                $entity::Modifier::update_many::<$EntityType>(conn.get_conn(), names_objects)
            }

            fn delete(
                conn: &State,
                name: &str,
            ) -> Result<Deleted<$EntityType>, EntityError> {
                $entity::Modifier::delete::<$EntityType>(conn.get_conn(), name)
            }

            fn delete_many(
                conn: &State,
                names: &[&str],
            ) -> Result<DeletedSet<$EntityType>, EntityError> {
                $entity::Modifier::delete_many::<$EntityType>(conn.get_conn(), names)
            }
        }
    );
}

make_modifiers!(table, data::Table);
make_modifiers!(query, data::Query);
make_modifiers!(script, data::Script);

//TODO: thesse macros are really bad. Use generics
macro_rules! implement_retriever_and_modifier {

    ($DataEntityType:ty, $data_table:ident) => {

        use super::*;

        type RD = <$DataEntityType as dbdata::RawEntityTypes>::Data;
        type NRD = <$DataEntityType as dbdata::RawEntityTypes>::NewData;

        pub struct Retriever;
        impl Retriever {


            pub fn get_all<O>(
                conn: &Conn,
            ) -> Result<Vec<O>, EntityError>
            where
                O: ConvertRaw<RD>,
            {
                let entities = schema::entity::table.load::<dbdata::RawEntity>(conn).unwrap();
                let raw_data = RD::belonging_to(&entities)
                    .order_by(schema::$data_table::modified_at.desc())
                    .load::<RD>(conn)
                    .unwrap() //TODO:!!!!
                    .grouped_by(&entities);

                let data = entities
                    .into_iter()
                    .zip(raw_data)

                    .map(|(entity, xs)| {
                        let new_xs = xs.first().map(|x| x.to_owned());
                        (entity, new_xs)
                    })
                    .filter(|(entity, x)| x.is_some())
                    .map(|(entity, xs)|  (entity, xs.unwrap()) ) //NOTE: this unwrap should be panic less, since we are already filtering all empty ones out
                    .filter(|(entity, x)| !x.is_deleted) //only if specified
                    .collect::<Vec<_>>();
                println!("queries: {:?}", data);

                Ok(vec![])
            }

            //TODO: put more of this into SQL, this is looping through all the values
            pub fn get_one<O>(
                conn: &Conn,
                name: &str,
            ) -> Result<Option<O>, EntityError>
            where
                O: ConvertRaw<RD>,
            {
                let entities = schema::entity::table.load::<dbdata::RawEntity>(conn).unwrap();
                let raw_data = RD::belonging_to(&entities)
                    .order_by(schema::$data_table::modified_at.desc())
                    .load::<RD>(conn)
                    .unwrap() //TODO:!!!!
                    .grouped_by(&entities);

                let data = entities
                    .into_iter()
                    .zip(raw_data)

                    .map(|(entity, xs)| {
                        let new_xs = xs.first().map(|x| x.to_owned());
                        (entity, new_xs)
                    })
                    .filter(|(entity, x)| x.is_some())
                    .map(|(entity, xs)|  (entity, xs.unwrap()) ) //NOTE: this unwrap should be panic less, since we are already filtering all empty ones out
                    .filter(|(entity, x)| !x.is_deleted) //only if specified
                    .filter(|(entity, x)| x.name == name)
                    .collect::<Vec<_>>();
                println!("queries: {:?}", data);

                Ok(None)

            }
        }

        pub struct Modifier;
        impl Modifier {
            pub fn create<O>(
                conn: &Conn,
                object: O,
            ) -> Result<Created<O>, EntityError>
            where
                O: GenerateRaw<NRD>,
            {
                //let db_object = object.
                Err(EntityError::Unknown)
            }

            pub fn create_many<O>(
                conn: &Conn,
                objects: &[O],
            ) -> Result<CreatedSet<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                for i in objects {

                }
                Err(EntityError::Unknown)
            }

            pub fn upsert<O>(
                conn: &Conn,
                object: O,
            ) -> Result<Upserted<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }

            pub fn upsert_many<O>(
                conn: &Conn,
                objects: &[O],
            ) -> Result<UpsertedSet<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }

            pub fn update<O>(
                conn: &Conn,
                name_object: (&str, O),
            ) -> Result<Updated<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }

            pub fn update_many<O>(
                conn: &Conn,
                names_objects: &[(&str, O)],
            ) -> Result<UpdatedSet<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }

            pub fn delete<O>(
                conn: &Conn,
                name: &str,
            ) -> Result<Deleted<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }

            pub fn delete_many<O>(
                conn: &Conn,
                names: &[&str],
            ) -> Result<DeletedSet<O>, EntityError>
            where
                O: GenerateRaw<NRD>
            {
                Err(EntityError::Unknown)
            }
        }
    }
}

pub mod table {
    implement_retriever_and_modifier!(data::Table, table_schema);
}

pub mod query {
    implement_retriever_and_modifier!(data::Query, query);
}

pub mod script {
    implement_retriever_and_modifier!(data::Script, script);
}