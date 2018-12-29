use diesel::prelude::*;

use super::schema;
use diesel::query_source::Table;
use diesel::r2d2::{ PooledConnection, ConnectionManager };
use diesel::pg::PgConnection;
use model::dbdata;
use data;



pub trait ConvertRaw<RD> {
    fn convert(raw: &RD) -> Self;
}

impl ConvertRaw<dbdata::RawTable> for data::Table {
    fn convert(raw: &dbdata::RawTable) -> data::Table {
        data::Table {
            name: "".to_string(),
            description: "".to_string(),
            schema: data::SchemaState {
                columns: vec![],
                constraint: vec![],
            },
        }
    }
}

impl ConvertRaw<dbdata::RawQuery> for data::Query {
    fn convert(raw: &dbdata::RawQuery) -> data::Query {
        data::Query {
            name: raw.name.to_owned(),
            description: raw.description.to_owned(),
            statement: raw.statement.to_owned(),
        }
    }
}

impl ConvertRaw<dbdata::RawScript> for data::Script {
    fn convert(raw: &dbdata::RawScript) -> data::Script {
        data::Script {
            name: raw.name.to_owned(),
            description: raw.description.to_owned(),
            text: raw.script_text.to_owned(),
        }
    }
}