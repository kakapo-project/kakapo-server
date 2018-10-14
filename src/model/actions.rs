

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

use super::api;
use super::data;
use super::err::StateError;

use auth;
use super::schema::{entity, table_schema, table_schema_history};
use super::connection::DatabaseExecutor;



#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccount {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewEntity {
    pub scope_id: i64,
    pub created_by: i64,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema"]
pub struct NewTableSchema {
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "table_schema_history"]
pub struct NewTableSchemaHistory {
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_by: i64,
}

#[derive(Debug, Queryable)]
pub struct Entity {
    pub entity_id: i64,
    pub scope_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by: i64,
}

#[derive(Debug, Queryable)]
pub struct TableSchema {
    pub table_schema_id: i64,
    pub entity_id: i64,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct TableSchemaHistory {
    pub table_schema_history_id: i64,
    pub table_schema_id: i64,
    pub description: String,
    pub modification: serde_json::Value,
    pub modified_at: NaiveDateTime,
    pub modified_by: i64,
}

fn get_table_history_items(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<Vec<TableSchemaHistory>, diesel::result::Error> {

    let table_history_items: Vec<TableSchemaHistory> = table_schema_history::table
        .filter(table_schema_history::table_schema_id.eq(table_schema.table_schema_id))
        .order_by(table_schema_history::modified_at.asc())
        .load::<TableSchemaHistory>(conn)?;

    Ok(table_history_items)
}

fn parse_table_history_items_to_modification_commits(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_history_items: &Vec<TableSchemaHistory>
) -> Result<Vec<data::SchemaModificationCommit>, diesel::result::Error> {


    let modifications: Vec<data::SchemaModificationCommit> = table_history_items.iter()
        .map(|table_history_item| {
            let schema_modification = data::SchemaModificationCommit {
                date: table_history_item.modified_at,
                action: serde_json::from_value::<data::SchemaModification>(table_history_item.modification.to_owned())?,
            };
            Ok(schema_modification)
        })
        .collect::<Result<Vec<data::SchemaModificationCommit>, serde_json::Error>>()
        .or_else(|err| Err(diesel::result::Error::SerializationError(Box::new(err))))?;

    Ok(modifications)
}


fn get_modification_commits(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<Vec<data::SchemaModificationCommit>, diesel::result::Error> {

    let table_history_items = get_table_history_items(&conn, &table_schema)?;
    let modification_commits = parse_table_history_items_to_modification_commits(&conn, &table_history_items)?;

    Ok(modification_commits)
}

fn get_single_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<data::DetailedTable, diesel::result::Error> {

    let table_history_items = get_table_history_items(&conn, &table_schema)?;
    let modification_commits = parse_table_history_items_to_modification_commits(&conn, &table_history_items)?;

    let last_table_history_item = table_history_items.last()
        .ok_or(diesel::result::Error::NotFound)?;

    let description = &last_table_history_item.description;

    let detailed_table = data::DetailedTable {
        name: table_schema.name.to_string(),
        description: description.to_string(),
        schema: modification_commits,
    };

    Ok(detailed_table)
}

fn unroll_modifications(modifications: Vec<data::SchemaModification>) -> Result<data::SchemaState, StateError> {
    let initial_state = data::SchemaState {
        columns: vec![],
        constraint: vec![],
    };
    let empty_state_stack: Vec<data::SchemaState> = vec![];

    let result = modifications.iter()
        .fold(Ok(empty_state_stack.to_owned()), |state_result, modification| {
            state_result.and_then(|state|
                match modification {
                    data::SchemaModification::Create { columns, constraint, } => {
                        let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
                        let mut state_clone = state.to_owned();

                        //TODO: check if it exists

                        last_state.columns.extend(columns.to_owned());
                        last_state.constraint.extend(constraint.to_owned());

                        state_clone.push(last_state);
                        Ok(state_clone)
                    },
                    data::SchemaModification::Remove { column, constraint, } => {
                        let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
                        let mut state_clone = state.to_owned();

                        last_state.columns.retain(|x| column.contains(&x.name));
                        last_state.constraint.retain(|x| constraint.contains(&x));

                        state_clone.push(last_state);
                        Ok(state_clone)
                    },
                    data::SchemaModification::Rename { mapping, } => {
                        let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
                        let mut state_clone = state.to_owned();

                        last_state.columns = last_state.columns.iter()
                             .map(|column| {
                                 mapping.get(&column.name)
                                     .and_then(|new_name| {
                                         let mut new_column = column.to_owned();
                                         new_column.name = new_name.to_owned();
                                         Some(new_column)
                                     })
                                     .unwrap_or(column.to_owned())
                             })
                             .collect();

                        state_clone.push(last_state);
                        Ok(state_clone)
                    },
                    data::SchemaModification::Raw { up, down, } => {
                        let last_state = state.last().unwrap_or(&initial_state).to_owned();
                        let mut state_clone = state.to_owned();

                        // Just append the same state, so that a pop will push it out
                        state_clone.push(last_state);
                        Ok(state_clone)
                    },
                    data::SchemaModification::Nop => Ok(state),
                    data::SchemaModification::Revert => {
                        let split = state.split_last();
                        match split {
                            None => Err(StateError::RevertError),// Can't revert if the state is empty
                            Some((last, init)) => Ok(init.to_vec()),
                        }
                    },
                    data::SchemaModification::Delete => Ok(empty_state_stack.to_owned()),
                }
            )
        })?;

    Ok(result.last()
        .unwrap_or(&initial_state)
        .to_owned())
}

fn unroll_modification_commits(modification_commits: Vec<data::SchemaModificationCommit>) -> Result<data::SchemaState, StateError> {
    let modifications: Vec<data::SchemaModification> = modification_commits.iter()
        .map(|modification_commit| modification_commit.action.to_owned())
        .collect::<Vec<data::SchemaModification>>();

    unroll_modifications(modifications)
}

pub fn create_table(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    post_table: api::PostTable
) -> Result<(), api::Error> {

    let result = conn.transaction::<(), diesel::result::Error, _>(|| {

        let user_id = auth::get_current_user();
        let table_name = post_table.name;

        let table_schema: TableSchema = table_schema::table
            .filter(table_schema::name.eq(table_name.to_string()))
            .get_result::<TableSchema>(&conn)
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
                    .get_result::<Entity>(&conn)?;

                let result = insert_into(table_schema::table)
                    .values(&NewTableSchema {
                        entity_id: entity.entity_id,
                        name: table_name.to_string(),
                    })
                    .get_result::<TableSchema>(&conn)?;

                println!("new table addedd: {:?}", &result);
                Ok(result)
            })?;

        let previous_modifications: Vec<data::SchemaModificationCommit> = get_modification_commits(&conn, &table_schema)?;
        let schema_state = unroll_modification_commits(previous_modifications);
        let schema_action: data::SchemaModification = post_table.action;
        //let ddl_query = sqlgen.migrate_up(modification, is_new)?;

        let json_schema_action = serde_json::to_value(&schema_action)
            .or_else(|err| {
                Err(Error::SerializationError(Box::new(err)))
            })?;

        let table_schema_history = insert_into(table_schema_history::table)
            .values(&NewTableSchemaHistory {
                table_schema_id: table_schema.table_schema_id,
                description: post_table.description,
                modification: json_schema_action,
                modified_by: user_id,
            })
            .get_result::<TableSchemaHistory>(&conn)?;

        println!("table_schema:         {:?}", table_schema);
        println!("table_schema_history: {:?}", table_schema_history);

        Ok(())
    });

    result.or_else(|err| Err(api::Error::DatabaseError(err)))
}




pub fn get_tables(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    detailed: bool,
    show_deleted: bool,
) -> Result<api::GetTablesResult, api::Error> {

    let result = conn.transaction::<api::GetTablesResult, diesel::result::Error, _>(|| {
        let table_schemas: Vec<TableSchema> = table_schema::table
            .load::<TableSchema>(&conn)?;
        println!("table schemas: {:?}", table_schemas);

        let tables: Vec<data::DetailedTable> = table_schemas.iter()
            .map(|table_schema| {
                get_single_table(&conn, &table_schema)
            })
            .collect::<Result<Vec<data::DetailedTable>, diesel::result::Error>>()?;

        Ok(api::GetTablesResult::DetailedTables(tables))
    });

    result.or_else(|err| Err(api::Error::DatabaseError(err)))
}

pub fn get_table(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    detailed: bool,
) -> Result<api::GetTableResult, api::Error> {

    let result = conn.transaction::<data::DetailedTable, diesel::result::Error, _>(|| {
        let table_schema: TableSchema = table_schema::table
            .filter(table_schema::name.eq(table_name))
            .get_result::<TableSchema>(&conn)?;
        println!("table schema: {:?}", table_schema);


        let table: data::DetailedTable = get_single_table(&conn, &table_schema)?;

        Ok(table)

    });

    result
        .or_else(|err| Err(api::Error::DatabaseError(err)))
        .and_then(|table| {
            let get_table_result = match detailed {
                true => api::GetTableResult::DetailedTable(table),
                false => {
                    let modifications = table.schema.iter()
                        .map(|x| x.action.to_owned())
                        .collect::<Vec<data::SchemaModification>>();
                    let schema_state = unroll_modifications(modifications)
                        .or_else(|err| Err(api::Error::InvalidStateError))?;
                    let unrolled_table = data::Table {
                        name: table.name,
                        description: table.description,
                        schema: schema_state,
                    };

                    api::GetTableResult::Table(unrolled_table)
                }
            };
            Ok(get_table_result)
        })

}