
use data::dbdata;
use data;

use serde_json;
use model::entity::internals::InternalRetrieverFunctions;
use model::entity::internals::InternalModifierFunctions;
use model::entity::update_state::UpdateActionFunctions;
use serde::Serialize;
use std::fmt::Debug;
use data::dbdata::RawTable;
use data::dbdata::NewRawTable;
use data::dbdata::RawQuery;
use data::dbdata::NewRawQuery;
use data::dbdata::RawScript;
use data::dbdata::NewRawScript;

pub trait RawEntityTypes
    where
        Self: Clone + Send + Debug + Serialize,
        Self::Data: ConvertRaw<Self>,
        Self::NewData: GenerateRaw<Self>,
        Self: InternalRetrieverFunctions + InternalModifierFunctions + UpdateActionFunctions,
{
    type Data;
    type NewData;

    fn get_name(&self) -> String;
}

pub trait ConvertRaw<T> {
    fn convert(&self) -> T;
}

impl ConvertRaw<data::Table> for dbdata::RawTable {
    fn convert(&self) -> data::Table {
        let schema: data::SchemaState = serde_json::from_value(self.table_data.to_owned())
            .unwrap_or_default(); //TODO: return serialization error!
        data::Table {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            schema,
        }
    }
}

impl ConvertRaw<data::Query> for dbdata::RawQuery {
    fn convert(&self) -> data::Query {
        data::Query {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            statement: self.statement.to_owned(),
        }
    }
}

impl ConvertRaw<data::Script> for dbdata::RawScript {
    fn convert(&self) -> data::Script {
        data::Script {
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            text: self.script_text.to_owned(),
        }
    }
}

pub trait GenerateRaw<T> {
    fn new(data: &T, entity_id: i64, modified_by: i64) -> Self;
    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self;
}

impl GenerateRaw<data::Table> for dbdata::NewRawTable {
    fn new(data: &data::Table, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawTable {
            entity_id,
            name: data.name.to_owned(),
            description: data.description.to_owned(),
            table_data: serde_json::to_value(data.schema.to_owned()).unwrap(),
            is_deleted: false,
            modified_by
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self {
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

impl GenerateRaw<data::Query> for dbdata::NewRawQuery {
    fn new(data: &data::Query, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawQuery {
            entity_id,
            name: data.name.to_owned(),
            description: data.description.to_owned(),
            statement: data.statement.to_owned(),
            query_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: false,
            modified_by
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self {
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

impl GenerateRaw<data::Script> for dbdata::NewRawScript {
    fn new(data: &data::Script, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawScript {
            entity_id,
            name: data.name.to_owned(),
            description: data.description.to_owned(),
            script_language: "Python".to_string(), //Only Python is supported right now
            script_text: data.text.to_owned(),
            script_info: serde_json::to_value(json!({})).unwrap(),
            is_deleted: false,
            modified_by,
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self {
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

impl RawEntityTypes for data::Table {
    type Data = RawTable;
    type NewData = NewRawTable;

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

impl RawEntityTypes for data::Query {
    type Data = RawQuery;
    type NewData = NewRawQuery;

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

impl RawEntityTypes for data::Script {
    type Data = RawScript;
    type NewData = NewRawScript;

    fn get_name(&self) -> String {
        self.name.to_owned()
    }
}