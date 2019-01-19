use diesel::prelude::*;

use model::schema;
use diesel::query_source::Table;
use model::dbdata;
use data;

use connection::executor::Conn;

use model::entity::conversion::*;
use model::dbdata::RawEntityTypes;
use model::dbdata::RawEntity;
use model::dbdata::NewRawEntity;


use model::entity::error::EntityError;
use model::entity::results::*;

use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::state::State;

use model::state::GetConnection;
use model::state::GetUserInfo;

use std::error::Error;

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
                $entity::Modifier::create::<$EntityType>(conn.get_conn(), conn.get_user_id(), object)
            }

            fn upsert(
                conn: &State,
                object: $EntityType,
            ) -> Result<Upserted<$EntityType>, EntityError> {
                $entity::Modifier::upsert::<$EntityType>(conn.get_conn(), conn.get_user_id(), object)
            }

            fn update(
                conn: &State,
                name_object: (&str, $EntityType),
            ) -> Result<Updated<$EntityType>, EntityError> {
                $entity::Modifier::update::<$EntityType>(conn.get_conn(), conn.get_user_id(), name_object)
            }

            fn delete(
                conn: &State,
                name: &str,
            ) -> Result<Deleted<$EntityType>, EntityError> {
                $entity::Modifier::delete::<$EntityType>(conn.get_conn(), conn.get_user_id(), name)
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

        fn query_entities_by_name(conn: &Conn, name: String) -> Result<Option<RD>, EntityError> {
            let query = format!(r#"
                WITH entity_list AS (
                    SELECT m.*, ROW_NUMBER() OVER (PARTITION BY entity_id ORDER BY modified_at DESC) AS rn
                    FROM {} AS m
                )
                SELECT * FROM entity_list
                WHERE rn = 1 AND is_deleted = false AND name = $1
                ORDER BY name ASC;
                "#, stringify!($data_table));

            let result = diesel::sql_query(query)
                .bind::<diesel::sql_types::Text, _>(name)
                .load(conn)
                .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

            if result.len() > 1 {
                return Err(EntityError::InvalidState);
            }

            Ok(result.first().map(|x: &RD| x.to_owned()))
        }

        fn query_all_entities(conn: &Conn, show_deleted: bool) -> Result<Vec<RD>, EntityError> {
            let query = format!(r#"
                WITH entity_list AS (
                    SELECT m.*, ROW_NUMBER() OVER (PARTITION BY entity_id ORDER BY modified_at DESC) AS rn
                    FROM {} AS m
                )
                SELECT * FROM entity_list
                WHERE rn = 1 {}
                ORDER BY name ASC;
                "#, stringify!($data_table), if show_deleted { "" } else { "AND is_deleted = false" });

            let result = diesel::sql_query(query)
                .load(conn)
                .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

            Ok(result)
        }

        pub struct Retriever;
        impl Retriever {


            pub fn get_all<O>(
                conn: &Conn,
            ) -> Result<Vec<O>, EntityError>
            where
                RD: ConvertRaw<O>,
            {
                let entities: Vec<RD> = query_all_entities(conn, false)?;

                let ok_result = entities.into_iter()
                    .map(|entity| entity.convert())
                    .collect();

                Ok(ok_result)
            }

            pub fn get_one<O>(
                conn: &Conn,
                name: &str,
            ) -> Result<Option<O>, EntityError>
            where
                RD: ConvertRaw<O>,
            {
                let entities: Option<RD> = query_entities_by_name(conn, name.to_string())?;

                let ok_result = match entities {
                    Some(entity) => Some(entity.convert()),
                    None => None
                };

                Ok(ok_result)
            }
        }

        pub struct Modifier;
        impl Modifier {

            fn create_internal<O>(
                conn: &Conn,
                user_id: i64,
                object: O,
            ) -> Result<RD, EntityError>
                where
                    O: RawEntityTypes,
                    NRD: GenerateRaw<O>,
                    RD: ConvertRaw<O>,
            {
                debug!("creating entity");
                let entity: RawEntity = diesel::insert_into(schema::entity::table)
                    .values(&NewRawEntity {
                        scope_id: 1, //TODO: right now scopes haven't been implemented
                        created_by: user_id,
                    })
                    .get_result(conn)
                    .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

                debug!("creating entity history");
                let new_val: RD = diesel::insert_into(schema::$data_table::table)
                    .values(NRD::new(&object, entity.entity_id, user_id))
                    .get_result(conn)
                    .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

                Ok(new_val)
            }

            fn update_internal<O>(
                conn: &Conn,
                user_id: i64,
                entity_id: i64,
                object: O,
            ) -> Result<RD, EntityError>
                where
                    O: RawEntityTypes,
                    NRD: GenerateRaw<O>,
                    RD: ConvertRaw<O>,
            {

                let new_val: RD = diesel::insert_into(schema::$data_table::table)
                    .values(NRD::new(&object, entity_id, user_id))
                    .get_result(conn)
                    .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

                Ok(new_val)
            }

            fn delete_internal<O>(
                conn: &Conn,
                user_id: i64,
                entity_id: i64,
                entity_name: String,
            ) -> Result<(), EntityError>
                where
                    O: RawEntityTypes,
                    NRD: GenerateRaw<O>,
                    RD: ConvertRaw<O>,
            {

                let new_val: RD = diesel::insert_into(schema::$data_table::table)
                    .values(NRD::tombstone(entity_name, entity_id, user_id))
                    .get_result(conn)
                    .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

                Ok(())
            }


            pub fn create<O>(
                conn: &Conn,
                user_id: i64,
                object: O,
            ) -> Result<Created<O>, EntityError>
            where
                O: RawEntityTypes,
                NRD: GenerateRaw<O>,
                RD: ConvertRaw<O>,
            {
                debug!("string to create entity: name => {}", object.get_name());
                let entities: Option<RD> = query_entities_by_name(conn, object.get_name())?;

                match entities {
                    Some(entity) => Ok(Created::Fail {
                        existing: entity.convert()
                    }),
                    None => {
                        debug!("no object found");
                        let new_val = Modifier::create_internal(conn, user_id, object)?;
                        Ok(Created::Success {
                            new: new_val.convert(),
                        })
                    }
                }
            }

            pub fn upsert<O>(
                conn: &Conn,
                user_id: i64,
                object: O,
            ) -> Result<Upserted<O>, EntityError>
            where
                O: RawEntityTypes,
                NRD: GenerateRaw<O>,
                RD: ConvertRaw<O>,
            {
                let entities: Option<RD> = query_entities_by_name(conn, object.get_name())?;

                match entities {
                    Some(entity) => {
                        let new_val = Modifier::update_internal(conn, user_id, entity.entity_id, object)?;
                        Ok(Upserted::Update {
                            old: entity.convert(),
                            new: new_val.convert(),
                        })
                    },
                    None => {
                        let new_val = Modifier::create_internal(conn, user_id, object)?;
                        Ok(Upserted::Create {
                            new: new_val.convert(),
                        })
                    }
                }
            }

            pub fn update<O>(
                conn: &Conn,
                user_id: i64,
                name_object: (&str, O),
            ) -> Result<Updated<O>, EntityError>
            where
                O: RawEntityTypes,
                NRD: GenerateRaw<O>,
                RD: ConvertRaw<O>,
            {
                let (object_name, object) = name_object;
                let entities: Option<RD> = query_entities_by_name(conn, object_name.to_string())?;

                match entities {
                    Some(entity) => {
                        let new_val = Modifier::update_internal(conn, user_id, entity.entity_id, object)?;
                        Ok(Updated::Success {
                            old: entity.convert(),
                            new: new_val.convert(),
                        })
                    },
                    None => {
                        Ok(Updated::Fail)
                    }
                }
            }

            pub fn delete<O>(
                conn: &Conn,
                user_id: i64,
                name: &str,
            ) -> Result<Deleted<O>, EntityError>
            where
                O: RawEntityTypes,
                NRD: GenerateRaw<O>,
                RD: ConvertRaw<O>,
            {
                let entities: Option<RD> = query_entities_by_name(conn, name.to_string())?;

                match entities {
                    Some(entity) => {
                        Modifier::delete_internal::<O>(conn, user_id, entity.entity_id, name.to_string())?;
                        Ok(Deleted::Success {
                            old: entity.convert(),
                        })
                    },
                    None => {
                        Ok(Deleted::Fail)
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;
    use std::env;
    use diesel::result::Error;

    use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
    use diesel::r2d2::Pool;

    pub fn establish_connection() -> Conn {
        dotenv().ok(); //assuming the user has a postgres repo setup

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("DATABASE_USER").expect("var not set"),
            env::var("DATABASE_PASS").expect("var not set"),
            env::var("DATABASE_HOST").expect("var not set"),
            env::var("DATABASE_PORT").expect("var not set"),
            env::var("DATABASE_DB").expect("var not set"),
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder().build(manager)
            .expect("Could not start connection")
            .get()
            .unwrap()
    }

    #[test]
    fn add_query_to_empty_table() {
        let conn = establish_connection();
        conn.test_transaction::<_, Error, _>(|| {

            let object = data::Query {
                name: "get_all_eggs".to_string(),
                description: "This query gets all the values from the eggs table".to_string(),
                statement: "SELECT * FROM eggs".to_string(),
            };

            let result = query::Modifier::create(&conn, 1, object).expect("Could not create value");

            let get_back_maybe = query::Retriever::get_one(&conn, "get_all_eggs").expect("could not retrieve back value");

            let get_back: data::Query = get_back_maybe.unwrap();
            assert_eq!(get_back.name, "get_all_eggs");
            assert_eq!(get_back.description, "This query gets all the values from the eggs table");
            assert_eq!(get_back.statement, "SELECT * FROM eggs");

            Ok(())
        });
    }
}