
use plugins::v1::Domain;
use plugins::v1::Datastore;
use plugins::v1::DomainBuilder;
use plugins::v1::DataStoreEntity;
use plugins::v1::DatastoreError;
use plugins::v1::DataQuery;
use plugins::v1::DataQueryEntity;

use kakapo_redis::KakapoRedis;
use kakapo_redis::data::Keys;
use kakapo_redis::data::KeyValues;

use r2d2::Pool;
use r2d2::PooledConnection;

use r2d2_redis::r2d2;
use r2d2_redis::RedisConnectionManager;
use r2d2_redis::redis::Commands;
use r2d2_redis::redis::RedisError;

use linked_hash_map::LinkedHashMap;
use kakapo_redis::data::Table;


#[derive(Clone)]
pub struct KakapoRedisDone {
    pool: Pool<RedisConnectionManager>,
}


pub struct KakapoRedisConnection {
    conn: PooledConnection<RedisConnectionManager>
}


impl DomainBuilder for KakapoRedis {
    fn build(&self) -> Box<Domain> {
        info!("Initializing postgres connection");
        let database_url = format!(
            "redis://{}:{}",
            self.host,
            self.port,
        );
        let manager = RedisConnectionManager::new(&database_url[..])
            .expect("Could not form database url");
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        Box::new(KakapoRedisDone { pool })
    }
}

impl Domain for KakapoRedisDone {

    fn domain_type(&self) -> &'static str {
        "REDIS"
    }

    fn connect_datastore(&self) -> Option<Box<Datastore>> {
        debug!("connecting to the pool for datastore");
        let conn = self.pool.get()
            .expect("Could not get connection");

        let redis_connection = KakapoRedisConnection { conn };
        Some(Box::new(redis_connection))
    }

    fn connect_query(&self) -> Option<Box<DataQuery>> {
        None
    }
}

//Note that I'm doing redis tables as namespace
impl Datastore for KakapoRedisConnection {
    fn retrieve(&self, data_store: &DataStoreEntity, query: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;
        let table_name = table.get_name();

        let results: Vec<String> = self.conn.keys(&format!("{}:*", table_name))
            .map_err(|err| DatastoreError::DbError(err.to_string()))?;

        let res = serde_json::to_value(results)
            .map_err(|err| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn insert(&self, data_store: &DataStoreEntity, rows: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;
        let table_name = table.get_name();

        let data: KeyValues = serde_json::from_value(rows.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?;

        let mut results = KeyValues::new();
        for (key, value) in data {
            let key_val: Option<(String, String)> = self.conn.set(&format!("{}:{}", table_name, &key), &value) //TODO: maybe use multiset
                .map(|res: String| {
                    if res == "OK" {
                        Some((key, value))
                    } else {
                        warn!("Could not set value");
                        None
                    }
                })
                .unwrap_or_else(|err| {
                    warn!("encountered an error: {:?}", &err);
                    None
                });

            if let Some((ok_key, ok_val)) = key_val {
                results.insert(ok_key, ok_val);
            }
        }

        let res = serde_json::to_value(results)
            .map_err(|err| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn upsert(&self, data_store: &DataStoreEntity, rows: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        self.insert(data_store, rows) // Same as insert
    }

    fn update(&self, data_store: &DataStoreEntity, key_values: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        self.insert(data_store, key_values) // Same as insert
    }

    fn delete(&self, data_store: &DataStoreEntity, keys: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        unimplemented!()
    }

    fn on_datastore_created(&self, new: &DataStoreEntity) -> Result<(), DatastoreError> {
        unimplemented!()
    }

    fn on_datastore_updated(&self, old: &DataStoreEntity, new: &DataStoreEntity) -> Result<(), DatastoreError> {
        unimplemented!()
    }

    fn on_datastore_deleted(&self, old: &DataStoreEntity) -> Result<(), DatastoreError> {
        unimplemented!()
    }
}