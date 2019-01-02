use diesel::prelude::*;

use model::schema;
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

pub trait GenerateRaw<NRD> {
    fn new(&self) -> NRD;
}

impl GenerateRaw<dbdata::NewRawTable> for data::Table {
    fn new(&self) -> dbdata::NewRawTable {
        dbdata::NewRawTable {
            entity_id: 1, //figure out
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            table_data: serde_json::to_value(self.schema.to_owned()).unwrap(),
            modified_by: 1, //figure out
        }
    }
}

impl GenerateRaw<dbdata::NewRawQuery> for data::Query {
    fn new(&self) -> dbdata::NewRawQuery {
        dbdata::NewRawQuery {
            entity_id: 1,//figure out
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            statement: "".to_string(),
            query_info: serde_json::to_value(json!({})).unwrap(),
            modified_by: 1,//figure out
        }
    }
}

impl GenerateRaw<dbdata::NewRawScript> for data::Script {
    fn new(&self) -> dbdata::NewRawScript {
        dbdata::NewRawScript {
            entity_id: 1,//figure out
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            script_language: "Python".to_string(),
            script_text: "".to_string(),
            script_info: serde_json::to_value(json!({})).unwrap(),
            modified_by: 1,//figure out
        }
    }
}