
use serde_json;
use linked_hash_map::LinkedHashMap;

pub mod utils;
pub mod auth;
pub mod claims;
pub mod channels;
pub mod permissions;
pub mod error;

pub trait Named {
    fn my_name(&self) -> &str;
}

pub trait GetDomainId {
    fn my_domain_id(&self) -> i64;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreEntity {
    pub name: String, //TODO: make sure this is an alphanumeric
    //pub domain_id: i64,
    pub description: String,
    pub schema: serde_json::Value,
}

impl Named for DataStoreEntity {
    fn my_name(&self) -> &str {
        &self.name
    }
}

//impl GetDomainId for DataStoreEntity {
//    fn my_domain_id(&self) -> i64 {
//        self.domain_id
//    }
//}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataQueryEntity {
    pub name: String, //TODO: make sure this is an alphanumeric
    //pub domain_id: i64,
    pub description: String,
    pub statement: String,
}

impl Named for DataQueryEntity {
    fn my_name(&self) -> &str {
        &self.name
    }
}

pub type ScriptParam = serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub text: String,
}

impl Named for Script {
    fn my_name(&self) -> &str {
        &self.name
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
    pub view_state: serde_json::Value,
}

impl Named for View {
    fn my_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub data: serde_json::Value,
    pub timestamp: chrono::NaiveDateTime,
    //TODO: maybe add the user as well
}
