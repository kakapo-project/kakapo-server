
use kakapo_postgres::data::Table;
use kakapo_postgres::data::RawTableData;
use kakapo_postgres::data::ObjectValues;
use kakapo_postgres::data::ObjectKeys;
use kakapo_postgres::data::Value;
use kakapo_postgres::database::error::DbError;
use kakapo_postgres::database::DatabaseFunctions;

use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;
use data::Named;
use plugins::v1::DatastoreError;
use kakapo_postgres::data::Query;
use kakapo_postgres::data::QueryParams;

pub struct QueryTable<'a> {
    conn: &'a PooledConnection<ConnectionManager<PgConnection>>,
}

impl<'a> QueryTable<'a> {
    pub fn new(conn: &'a PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { conn }
    }
}


pub trait QueryTableOps {
    fn run_query(&self, query: &Query, params: QueryParams) -> Result<RawTableData, DatastoreError>;
}


impl<'a> QueryTableOps for QueryTable<'a> {
    fn run_query(&self, query: &Query, params: QueryParams) -> Result<RawTableData, DatastoreError> {


        let db_params = params.value_list();

        /* TODO: ...
        let username = state.get_authorization().username();
        if let Some(db_user) = username.to_owned() {
            state
                .get_database()
                .exec("SET SESSION AUTHORIZATION $1", vec![Value::String(db_user)])
                .or_else(|err| Err(DatastoreError::DbError(err.to_string())))?;

        }
        */

        let result = self
            .conn
            .exec(&query.statement, db_params)
            .or_else(|err| Err(DatastoreError::DbError(err.to_string())))?;

        /*
        if let Some(db_user) = username {
            state
                .get_database()
                .exec("RESET SESSION AUTHORIZATION", vec![])
                .or_else(|err| Err(DatastoreError::DbError(err.to_string())))?;
        }
        */

        Ok(result)
    }
}