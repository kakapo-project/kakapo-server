use diesel::prelude::*;

use model::schema;
use diesel::query_source::Table;
use model::dbdata;
use data;

use connection::executor::Conn;

use model::conversion::*;
use model::dbdata::RawEntityTypes;

use model::entity::error::DBError;
use model::entity::results::*;

//TODO: thesse macros are really bad. Use generics
macro_rules! implement_retriever_and_modifier {

    ($DataEntityType:ident, $data_table:ident) => {

        use super::*;

        type RD = <$DataEntityType as dbdata::RawEntityTypes>::Data;

        pub struct Retriever;
        impl Retriever {


            pub fn get_all<O>(
                conn: &Conn,
            ) -> Result<Vec<O>, DBError>
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
            ) -> Result<Option<O>, DBError>
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
            ) -> Result<Created<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn create_many<O>(
                conn: &Conn,
                objects: &[O],
            ) -> Result<CreatedSet<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn upsert<O>(
                conn: &Conn,
                object: O,
            ) -> Result<Upserted<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn upsert_many<O>(
                conn: &Conn,
                objects: &[O],
            ) -> Result<UpsertedSet<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn update<O>(
                conn: &Conn,
                name_object: (&str, O),
            ) -> Result<Updated<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn update_many<O>(
                conn: &Conn,
                names_objects: &[(&str, O)],
            ) -> Result<UpdatedSet<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn delete<O>(
                conn: &Conn,
                name: &str,
            ) -> Result<Deleted<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }

            pub fn delete_many<O>(
                conn: &Conn,
                names: &[&str],
            ) -> Result<DeletedSet<O>, DBError>
            where
                O: ConvertRaw<RD>,
            {
                Err(DBError::Unknown)
            }
        }
    }
}

pub mod table {
    use model::dbdata::RawTableEntityTypes;

    implement_retriever_and_modifier!(RawTableEntityTypes, table_schema);
}

pub mod query {
    use model::dbdata::RawQueryEntityTypes;

    implement_retriever_and_modifier!(RawQueryEntityTypes, query);
}

pub mod script {
    use model::dbdata::RawScriptEntityTypes;

    implement_retriever_and_modifier!(RawScriptEntityTypes, script);
}