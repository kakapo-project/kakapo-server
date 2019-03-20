
pub mod user_management;
pub mod domain_management;
pub mod authorization;
pub mod authentication;
pub mod pub_sub;
mod conversion;
mod dbdata;
mod schema;

use std::collections::HashMap;
use std::fmt::Debug;

use diesel::prelude::*;
use chrono::Utc;
use argonautica::Hasher;

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

use plugins::v1::Domain;


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

// Meta store helpers
pub fn sync_domains(database_url: &str, domains: &HashMap<String, Box<Domain>>) -> Result<(), String> {
    let conn = PgConnection::establish(&database_url)
        .map_err(|err| {
            error!("Could not create user, couldn't establish connection: {:?}", &err);
            err.to_string()
        })?;

    for (name, domain) in domains.iter() {
        let _ = diesel::insert_into(schema::domain::table)
            .values((
                schema::domain::columns::name.eq(name),
                schema::domain::columns::type_.eq(domain.domain_type()),
                schema::domain::columns::description.eq(""),
            ))
            .on_conflict(schema::domain::columns::name)
            .do_nothing()
            .execute(&conn)
            .map_err(|err| err.to_string())?;
    }

    Ok(())
}

pub fn setup_admin(database_url: &str, username: &str, email: &str, display_name: &str, password: &str) -> Result<(), String> {

    let id = ADMIN_USER_ID;

    let conn = PgConnection::establish(database_url)
        .map_err(|err| {
            error!("Could not create user, couldn't establish connection: {:?}", &err);
            err.to_string()
        })?;

    let password_secret = "Hello World Hello Wold";

    //TODO: test password complexity
    let mut hasher = Hasher::default();
    let password = hasher
        .with_password(password)
        .with_secret_key(password_secret)
        .hash()
        .map_err(|err| {
            error!("Could not create user, couldn't hash: {:?}", &err);
            err.to_string()
        })?;

    let result = diesel::insert_into(schema::user::table)
        .values((
            schema::user::columns::user_id.eq(id),
            schema::user::columns::username.eq(username),
            schema::user::columns::email.eq(email),
            schema::user::columns::display_name.eq(display_name),
            schema::user::columns::password.eq(&password),
            schema::user::columns::user_info.eq(json!({})),
            schema::user::columns::joined_at.eq(Utc::now().naive_utc()),
        ))
        .on_conflict(schema::user::columns::user_id)
        .do_update()
        .set((
            schema::user::columns::username.eq(username),
            schema::user::columns::email.eq(email),
            schema::user::columns::display_name.eq(display_name),
            schema::user::columns::password.eq(&password),
            schema::user::columns::user_info.eq(json!({})),
            schema::user::columns::joined_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&conn)
        .map_err(|err| err.to_string())?;

    info!("Admin user has been set up!");

    Ok(())
}

const ADMIN_USER_ID: i64 = 1;

fn get_user_id(controller: &EntityModifierController) -> Option<i64> {
    match controller.claims {
        None => {
            error!("This user does not have any id, however, the user is authorized. This isn't normal behavior, look into this");
            None
        },
        Some(claims) => {
            Some(claims.get_user_id())
        }
    }
}

fn get_domain_id(controller: &EntityModifierController) -> Option<i64> {
    let domain_name = match controller.get_domain_name() {
        Some(x) => x,
        None => return None,
    };

    let domain = schema::domain::table
        .filter(schema::domain::columns::name.eq(&domain_name))
        .get_result::<dbdata::RawDomainInfo>(controller.conn)
        .map_err(|err| {
            error!("get_domain_id error: {:?}", &err);
            err
        })
        .ok();

    domain.map(|x| x.domain_id)
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
                let user_id = get_user_id(state).ok_or_else(|| EntityError::Unknown)?;
                let domain_id = get_domain_id(state).ok_or_else(|| EntityError::Unknown)?;
                $entity::create::<$EntityType>(state.conn, user_id, domain_id, object)
            }

            fn upsert(state: &EntityModifierController, object: $EntityType) -> Result<Upserted<$EntityType>, EntityError> {
                info!("upsert object: {:?}", &object);
                let user_id = get_user_id(state).ok_or_else(|| EntityError::Unknown)?;
                let domain_id = get_domain_id(state).ok_or_else(|| EntityError::Unknown)?;
                $entity::upsert::<$EntityType>(state.conn, user_id, domain_id, object)
            }

            fn update(state: &EntityModifierController, name_object: (&str, $EntityType)) -> Result<Updated<$EntityType>, EntityError> {
                info!("update object: {:?}", &name_object);
                let user_id = get_user_id(state).ok_or_else(|| EntityError::Unknown)?;
                $entity::update::<$EntityType>(state.conn, user_id, name_object)
            }

            fn delete(state: &EntityModifierController, name: &str) -> Result<Deleted<$EntityType>, EntityError> {
                info!("delete object: {:?}", &name);
                let user_id = get_user_id(state).ok_or_else(|| EntityError::Unknown)?;
                $entity::delete::<$EntityType>(state.conn, user_id, name)
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
                SELECT entity_list.* FROM entity_list
                INNER JOIN entity
                    ON entity_list.entity_id = entity.entity_id
                INNER JOIN domain
                    ON entity.domain_id = domain.domain_id
                WHERE rn = 1 AND is_deleted = false AND entity_list.name = $1
                ORDER BY entity_list.name ASC;
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
                SELECT entity_list.* FROM entity_list
                INNER JOIN entity
                    ON entity_list.entity_id = entity.entity_id
                INNER JOIN domain
                    ON entity.domain_id = domain.domain_id
                WHERE rn = 1 {}
                ORDER BY entity_list.name ASC;
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
            domain_id: i64,
            object: O,
        ) -> Result<RD, EntityError>
            where
                O: RawEntityTypes,
                NRD: GenerateRaw<O>,
                RD: ConvertRaw<O>,
        {
            let new_raw_entity = NewRawEntity {
                scope_id: 1, //TODO: right now scopes haven't been implemented
                domain_id,
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
            domain_id: i64,
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
                    let new_val = create_internal(conn, user_id, domain_id, object)?;
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
            domain_id: i64,
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
                    let new_val = create_internal(conn, user_id, domain_id, object)?;
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


make_crud_ops!(table, data::DataStoreEntity);
make_crud_ops!(query, data::DataQueryEntity);
make_crud_ops!(script, data::Script);
make_crud_ops!(view, data::View);

pub mod table {
    implement_retriever_and_modifier!(data::DataStoreEntity, table_schema);
}

pub mod query {
    implement_retriever_and_modifier!(data::DataQueryEntity, query);
}

pub mod script {
    implement_retriever_and_modifier!(data::Script, script);
}

pub mod view {
    implement_retriever_and_modifier!(data::View, view);
}