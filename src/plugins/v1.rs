
use serde::Serialize;
use serde::de::DeserializeOwned;

pub use data::DataStoreEntity;
pub use data::DataQueryEntity;
pub use data::error::DatastoreError;

pub trait DomainBuilder
    where
        Self: Send + Sync,
{
    fn build(&self) -> Box<Domain>;
}

pub trait Domain
    where
        Self: Send + Sync,
{
    fn domain_type(&self) -> &'static str;
    fn connect_datastore(&self) -> Option<Box<Datastore>>;
    fn connect_query(&self) -> Option<Box<DataQuery>>;
}

type Rows = serde_json::Value;
type Keys = serde_json::Value;
type KeyValues = serde_json::Value;
type Dataset = serde_json::Value;

pub trait Datastore:
    where
        Self: Send
{
    //Warning: this api might be modified to allow for more generic operations

    // NOTE: the goal is to have associated types here, but rust doesn't really work with passing
    // boxes of traits with associated types, and putting this in a dynamic library is going to be
    // pretty much impossible without reflections. Which rust doesn't have obviously.
    /*
    type Keys;
    type Rows;
    type Dataset;

    fn insert(&self, rows: Self::Rows) -> Self::Dataset;
    fn upsert(&self, rows: Self::Rows) -> Self::Dataset;
    fn update(&self, keys: Self::Keys, rows: Self::Rows) -> Self::Dataset;
    fn delete(&self, keys: Self::Keys) -> Self::Dataset;
    fn retrieve(&self) -> Self::Dataset;
    */

    fn retrieve(&self, data_store: &DataStoreEntity) -> Result<Dataset, DatastoreError>;
    fn insert(&self, data_store: &DataStoreEntity, rows: &Rows) -> Result<Dataset, DatastoreError>;
    fn upsert(&self, data_store: &DataStoreEntity, rows: &Rows) -> Result<Dataset, DatastoreError>;
    fn update(&self, data_store: &DataStoreEntity, key_values: &KeyValues) -> Result<Dataset, DatastoreError>;
    fn delete(&self, data_store: &DataStoreEntity, keys: &Keys) -> Result<Dataset, DatastoreError>;

    fn on_datastore_created(&self, new: &DataStoreEntity) -> Result<(), DatastoreError>;
    fn on_datastore_updated(&self, old: &DataStoreEntity, new: &DataStoreEntity) -> Result<(), DatastoreError>;
    fn on_datastore_deleted(&self, old: &DataStoreEntity) -> Result<(), DatastoreError>;
}

type QueryParams = serde_json::Value;
type QueryFormat = serde_json::Value;

pub trait DataQuery
    where
        Self: Send
{
    fn query(&self, query: &DataQueryEntity, query_params: &QueryParams, format: &QueryFormat) -> Result<Dataset, DatastoreError>; //TODO: rename to DatasetError
}

