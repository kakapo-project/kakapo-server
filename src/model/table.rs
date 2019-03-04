
use data;
use data::Named;
use data::error::DatastoreError;

use connection::executor::DomainError;

use plugins::v1::Datastore;


pub struct DatastoreAction<'a> {
    pub conn: &'a Result<Box<Datastore>, DomainError>,
}

pub trait DatastoreActionOps {
    fn query(&self, table: &data::DataStoreEntity) -> Result<serde_json::Value, DatastoreError>;

    fn insert_row(&self, table: &data::DataStoreEntity, data: &serde_json::Value, fail_on_duplicate: bool) -> Result<serde_json::Value, DatastoreError>;

    fn upsert_row(&self, table: &data::DataStoreEntity, data: &serde_json::Value) -> Result<serde_json::Value, DatastoreError>;

    fn update_row(&self, table: &data::DataStoreEntity, keyed_data: &serde_json::Value, fail_on_not_found: bool) -> Result<serde_json::Value, DatastoreError>;

    fn delete_row(&self, table: &data::DataStoreEntity, keys: &serde_json::Value, fail_on_not_found: bool) -> Result<serde_json::Value, DatastoreError>;
}

impl From<&DomainError> for DatastoreError {
    fn from(domain: &DomainError) -> Self {
        match domain {
            DomainError::DomainNotFound(x) => DatastoreError::DomainNotFound(x.to_owned()),
            DomainError::Unknown => DatastoreError::Unknown,
            DomainError::DatastoreNotAvailable => DatastoreError::NotSupported,
            DomainError::QueryNotAvailable => DatastoreError::NotSupported,
        }
    }
}

impl<'a> DatastoreActionOps for DatastoreAction<'a> {
    fn query(&self, table: &data::DataStoreEntity) -> Result<serde_json::Value, DatastoreError> {
        match self.conn {
            Ok(conn) => conn.retrieve(table), //TODO: should be able to query
            Err(err) => Err(err.into())
        }
    }

    fn insert_row(&self, table: &data::DataStoreEntity, data: &serde_json::Value, fail_on_duplicate: bool) -> Result<serde_json::Value, DatastoreError> {
        match self.conn {
            Ok(conn) => conn.insert(table, data),
            Err(err) => Err(err.into())
        }
    }

    fn upsert_row(&self, table: &data::DataStoreEntity, data: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        match self.conn {
            Ok(conn) => conn.upsert(table, data),
            Err(err) => Err(err.into())
        }
    }

    fn update_row(&self, table: &data::DataStoreEntity, keyed_data: &serde_json::Value, fail_on_not_found: bool) -> Result<serde_json::Value, DatastoreError> {
        match self.conn {
            Ok(conn) => conn.update(table, keyed_data),
            Err(err) => Err(err.into())
        }
    }

    fn delete_row(&self, table: &data::DataStoreEntity, keys: &serde_json::Value, fail_on_not_found: bool) -> Result<serde_json::Value, DatastoreError> {
        match self.conn {
            Ok(conn) => conn.delete(table, keys),
            Err(err) => Err(err.into())
        }

    }
}