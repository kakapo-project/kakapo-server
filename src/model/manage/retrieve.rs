

use actix::prelude::*;
use diesel;
use diesel::result::Error;
use diesel::{
    prelude::*,
    insert_into,
    delete,
    update,
};
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use chrono::NaiveDateTime;
use serde_json;

use std::error;
use std::collections::HashMap;
use std::io;

use failure::Fail;

use data;
use data::error::StateError;
use data::api;

use super::super::auth;
use super::super::schema::{entity, table_schema, table_schema_history, query, query_history, script, script_history};
use super::super::connection::executor::DatabaseExecutor;

use super::super::dbdata::*;

use super::utils::{get_modification_commits, unroll_modification_commits, update_migration, generate_error, get_single_table, unroll_table};


pub fn get_tables(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    detailed: bool,
    show_deleted: bool,
) -> Result<api::GetTablesResult, api::Error> {

    let result = conn.transaction::<Vec<data::DetailedTable>, diesel::result::Error, _>(|| {
        let table_schemas: Vec<TableSchema> = table_schema::table
            .load::<TableSchema>(conn)?;
        println!("table schemas: {:?}", table_schemas);

        let tables: Vec<data::DetailedTable> = table_schemas.iter()
            .map(|table_schema| {
                get_single_table(conn, &table_schema)
            })
            .collect::<Result<Vec<data::DetailedTable>, diesel::result::Error>>()?;

        Ok(tables)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|tables| {
            let get_table_result = match detailed {
                true => api::GetTablesResult::DetailedTables(tables),
                false => {
                    let unrolled_tables = tables.iter()
                        .map(|table| unroll_table(table.to_owned()))
                        .collect::<Result<Vec<data::Table>, StateError>>()
                        .or_else(|err| Err(api::Error::InvalidStateError))?;

                    api::GetTablesResult::Tables(unrolled_tables)
                }
            };
            Ok(get_table_result)
        })

}


pub fn get_queries(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    show_deleted: bool,
) -> Result<api::GetQueriesResult, api::Error> {

    let result = conn.transaction::<Vec<data::Query>, diesel::result::Error, _>(|| {
        let queries = query::table
            .load::<DataQuery>(conn)?;
        println!("table schemas: {:?}", queries);

        let parsed_queries = queries.iter()
            .map(|query| {
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
            })
            .collect::<Result<Vec<data::Query>, diesel::result::Error>>()?;

        println!("parsed_queries: {:?}", parsed_queries);

        Ok(parsed_queries)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|queries| Ok(api::GetQueriesResult(queries)))

}


pub fn get_scripts(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<api::GetScriptsResult, api::Error> {

    let result = conn.transaction::<Vec<data::Script>, diesel::result::Error, _>(|| {
        let scripts = script::table
            .load::<DataScript>(conn)?;
        println!("table schemas: {:?}", scripts);

        let parsed_scripts = scripts.iter()
            .map(|script| {
                let script_history: DataScriptHistory = script_history::table
                    .filter(script_history::script_id.eq(script.script_id))
                    .order_by(script_history::modified_at.desc())
                    .limit(1)
                    .get_result::<DataScriptHistory>(conn)?;

                let script_item = data::Script {
                    name: script.name.to_owned(),
                    description: script_history.description,
                    text: script_history.script_text,
                };

                Ok(script_item)
            })
            .collect::<Result<Vec<data::Script>, diesel::result::Error>>()?;

        println!("parsed_scripts: {:?}", parsed_scripts);

        Ok(parsed_scripts)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|scripts| Ok(api::GetScriptsResult(scripts)))

}

pub fn get_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    detailed: bool,
) -> Result<api::GetTableResult, api::Error> {

    let result = conn.transaction::<data::DetailedTable, diesel::result::Error, _>(|| {
        let table_schema: TableSchema = table_schema::table
            .filter(table_schema::name.eq(table_name))
            .get_result::<TableSchema>(conn)?;
        println!("table schema: {:?}", table_schema);


        let table: data::DetailedTable = get_single_table(conn, &table_schema)?;

        Ok(table)

    });

    result
        .or_else(|err| match err {
            diesel::result::Error::NotFound => Err(api::Error::TableNotFound),
            _ => Err(api::Error::DatabaseError(err)),
        })
        .and_then(|table| {
            let get_table_result = match detailed {
                true => api::GetTableResult::DetailedTable(table),
                false => {
                    let unrolled_table = unroll_table(table.to_owned())
                        .or_else(|_| Err(api::Error::InvalidStateError))?;

                    api::GetTableResult::Table(unrolled_table)
                }
            };
            Ok(get_table_result)
        })

}


pub fn get_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    query_name: String,
) -> Result<api::GetQueryResult, api::Error> {

    let result = conn.transaction::<data::Query, diesel::result::Error, _>(|| {
        let query = query::table
            .filter(query::name.eq(query_name))
            .get_result::<DataQuery>(conn)?;

        println!("query: {:?}", query);

        //TODO: remove duplication
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

        println!("parsed_queries: {:?}", query_item);

        Ok(query_item)

    });

    result
        .or_else(|err| match err {
            diesel::result::Error::NotFound => Err(api::Error::QueryNotFound),
            _ => Err(api::Error::DatabaseError(err)),
        })
        .and_then(|query| Ok(api::GetQueryResult(query)))

}





//TODO: crate public
pub fn get_one_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    script_name: String,
) -> Result<data::Script, diesel::result::Error> {

    let raw_script_data = script::table
        .filter(script::name.eq(script_name))
        .get_result::<DataScript>(conn)?;

    println!("script: {:?}", raw_script_data);

    //TODO: remove duplication
    let script_history: DataScriptHistory = script_history::table
        .filter(script_history::script_id.eq(raw_script_data.script_id))
        .order_by(script_history::modified_at.desc())
        .limit(1)
        .get_result::<DataScriptHistory>(conn)?;

    let script_item = data::Script {
        name: raw_script_data.name.to_owned(),
        description: script_history.description,
        text: script_history.script_text,
    };

    Ok(script_item)
}

pub fn get_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    script_name: String,
) -> Result<api::GetScriptResult, api::Error> {

    let result = conn.transaction::<data::Script, diesel::result::Error, _>(|| {

        let script_item = get_one_script(conn, script_name)?;
        println!("parsed_scripts: {:?}", script_item);

        Ok(script_item)
    });

    result
        .or_else(|err| match err {
            diesel::result::Error::NotFound => Err(api::Error::ScriptNotFound),
            _ => Err(api::Error::DatabaseError(err)),
        })
        .and_then(|script| Ok(api::GetScriptResult(script)))

}