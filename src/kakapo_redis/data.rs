use linked_hash_map::LinkedHashMap;
use plugins::v1::DataStoreEntity;
use plugins::v1::DatastoreError;

pub type KeyValues = LinkedHashMap<String, String>;

pub type Keys = Vec<String>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub name: String, //TODO: make sure this is an alphanumeric
    pub description: String,
}

impl Table {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}


impl From<&DataStoreEntity> for Result<Table, DatastoreError> {
    fn from(item: &DataStoreEntity) -> Result<Table, DatastoreError> {
        Ok(Table {
            name: item.name.to_owned(),
            description: item.description.to_owned(),
        })
    }
}