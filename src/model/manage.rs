

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

use failure::Fail;

use super::api;
use super::data;
use super::err::StateError;

use auth;
use super::schema::{entity, table_schema, table_schema_history, query, query_history};
use super::connection::DatabaseExecutor;

use super::dbdata::*;


fn parse_table_history_items_to_modification_commits(
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

fn unroll_one_modification(
    modification: &data::SchemaModification,
    state: Vec<data::SchemaState>,
    backwards_state: Vec<data::SchemaModification>,
) -> Result<(Vec<data::SchemaState>, Vec<data::SchemaModification>), StateError> {

    let initial_state = data::SchemaState {
        columns: vec![],
        constraint: vec![],
    };
    let empty_state_stack: Vec<data::SchemaState> = vec![];
    let empty_backwards_state_stack: Vec<data::SchemaModification> = vec![];

    match modification {
        data::SchemaModification::Create { columns, constraint, } => {
            let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
            let mut state_clone = state.to_owned();
            let mut backwards_state_clone = backwards_state.to_owned();

            //TODO: check if it exists

            last_state.columns.extend(columns.to_owned());
            last_state.constraint.extend(constraint.to_owned());

            state_clone.push(last_state);
            backwards_state_clone.push(modification.to_owned());
            Ok((state_clone, backwards_state_clone))
        },
        data::SchemaModification::Remove { column, constraint, } => {
            let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
            let mut state_clone = state.to_owned();
            let mut backwards_state_clone = backwards_state.to_owned();

            last_state.columns.retain(|x| column.contains(&x.name));
            // last_state.constraint.retain(|x| constraint.contains(&x)); //TODO: constraint identifier??

            state_clone.push(last_state);
            backwards_state_clone.push(modification.to_owned());
            Ok((state_clone, backwards_state_clone))
        },
        data::SchemaModification::Rename { mapping, } => {
            let mut last_state = state.last().unwrap_or(&initial_state).to_owned();
            let mut state_clone = state.to_owned();
            let mut backwards_state_clone = backwards_state.to_owned();

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
            backwards_state_clone.push(modification.to_owned());
            Ok((state_clone, backwards_state_clone))
        },
        data::SchemaModification::Raw { up, down, } => {
            let last_state = state.last().unwrap_or(&initial_state).to_owned();
            let mut state_clone = state.to_owned();
            let mut backwards_state_clone = backwards_state.to_owned();

            // Just append the same state, so that a pop will push it out
            state_clone.push(last_state);
            backwards_state_clone.push(modification.to_owned());
            Ok((state_clone, backwards_state_clone))
        },
        data::SchemaModification::Nop => Ok((state, backwards_state)),
        data::SchemaModification::Revert => {
            let split = state.split_last();
            let backwards_split = backwards_state.split_last();
            match (split, backwards_split) {
                (Some((_, init_state)), Some((_, backward_init_state))) => {
                    Ok((init_state.to_vec(), backward_init_state.to_vec()))
                },
                _ => Err(StateError::RevertError),// Can't revert if the state is empty
            }
        },
        data::SchemaModification::Delete => {
            Ok((empty_state_stack.to_owned(), empty_backwards_state_stack.to_owned()))
        },
    }
}

// Gets the next modification state from using the current state, and the next modifcation
fn unroll_next_modification(
    modification: &data::SchemaModification,
    state: data::SchemaState,
    backwards_state: Option<data::SchemaModification>,
) -> Result<(data::SchemaState, Option<data::SchemaModification>), StateError> {

    let initial_state = data::SchemaState {
        columns: vec![],
        constraint: vec![],
    };
    let state_vec = vec![state];
    let backwards_state_vec = match backwards_state {
        Some(x) => vec![x],
        None => vec![],
    };
    let (state_result, backwards_state_result) = unroll_one_modification(modification, state_vec, backwards_state_vec)?;

    let final_state = state_result.last()
        .unwrap_or(&initial_state)
        .to_owned();
    let final_backwards_state = match backwards_state_result.last() {
        Some(x) => Some(x.to_owned()),
        None => None
    };

    Ok((final_state, final_backwards_state))
}


fn unroll_modifications(
    modifications: Vec<data::SchemaModification>
) -> Result<(data::SchemaState, Option<data::SchemaModification>), StateError> {

    let initial_state = data::SchemaState {
        columns: vec![],
        constraint: vec![],
    };
    let empty_state_stack: Vec<data::SchemaState> = vec![];
    let empty_backwards_state_stack: Vec<data::SchemaModification> = vec![];

    let (state_result, backwards_state_result) = modifications.iter()
        .fold(Ok((empty_state_stack.to_owned(), empty_backwards_state_stack.to_owned())),
              |state_result, modification| {
            state_result.and_then(|(state, backwards_state)|
                unroll_one_modification(modification, state, backwards_state)
            )
        })?;

    let final_state = state_result.last()
        .unwrap_or(&initial_state)
        .to_owned();
    let final_backwards_state = match backwards_state_result.last() {
        Some(x) => Some(x.to_owned()),
        None => None
    };

    Ok((final_state, final_backwards_state))
}

fn unroll_modification_commits(
    modification_commits: Vec<data::SchemaModificationCommit>
) -> Result<(data::SchemaState, Option<data::SchemaModification>), StateError> {

    let modifications: Vec<data::SchemaModification> = modification_commits.iter()
        .map(|modification_commit| modification_commit.action.to_owned())
        .collect::<Vec<data::SchemaModification>>();

    unroll_modifications(modifications)
}

//Fixme: this is not supposed to be public for all, this is public for table.rs,
pub fn unroll_table(table: data::DetailedTable) -> Result<data::Table, StateError> {
    let modifications = table.schema.iter()
        .map(|x| x.action.to_owned())
        .collect::<Vec<data::SchemaModification>>();
    let (schema_state, _) = unroll_modifications(modifications)?;
    let unrolled_table = data::Table {
        name: table.name,
        description: table.description,
        schema: schema_state,
    };

    Ok(unrolled_table)
}

fn format_column(column: &data::Column) -> String {
    let result = match &column.data_type {
        data::DataType::String => format!("{} TEXT", &column.name),
        data::DataType::Integer => format!("{} INTEGER", &column.name),
        data::DataType::Json => format!("{} JSON", &column.name),
    };

    result.to_string()
}

fn run_sql_commands_for_modification(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    new_modification: &data::SchemaModification,
    table_name: String,
    is_empty: bool
) -> Result<(), diesel::result::Error> {

    let sql_commands = match new_modification {
        data::SchemaModification::Create { columns, constraint, } => {
            let formatted_columns: Vec<String> = columns.iter().map(|x| format_column(x)).collect();
            let mut commands = if is_empty {
                //TODO: escape values?
                vec![format!("CREATE TABLE {} ({});", table_name, formatted_columns.join(", "))]
            } else {
                if columns.is_empty() {
                    vec![]
                } else {
                    vec![format!("ALTER TABLE {} ADD COLUMN {};", table_name, formatted_columns.join(", ADD COLUMN"))]
                }
            };

            for cons in constraint {
                match cons {
                    data::Constraint::Key(x) => {
                        commands.append(&mut vec![format!("ALTER TABLE {} ADD PRIMARY KEY ({});", table_name, x)]); //FIXME: SQL INJECTION!!!!!!
                    },
                    data::Constraint::Unique(x) => {}, //TODO:...
                    data::Constraint::UniqueTogether(xs) => {}, //TODO:...
                    data::Constraint::Check(expression) => {}, //TODO:...
                    data::Constraint::Reference { column, foreign_table, foreign_column, } => {}, //TODO:...
                    data::Constraint::ReferenceTogether { columns, foreign_table, foreign_columns, } => {}, //TODO:...
                };
            }

            commands
        },
        data::SchemaModification::Remove { column, constraint, } => {
            vec![] //TODO: ...
        },
        data::SchemaModification::Rename { mapping, } => {
            vec![] //TODO: ...
        },
        data::SchemaModification::Raw { up, down, } => {
            vec![] //TODO: ...
        },
        data::SchemaModification::Nop => {
            vec![] //TODO: ...
        },
        data::SchemaModification::Revert => {
            vec![] //TODO: ...
        },
        data::SchemaModification::Delete => {
            vec![format!("DROP TABLE IF EXISTS {}", table_name)]
        },
    };

    println!("running for: {:?}", sql_commands);

    for cmd in sql_commands.iter() {
        diesel::dsl::sql_query(cmd.to_owned()).execute(conn)?;
    }

    Ok(())
}

fn update_migration(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_name: String,
    schema_state: data::SchemaState,
    last_modification: Option<data::SchemaModification>,
    new_modification: data::SchemaModification,
) -> Result<data::SchemaState, diesel::result::Error> {

    let data::SchemaState { columns, .. } = schema_state.to_owned();
    let is_empty = columns.is_empty();

    let sql_commands = run_sql_commands_for_modification(conn, &new_modification, table_name, is_empty)?;

    let (next_schema_state, _) = unroll_next_modification(&new_modification, schema_state, last_modification)
        .or_else(|err| {
            Err(diesel::result::Error::SerializationError(Box::new(err.compat())))
        })?;

    Ok(next_schema_state)
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

fn get_modification_commits(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<Vec<data::SchemaModificationCommit>, diesel::result::Error> {

    let table_history_items = get_table_history_items(conn, &table_schema)?;
    let modification_commits = parse_table_history_items_to_modification_commits(&table_history_items)?;

    Ok(modification_commits)
}

//Fixme: this is not supposed to be public for all, this is public for table.rs,
pub fn get_single_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<data::DetailedTable, diesel::result::Error> {

    let table_history_items = get_table_history_items(conn, &table_schema)?;
    let modification_commits = parse_table_history_items_to_modification_commits(&table_history_items)?;

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

pub fn create_table(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
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


pub fn create_query(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    post_table: api::PostQuery
) -> Result<api::CreateTableResult, api::Error> {

    Err(api::Error::UnknownError)
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