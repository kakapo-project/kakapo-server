
use diesel;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use data;
use data::api;

use super::dbdata::*;
use super::schema::{query, query_history};
use super::database;

fn get_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    query_name: String
) -> Result<data::Query, diesel::result::Error> {

    let query = query::table
        .filter(query::name.eq(query_name))
        .get_result::<DataQuery>(conn)?;
    println!("table schemas: {:?}", query);

    let query_history: DataQueryHistory = query_history::table
        .filter(query_history::query_id.eq(query.query_id))
        .order_by(query_history::modified_at.desc())
        .limit(1)
        .get_result::<DataQueryHistory>(conn)?;

    let query_item = data::Query {
        name: query.name.to_owned(),
        description: query_history.description,
        statement: query_history.statement,
    };

    Ok(query_item)
}

pub fn run_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    query_name: String,
    format: api::TableDataFormat,
    params: api::QueryParams,
    //TODO: Add output format: indexed, rows (default), columns, schema
) -> Result<api::RunQueryResult, api::Error> {
    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let query = get_query(conn, query_name)?;
        let data = database::execute_query(conn, &query)?;
        let formatted_data = match format {
            data::TableDataFormat::Rows => data.into_rows_data(),
            data::TableDataFormat::FlatRows => data.into_rows_flat_data(),
        };

        let query_with_data = data::QueryWithData {
            query: query,
            data: formatted_data,
        };

        Ok(query_with_data)

    }).or_else(|err| match err {
        diesel::result::Error::NotFound => Err(api::Error::QueryNotFound),
        _ => Err(api::Error::DatabaseError(err)),
    })?;

    println!("final result: {:?}", result);

    Ok(api::RunQueryResult(result.data))
}