use diesel::prelude::*;

use super::schema;
use diesel::query_source::Table;
use diesel::r2d2::{ PooledConnection, ConnectionManager };
use diesel::pg::PgConnection;
use model::dbdata;
use data;


//type RD = RawQuery;
//type RDH = RawQueryHistory;

pub trait ConvertRaw<RD, RDH> {
    fn convert(raw: &RD, raw_history: &RDH) -> Self;
}

impl ConvertRaw<dbdata::RawQuery, dbdata::RawQueryHistory> for data::Query {
    fn convert(raw: &dbdata::RawQuery, raw_history: &dbdata::RawQueryHistory) -> data::Query {
        data::Query {
            name: raw.name.to_owned(),
            description: raw_history.description.to_owned(),
            statement: raw_history.statement.to_owned(),
        }
    }
}

//TODO: this macro is really bad. Use generics
macro_rules! implement_retriever {

    ($DataEntityType:ident, $data_table:ident, $data_table_history:ident) => {

        use super::*;

        pub struct Retriever;
        type RD = <$DataEntityType as dbdata::RawEntityTypes>::Data;
        type RDH = <$DataEntityType as dbdata::RawEntityTypes>::DataHistory;

        impl Retriever {

            fn get_latest_history_object_from_db(
                conn: &PooledConnection<ConnectionManager<PgConnection>>,
                raw_query: &RD
            ) -> Result<RDH, ()> {
                RDH::belonging_to(raw_query)
                    .order_by(schema::$data_table_history::columns::modified_at.desc())
                    .get_result::<RDH>(conn)
                    .or_else(|err| Err(()))
            }

            pub fn get_all<O>(
                conn: &PooledConnection<ConnectionManager<PgConnection>>,
            ) -> Vec<O>
            where
                O: ConvertRaw<RD, RDH>,
            {
                let table = schema::$data_table::table;
                let raw_queries: Vec<RD> = table.load::<RD>(conn).unwrap();

                let results = raw_queries.iter()
                    .map(|raw_query| {

                        let raw_query_history = Retriever::get_latest_history_object_from_db(conn, raw_query).unwrap();

                        let final_result: O = ConvertRaw::<RD, RDH>::convert(&raw_query, &raw_query_history);

                        Ok(final_result)

                    })
                    .collect::<Result<Vec<O>, diesel::result::Error>>().unwrap();

                results
            }
        /*
            pub fn get_one() -> Result<Query, ()> {

                let one = db::get_one();
                one.into()

            }
            */
        }

    }

}

pub mod table {
    use model::dbdata::RawTableEntityTypes;

    implement_retriever!(RawTableEntityTypes, table_schema, table_schema_history);
}

pub mod query {
    use model::dbdata::RawQueryEntityTypes;

    implement_retriever!(RawQueryEntityTypes, query, query_history);
}

pub mod script {
    use model::dbdata::RawScriptEntityTypes;

    implement_retriever!(RawScriptEntityTypes, script, script_history);
}
