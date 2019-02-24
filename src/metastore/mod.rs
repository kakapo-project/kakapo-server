
pub mod user_management;
pub mod authorization;
pub mod authentication;
pub mod pub_sub;
mod conversion;
mod dbdata;
mod schema;

use std::fmt::Debug;

use data;

use connection::executor::Conn;

use metastore::dbdata::RawEntity;
use metastore::dbdata::NewRawEntity;
use model::entity::RawEntityTypes;
use model::entity::GenerateRaw;
use model::entity::ConvertRaw;


use model::entity::error::EntityError;
use model::entity::results::*;

use state::ActionState;
use model::entity::EntityModifierController;
use model::entity::EntityRetrieverController;


//TODO: put all of this in internal
pub trait EntityCrudOps
    where Self: Sized + Debug,
{
    fn get_all(state: &EntityRetrieverController) -> Result<Vec<Self>, EntityError>;

    fn get_one(state: &EntityRetrieverController, name: &str) -> Result<Option<Self>, EntityError>;

    fn create(state: &EntityModifierController, object: Self) -> Result<Created<Self>, EntityError>;

    fn upsert(state: &EntityModifierController, object: Self) -> Result<Upserted<Self>, EntityError>;

    fn update(state: &EntityModifierController, name_object: (&str, Self)) -> Result<Updated<Self>, EntityError>;

    fn delete(state: &EntityModifierController, name: &str) -> Result<Deleted<Self>, EntityError>;
}

const ADMIN_USER_ID: i64 = 1;

fn get_user_id(controller: &EntityModifierController) -> i64 {
    match controller.claims {
        None => {
            warn!("This user does not have any id, however, the user is authorized. Setting content as admin");
            ADMIN_USER_ID
        },
        Some(claims) => {
            claims.get_user_id()
        }
    }
}


macro_rules! make_crud_ops {
    ($entity:ident, $EntityType:ty) => (

        impl EntityCrudOps for $EntityType {

            fn get_all(state: &EntityRetrieverController) -> Result<Vec<$EntityType>, EntityError> {
                $entity::get_all::<$EntityType>(state.conn)
            }

            fn get_one(state: &EntityRetrieverController, name: &str) -> Result<Option<$EntityType>, EntityError> {
                $entity::get_one::<$EntityType>(state.conn, name)
            }

            fn create(state: &EntityModifierController, object: $EntityType) -> Result<Created<$EntityType>, EntityError> {
                info!("create object: {:?}", &object);
                $entity::create::<$EntityType>(state.conn, get_user_id(state), object)
            }

            fn upsert(state: &EntityModifierController, object: $EntityType) -> Result<Upserted<$EntityType>, EntityError> {
                info!("upsert object: {:?}", &object);
                $entity::upsert::<$EntityType>(state.conn, get_user_id(state), object)
            }

            fn update(state: &EntityModifierController, name_object: (&str, $EntityType)) -> Result<Updated<$EntityType>, EntityError> {
                info!("update object: {:?}", &name_object);
                $entity::update::<$EntityType>(state.conn, get_user_id(state), name_object)
            }

            fn delete(state: &EntityModifierController, name: &str) -> Result<Deleted<$EntityType>, EntityError> {
                info!("delete object: {:?}", &name);
                $entity::delete::<$EntityType>(state.conn, get_user_id(state), name)
            }
        }
    );
}


//TODO: thesse macros are really bad. Use generics
macro_rules! implement_retriever_and_modifier {

    ($DataEntityType:ty, $data_table:ident) => {

        use super::*;
        use diesel::RunQueryDsl;
        use std::error::Error;

        type RD = <$DataEntityType as RawEntityTypes>::Data;
        type NRD = <$DataEntityType as RawEntityTypes>::NewData;

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
            let new_raw_entity = NewRawEntity {
                scope_id: 1, //TODO: right now scopes haven't been implemented
                created_by: user_id,
            };
            let entity: RawEntity = diesel::insert_into(schema::entity::table)
                .values(&new_raw_entity)
                .get_result(conn)
                .or_else(|err| Err(EntityError::InternalError(err.description().to_string())))?;

            let new_raw_data = NRD::new(&object, entity.entity_id, user_id);
            let new_val: RD = diesel::insert_into(schema::$data_table::table)
                .values(&new_raw_data)
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

            let _new_val: RD = diesel::insert_into(schema::$data_table::table)
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
            debug!("string to create entity: \"name\": {}", object.my_name());
            let entities: Option<RD> = query_entities_by_name(conn, object.my_name().to_owned())?;

            match entities {
                Some(entity) => {
                    debug!("object already exists, old entity: {:?}", &entity);
                    Ok(Created::Fail {
                        existing: entity.convert()
                    })
                },
                None => {
                    debug!("no object found, putting object: {:?}", &object);
                    let new_val = create_internal(conn, user_id, object)?;
                    let converted = new_val.convert();
                    Ok(Created::Success {
                        new: converted,
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
            let entities: Option<RD> = query_entities_by_name(conn, object.my_name().to_owned())?;

            match entities {
                Some(entity) => {
                    let new_val = update_internal(conn, user_id, entity.entity_id, object)?;
                    Ok(Upserted::Update {
                        old: entity.convert(),
                        new: new_val.convert(),
                    })
                },
                None => {
                    let new_val = create_internal(conn, user_id, object)?;
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
                    let new_val = update_internal(conn, user_id, entity.entity_id, object)?;
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
                    delete_internal::<O>(conn, user_id, entity.entity_id, name.to_string())?;
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


make_crud_ops!(table, data::Table);
make_crud_ops!(query, data::Query);
make_crud_ops!(script, data::Script);
make_crud_ops!(view, data::View);

pub mod table {
    implement_retriever_and_modifier!(data::Table, table_schema);
}

pub mod query {
    implement_retriever_and_modifier!(data::Query, query);
}

pub mod script {
    implement_retriever_and_modifier!(data::Script, script);
}

pub mod view {
    implement_retriever_and_modifier!(data::View, view);
}