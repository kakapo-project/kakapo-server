use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::Pool;
use diesel::prelude::PgConnection;

use plugins::v1::Domain;
use plugins::v1::Datastore;
use plugins::v1::DomainBuilder;
use plugins::v1::DataStoreEntity;
use plugins::v1::DatastoreError;
use plugins::v1::DataQuery;
use plugins::v1::DataQueryEntity;

use kakapo_postgres::data::Table;
use kakapo_postgres::data::TableData;
use kakapo_postgres::data::KeyedTableData;
use kakapo_postgres::data::KeyData;
use kakapo_postgres::KakapoPostgres;
use kakapo_postgres::update_state::UpdateTable;
use kakapo_postgres::update_state::UpdateTableOps;
use kakapo_postgres::table::CrudTable;
use kakapo_postgres::table::CrudTableOps;
use kakapo_postgres::data::Query;
use kakapo_postgres::query::QueryTable;
use kakapo_postgres::query::QueryTableOps;
use kakapo_postgres::data::QueryParams;

#[derive(Clone)]
pub struct KakapoPostgresDone {
    pool: Pool<ConnectionManager<PgConnection>>,
}


pub struct KakapoPostgresConnection {
    conn: PooledConnection<ConnectionManager<PgConnection>>
}


impl DomainBuilder for KakapoPostgres {
    fn build(&self) -> Box<Domain> {
        info!("Initializing postgres connection");
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.pass,
            self.host,
            self.port,
            self.db,
        );
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        Box::new(KakapoPostgresDone { pool })
    }
}

impl Domain for KakapoPostgresDone {

    fn domain_type(&self) -> &'static str {
        "POSTGRES"
    }

    fn connect_datastore(&self) -> Option<Box<Datastore>> {
        info!("connecting to the poo");
        let conn = self.pool.get()
            .expect("Could not get connection");

        let postgres_connection = KakapoPostgresConnection { conn };
        Some(Box::new(postgres_connection))
    }

    fn connect_query(&self) -> Option<Box<DataQuery>> {
        info!("connecting to the poo");
        let conn = self.pool.get()
            .expect("Could not get connection");

        let postgres_connection = KakapoPostgresConnection { conn };
        Some(Box::new(postgres_connection))
    }
}

// All of this is just boilerplate -__-
impl Datastore for KakapoPostgresConnection {
    fn retrieve(&self, data_store: &DataStoreEntity) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;

        let action = CrudTable::new(
            &table,
            &self.conn,
        );

        let res = action.retrieve()?;
        let res = serde_json::to_value(res)
            .map_err(|err| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn insert(&self, data_store: &DataStoreEntity, rows: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;

        let data: TableData = serde_json::from_value(rows.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?; //TODO: the serialization should have more informative error messages
        let data = data.normalize();

        let action = CrudTable::new(
            &table,
            &self.conn,
        );

        let res = action.insert(data, true)?; //TODO: fail on duplicate?
        let res = serde_json::to_value(res)
            .map_err(|_| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn upsert(&self, data_store: &DataStoreEntity, rows: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;

        let data: TableData = serde_json::from_value(rows.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?;
        let data = data.normalize();

        let action = CrudTable::new(
            &table,
            &self.conn,
        );

        let res = action.upsert(data)?;
        let res = serde_json::to_value(res)
            .map_err(|_| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn update(&self, data_store: &DataStoreEntity, key_values: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;

        let keyed_data: KeyedTableData = serde_json::from_value(key_values.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?;
        let (keys, data) = keyed_data.normalize();

        let action = CrudTable::new(
            &table,
            &self.conn,
        );

        let res = action.update(keys, data, true)?; //TODO: fail on duplicate?
        let res = serde_json::to_value(res)
            .map_err(|_| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn delete(&self, data_store: &DataStoreEntity, keys: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let table: Result<Table, DatastoreError> = data_store.into();
        let table = table?;

        let keys: KeyData = serde_json::from_value(keys.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?;
        let keys = keys.normalize();

        let action = CrudTable::new(
            &table,
            &self.conn,
        );

        let res = action.delete(keys, true)?; //TODO: fail on duplicate?
        let res = serde_json::to_value(res)
            .map_err(|_| DatastoreError::SerializationError)?;

        Ok(res)
    }

    fn on_datastore_created(&self, new: &DataStoreEntity) -> Result<(), DatastoreError> {
        let new: Result<Table, DatastoreError> = new.into();
        let new = new?;

        let action = UpdateTable::new(&self.conn);
        action.create_table(&new)
    }

    fn on_datastore_updated(&self, old: &DataStoreEntity, new: &DataStoreEntity) -> Result<(), DatastoreError> {
        let new: Result<Table, DatastoreError> = new.into();
        let new = new?;

        let old: Result<Table, DatastoreError> = old.into();
        let old = old?;

        let action = UpdateTable::new(&self.conn);
        action.update_table(&old, &new)
    }

    fn on_datastore_deleted(&self, old: &DataStoreEntity) -> Result<(), DatastoreError> {
        let old: Result<Table, DatastoreError> = old.into();
        let old = old?;

        let action = UpdateTable::new(&self.conn);
        action.delete_table(&old)
    }
}

impl DataQuery for KakapoPostgresConnection {
    fn query(&self, query: &DataQueryEntity, query_params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, DatastoreError> {
        let query: Result<Query, DatastoreError> = query.into();
        let query = query?;

        let query_params: QueryParams = serde_json::from_value(query_params.to_owned())
            .map_err(|_| DatastoreError::SerializationError)?;

        let action = QueryTable::new(&self.conn);
        let res = action.run_query(&query, query_params)?; //TODO: format

        let res = serde_json::to_value(res)
            .map_err(|_| DatastoreError::SerializationError)?;

        Ok(res)
    }
}