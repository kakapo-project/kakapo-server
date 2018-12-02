

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

use super::super::api;
use super::super::data;
use super::super::err::StateError;

use auth;
use super::super::schema::{entity, table_schema, table_schema_history, query, query_history, script, script_history};
use super::super::connection::executor::DatabaseExecutor;

use super::super::dbdata::*;

use super::utils::{get_modification_commits, unroll_modification_commits, update_migration, generate_error, get_single_table, unroll_table};

pub fn create_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_table: api::PostTable
) -> Result<api::CreateTableResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let table_name = post_table.name;

        let table_schema: TableSchema = table_schema::table
            .filter(table_schema::name.eq(table_name.to_string()))
            .get_result::<TableSchema>(conn)
            .and_then(|result| {
                println!("table already loaded: {:?}", &result);
                Ok(result)
            })
            .or_else::<Error, _>(|error| {
                let entity = insert_into(entity::table)
                    .values(&NewEntity {
                        scope_id: 1,
                        created_by: user_id,
                    })
                    .get_result::<Entity>(conn)?;

                let result = insert_into(table_schema::table)
                    .values(&NewTableSchema {
                        entity_id: entity.entity_id,
                        name: table_name.to_string(),
                    })
                    .get_result::<TableSchema>(conn)?;

                println!("new table addedd: {:?}", &result);
                Ok(result)
            })?;

        let previous_modifications = get_modification_commits(conn, &table_schema)?;
        let (previouse_schema_state, previous_modification) = unroll_modification_commits(previous_modifications)
            .or_else(|err| {
                Err(Error::SerializationError(Box::new(err.compat())))
            })?;
        let new_modification: data::SchemaModification = post_table.action;

        let new_schema_state = update_migration(
            conn,
            table_name.to_string(),
            previouse_schema_state,
            previous_modification,
            new_modification.to_owned())?;

        let json_schema_action = serde_json::to_value(&new_modification)
            .or_else(|err| {
                Err(Error::SerializationError(Box::new(err)))
            })?;

        let table_schema_history = insert_into(table_schema_history::table)
            .values(&NewTableSchemaHistory {
                table_schema_id: table_schema.table_schema_id,
                description: post_table.description.to_string(),
                modification: json_schema_action,
                modified_by: user_id,
            })
            .get_result::<TableSchemaHistory>(conn)?;

        println!("table_schema:         {:?}", table_schema);
        println!("table_schema_history: {:?}", table_schema_history);

        let table = data::Table {
            name: table_name,
            description: post_table.description,
            schema: new_schema_state,
        };

        Ok(table)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|table| Ok(api::CreateTableResult(table)))
}


pub fn create_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_query: api::PostQuery
) -> Result<api::CreateQueryResult, api::Error> {


    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let query_name = post_query.name;

        let data_query: DataQuery = query::table
            .filter(query::name.eq(query_name.to_string()))
            .get_result::<DataQuery>(conn)
            .and_then(|result| {
                println!("query already loaded: {:?}", &result);
                Ok(result)
            })
            .or_else::<Error, _>(|error| {
                let entity = insert_into(entity::table)
                    .values(&NewEntity {
                        scope_id: 1,
                        created_by: user_id,
                    })
                    .get_result::<Entity>(conn)?;

                let result = insert_into(query::table)
                    .values(&NewDataQuery {
                        entity_id: entity.entity_id,
                        name: query_name.to_string(),
                    })
                    .get_result::<DataQuery>(conn)?;

                println!("new table addedd: {:?}", &result);
                Ok(result)
            })?;

        let query_history = insert_into(query_history::table)
            .values(&NewDataQueryHistory {
                query_id: data_query.query_id,
                description: post_query.description.to_string(),
                statement: post_query.statement.to_string(),
                query_info: serde_json::to_value(&json!({})).unwrap(), //TODO: what should go here?
                modified_by: user_id,
            })
            .get_result::<DataQueryHistory>(conn)?;

        println!("query:         {:?}", data_query);
        println!("query_history: {:?}", query_history);

        let query = data::Query {
            name: data_query.name,
            description: query_history.description,
            statement: query_history.statement,
        };

        Ok(query)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|query| Ok(api::CreateQueryResult(query)))
}


pub fn create_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_script: api::PostScript
) -> Result<api::CreateScriptResult, api::Error> {


    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let script_name = post_script.name;

        let (data_script, exists) = script::table
            .filter(script::name.eq(script_name.to_string()))
            .get_result::<DataScript>(conn)
            .and_then(|result| {
                println!("script already loaded: {:?}", &result);
                Ok((result, true))
            })
            .or_else::<Error, _>(|error| {
                if creation_method == data::CreationMethod::FailIfNotExists {
                    return Err(error);
                }
                let entity = insert_into(entity::table)
                    .values(&NewEntity {
                        scope_id: 1,
                        created_by: user_id,
                    })
                    .get_result::<Entity>(conn)?;

                let result = insert_into(script::table)
                    .values(&NewDataScript {
                        entity_id: entity.entity_id,
                        name: script_name.to_string(),
                    })
                    .get_result::<DataScript>(conn)?;

                println!("new table addedd: {:?}", &result);
                Ok((result, false))
            })?;

        if exists && creation_method == data::CreationMethod::FailIfExists {
            return Err(generate_error("script already loaded"));
        }

        let script_history = if exists && creation_method == data::CreationMethod::IgnoreIfExists {
            script_history::table
                .filter(script_history::script_id.eq(data_script.script_id))
                .order_by(script_history::modified_at.desc())
                .get_result::<DataScriptHistory>(conn)?
        } else {
            insert_into(script_history::table)
                .values(&NewDataScriptHistory {
                    script_id: data_script.script_id,
                    description: post_script.description.to_string(),
                    script_language: "Python3".to_string(), //only python3 currently supported
                    script_text: post_script.text.to_string(),
                    script_info: serde_json::to_value(&json!({})).unwrap(), //TODO: what should go here?
                    modified_by: user_id,
                })
                .get_result::<DataScriptHistory>(conn)?
        };

        println!("script:         {:?}", data_script);
        println!("script_history: {:?}", script_history);

        let script = data::Script {
            name: data_script.name,
            description: script_history.description,
            text: script_history.script_text,
        };

        Ok(script)
    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|script| Ok(api::CreateScriptResult(script)))
}

pub fn update_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_table: api::PutTable
) -> Result<api::CreateTableResult, api::Error> {

    let post_table = put_table.with_name(name);

    create_table(conn, data::CreationMethod::FailIfNotExists, post_table)
}


pub fn update_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_query: api::PutQuery
) -> Result<api::CreateQueryResult, api::Error> {

    let post_query = put_query.with_name(name);

    create_query(conn, data::CreationMethod::FailIfNotExists, post_query)
}

pub fn update_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_script: api::PutScript
) -> Result<api::CreateScriptResult, api::Error> {

    let post_script = put_script.with_name(name);

    create_script(conn, data::CreationMethod::FailIfNotExists, post_script)
}


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