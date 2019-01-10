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
    fn new(&self, entity_id: i64, modified_by: i64) -> NRD;
    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> NRD;
}

impl GenerateRaw<dbdata::NewRawTable> for data::Table {
    fn new(&self, entity_id: i64, modified_by: i64) -> dbdata::NewRawTable {
        dbdata::NewRawTable {
            entity_id,
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            table_data: serde_json::to_value(self.schema.to_owned()).unwrap(),
            is_deleted: false,
            modified_by
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> dbdata::NewRawTable {
        dbdata::NewRawTable {
            entity_id,
            name: "".to_string(),
            description: "".to_string(),
            table_data: serde_json::to_value(json!({})).unwrap(),
            is_deleted: true,
            modified_by
        }
    }
}

impl GenerateRaw<dbdata::NewRawQuery> for data::Query {
    fn new(&self, entity_id: i64, modified_by: i64) -> dbdata::NewRawQuery {
        dbdata::NewRawQuery {
            entity_id,
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            statement: self.statement.to_owned(),
            query_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: false,
            modified_by
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> dbdata::NewRawQuery {
        dbdata::NewRawQuery {
            entity_id,
            name,
            description: "".to_string(),
            statement: "".to_string(),
            query_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: true,
            modified_by
        }
    }
}

impl GenerateRaw<dbdata::NewRawScript> for data::Script {
    fn new(&self, entity_id: i64, modified_by: i64) -> dbdata::NewRawScript {
        dbdata::NewRawScript {
            entity_id,
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            script_language: "Python".to_string(), //Only Python is supported right now
            script_text: self.text.to_owned(),
            script_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: false,
            modified_by,
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> dbdata::NewRawScript {
        dbdata::NewRawScript {
            entity_id,
            name,
            description: "".to_string(),
            script_language: "Python".to_string(),
            script_text: "".to_string(),
            script_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: true,
            modified_by,
        }
    }
}