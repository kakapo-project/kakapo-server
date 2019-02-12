
use std::fmt::Debug;

use serde_json;
use serde::Serialize;

use metastore::dbdata;

use data;
use data::Named;

use metastore::EntityCrudOps;
use metastore::dbdata::RawTable;
use metastore::dbdata::NewRawTable;
use metastore::dbdata::RawQuery;
use metastore::dbdata::NewRawQuery;
use metastore::dbdata::RawScript;
use metastore::dbdata::NewRawScript;
use metastore::dbdata::RawView;
use metastore::dbdata::NewRawView;
use model::entity::ConvertRaw;
use model::entity::GenerateRaw;
use model::entity::RawEntityTypes;


impl ConvertRaw<data::Table> for dbdata::RawTable {
    fn convert(&self) -> data::Table {
        let schema: data::SchemaState = serde_json::from_value(self.table_data.to_owned())
            .unwrap_or_default(); //TODO: return serialization error!
        data::Table {
            name: self.my_name().to_owned(),
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
            name: self.my_name().to_owned(),
            description: self.description.to_owned(),
            text: self.script_text.to_owned(),
        }
    }
}

impl ConvertRaw<data::View> for dbdata::RawView {
    fn convert(&self) -> data::View {
        data::View {
            name: self.my_name().to_owned(),
            description: self.description.to_owned(),
            view_state: self.view_state.to_owned(),
        }
    }
}


impl GenerateRaw<data::Table> for dbdata::NewRawTable {
    fn new(data: &data::Table, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawTable {
            entity_id,
            name: data.my_name().to_owned(),
            description: data.description.to_owned(),
            table_data: serde_json::to_value(data.schema.to_owned()).unwrap_or_default(),
            is_deleted: false,
            modified_by
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawTable {
            entity_id,
            name: "".to_string(),
            description: "".to_string(),
            table_data: serde_json::to_value(json!({})).unwrap_or_default(),
            is_deleted: true,
            modified_by
        }
    }
}

impl GenerateRaw<data::Query> for dbdata::NewRawQuery {
    fn new(data: &data::Query, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawQuery {
            entity_id,
            name: data.my_name().to_owned(),
            description: data.description.to_owned(),
            statement: data.statement.to_owned(),
            query_info: serde_json::to_value(json!({})).unwrap_or_default(),
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
            query_info: serde_json::to_value(json!({})).unwrap_or_default(),
            is_deleted: true,
            modified_by
        }
    }
}

impl GenerateRaw<data::Script> for dbdata::NewRawScript {
    fn new(data: &data::Script, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawScript {
            entity_id,
            name: data.my_name().to_owned(),
            description: data.description.to_owned(),
            script_language: "Python".to_string(), //Only Python is supported right now
            script_text: data.text.to_owned(),
            script_info: serde_json::to_value(json!({})).unwrap_or_default(),
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
            script_info: serde_json::to_value(json!({})).unwrap_or_default(),
            is_deleted: true,
            modified_by,
        }
    }
}

impl GenerateRaw<data::View> for dbdata::NewRawView {
    fn new(data: &data::View, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawView {
            entity_id,
            name: data.my_name().to_owned(),
            description: data.description.to_owned(),
            view_state: data.view_state.to_owned(),
            view_info: serde_json::to_value(json!({})).unwrap_or_default(),
            is_deleted: false,
            modified_by,
        }
    }

    fn tombstone(name: String, entity_id: i64, modified_by: i64) -> Self {
        dbdata::NewRawView {
            entity_id,
            name,
            description: "".to_string(),
            view_state: serde_json::to_value(json!({})).unwrap_or_default(),
            view_info: serde_json::to_value(json!({})).unwrap_or_default(),
            is_deleted: true,
            modified_by,
        }
    }
}

impl RawEntityTypes for data::Table {
    const TYPE_NAME: &'static str = "table";
    type Data = RawTable;
    type NewData = NewRawTable;

}

impl RawEntityTypes for data::Query {
    const TYPE_NAME: &'static str = "query";
    type Data = RawQuery;
    type NewData = NewRawQuery;

}

impl RawEntityTypes for data::Script {
    const TYPE_NAME: &'static str = "script";
    type Data = RawScript;
    type NewData = NewRawScript;

}

impl RawEntityTypes for data::View {
    const TYPE_NAME: &'static str = "view";
    type Data = RawView;
    type NewData = NewRawView;

}