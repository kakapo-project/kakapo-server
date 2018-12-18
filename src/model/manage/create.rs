
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
use data::api;
use data::error::StateError;

use super::super::auth;
use super::super::schema::{entity, table_schema, table_schema_history, query, query_history, script, script_history};
use connection::executor::DatabaseExecutor;

use super::super::dbdata::*;

use super::utils::{get_modification_commits, unroll_modification_commits, update_migration, generate_error};

pub fn create_table_internal(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_table: api::NewTable
) -> Result<api::CreateTableResult, api::Error> {

    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let table_name = post_table.name;

        let (table_schema, exists) = table_schema::table
            .filter(table_schema::name.eq(table_name.to_string()))
            .get_result::<TableSchema>(conn)
            .and_then(|result| {
                println!("table already loaded: {:?}", &result);
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

                let result = insert_into(table_schema::table)
                    .values(&NewTableSchema {
                        entity_id: entity.entity_id,
                        name: table_name.to_string(),
                    })
                    .get_result::<TableSchema>(conn)?;

                println!("new table addedd: {:?}", &result);
                Ok((result, false))
            })?;

        if exists && creation_method == data::CreationMethod::FailIfExists {
            return Err(generate_error("script already loaded"));
        }

        let previous_modifications = get_modification_commits(conn, &table_schema)?;
        let (previouse_schema_state, previous_modification) = unroll_modification_commits(previous_modifications)
            .or_else(|err| {
                Err(Error::SerializationError(Box::new(err.compat())))
            })?;

        let new_schema_state = if exists && creation_method == data::CreationMethod::IgnoreIfExists {
            previouse_schema_state
        } else {
            let new_modification: data::SchemaModification = post_table.action;

            let updated_schema_state = update_migration(
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

            println!("table_schema_history: {:?}", table_schema_history);
            updated_schema_state
        };

        println!("table_schema:         {:?}", table_schema);
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


pub fn create_query_internal(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_query: api::NewQuery
) -> Result<api::CreateQueryResult, api::Error> {


    let result = conn.transaction::<_, diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let query_name = post_query.name;

        let (data_query, exists) = query::table
            .filter(query::name.eq(query_name.to_string()))
            .get_result::<DataQuery>(conn)
            .and_then(|result| {
                println!("query already loaded: {:?}", &result);
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

                let result = insert_into(query::table)
                    .values(&NewDataQuery {
                        entity_id: entity.entity_id,
                        name: query_name.to_string(),
                    })
                    .get_result::<DataQuery>(conn)?;

                println!("new table addedd: {:?}", &result);
                Ok((result, false))
            })?;

        if exists && creation_method == data::CreationMethod::FailIfExists {
            return Err(generate_error("script already loaded"));
        }

        let query_history = if exists && creation_method == data::CreationMethod::IgnoreIfExists {
            query_history::table
                .filter(query_history::query_id.eq(data_query.query_id))
                .order_by(query_history::modified_at.desc())
                .get_result::<DataQueryHistory>(conn)?
        } else {
            insert_into(query_history::table)
                .values(&NewDataQueryHistory {
                    query_id: data_query.query_id,
                    description: post_query.description.to_string(),
                    statement: post_query.statement.to_string(),
                    query_info: serde_json::to_value(&json!({})).unwrap(), //TODO: what should go here?
                    is_deleted: post_query.for_deletion,
                    modified_by: user_id,
                })
                .get_result::<DataQueryHistory>(conn)?
        };

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


fn create_script_internal(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    creation_method: api::CreationMethod,
    post_script: api::NewScript
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
                    is_deleted: post_script.for_deletion,
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

pub fn create_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    on_duplicate: api::OnDuplicate,
    post_table: api::PostTable
) -> Result<api::CreateTableResult, api::Error> {

    create_table_internal(conn, on_duplicate.into_method(), post_table.into_new())
}


pub fn create_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    on_duplicate: api::OnDuplicate,
    post_query: api::PostQuery
) -> Result<api::CreateQueryResult, api::Error> {

    create_query_internal(conn, on_duplicate.into_method(), post_query.into_new())
}


pub fn create_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    on_duplicate: api::OnDuplicate,
    post_script: api::PostScript
) -> Result<api::CreateScriptResult, api::Error> {

    create_script_internal(conn, on_duplicate.into_method(), post_script.into_new())
}

pub fn update_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_table: api::PutTable
) -> Result<api::CreateTableResult, api::Error> {

    create_table_internal(conn, data::CreationMethod::FailIfNotExists, put_table.with_name(name))
}


pub fn update_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_query: api::PutQuery
) -> Result<api::CreateQueryResult, api::Error> {

    create_query_internal(conn, data::CreationMethod::FailIfNotExists, put_query.into_new(name))
}

pub fn update_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
    put_script: api::PutScript
) -> Result<api::CreateScriptResult, api::Error> {

    create_script_internal(conn, data::CreationMethod::FailIfNotExists, put_script.into_new(name))
}

pub fn delete_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
) -> Result<(), api::Error> {

    create_table_internal(conn, data::CreationMethod::FailIfNotExists, api::NewTable::deletion(name))?;

    Ok(())
}


pub fn delete_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
) -> Result<(), api::Error> {

    create_query_internal(conn, data::CreationMethod::FailIfNotExists, api::NewQuery::deletion(name))?;

    Ok(())
}

pub fn delete_script(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    name: String,
) -> Result<(), api::Error> {

    create_script_internal(conn, data::CreationMethod::FailIfNotExists, api::NewScript::deletion(name))?;

    Ok(())
}
