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
use connection::executor::DatabaseExecutor;

use super::super::dbdata::*;


pub fn generate_error(fmt: &str) -> Error {
    Error::SerializationError(
        Box::new(
            io::Error::new(
                io::ErrorKind::Other, fmt
            )
        )
    )
}

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

pub fn unroll_modification_commits(
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
        data::DataType::SmallInteger => format!("{} SMALLINT", &column.name),
        data::DataType::Integer => format!("{} INTEGER", &column.name),
        data::DataType::BigInteger => format!("{} BIGINT", &column.name),
        data::DataType::Float => format!("{} REAL", &column.name),
        data::DataType::DoubleFloat => format!("{} DOUBLE PRECISION", &column.name),

        data::DataType::String => format!("{} TEXT", &column.name),
        data::DataType::VarChar { length } => format!("{} VARCHAR({})", &column.name, &length),

        data::DataType::Byte => format!("{} BYTEA", &column.name),

        data::DataType::Timestamp { with_tz } => format!("{} TIMESTAMP {}", &column.name, if *with_tz { "WITH TIMEZONE" } else { "WITHOUT TIMEZONE" }),
        data::DataType::Date => format!("{} DATE", &column.name),
        data::DataType::Time { with_tz } => format!("{} TIME {}", &column.name, if *with_tz { "WITH TIMEZONE" } else { "WITHOUT TIMEZONE" }),

        data::DataType::Boolean => format!("{} BOOLEAN", &column.name),

        data::DataType::Json => format!("{} JSON", &column.name),
    };

    result.to_string()
}

pub fn run_sql_commands_for_modification(
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

pub fn update_migration(
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

pub fn get_table_history_items(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table_schema: &TableSchema
) -> Result<Vec<TableSchemaHistory>, diesel::result::Error> {

    let table_history_items: Vec<TableSchemaHistory> = table_schema_history::table
        .filter(table_schema_history::table_schema_id.eq(table_schema.table_schema_id))
        .order_by(table_schema_history::modified_at.asc())
        .load::<TableSchemaHistory>(conn)?;

    Ok(table_history_items)
}

pub fn get_modification_commits(
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