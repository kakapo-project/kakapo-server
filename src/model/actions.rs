

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

use super::api;
use super::data;
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
            .and_then::<TableSchema, _>(|result| {
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


        let modification = serde_json::to_value(&post_table.action)
            .or_else(|err| {
                Err(Error::SerializationError(Box::new(err)))
            })?;

        let table_schema_history = insert_into(table_schema_history::table)
            .values(&NewTableSchemaHistory {
                table_schema_id: table_schema.table_schema_id,
                description: post_table.description,
                modification: modification,
                //TODO: deleted
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
                let table_history_items: Vec<TableSchemaHistory> = table_schema_history::table
                    .filter(table_schema_history::table_schema_id.eq(table_schema.table_schema_id))
                    .order_by(table_schema_history::modified_at.asc())
                    .load::<TableSchemaHistory>(&conn)?;

                let last_table_history_item = table_history_items.last()
                    .ok_or(diesel::result::Error::NotFound)?;

                let description = &last_table_history_item.description;

                let modifications: Vec<data::SchemaModification> = table_history_items.iter()
                    .map(|table_history_item| {
                        let schema_modification = data::SchemaModification {
                            date: table_history_item.modified_at,
                            action: serde_json::from_value::<data::SchemaAction>(table_history_item.modification.to_owned())?,
                        };
                        Ok(schema_modification)
                    })
                    .collect::<Result<Vec<data::SchemaModification>, serde_json::Error>>()
                    .or_else(|err| Err(diesel::result::Error::SerializationError(Box::new(err))))?;

                let detailed_table = data::DetailedTable {
                    name: table_schema.name.to_string(),
                    description: description.to_string(),
                    schema: modifications,
                };

                Ok(detailed_table)
            })
            .collect::<Result<Vec<data::DetailedTable>, diesel::result::Error>>()?;

        Ok(api::GetTablesResult::DetailedTables(tables))
    });

    result.or_else(|err| Err(api::Error::DatabaseError(err)))
}
