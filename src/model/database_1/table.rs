
use std::ops::Deref;
use std::os::raw;
use std::ffi::{CString, CStr};
use std::ptr::NonNull;
use std::mem::transmute_copy;
use std::mem;
use std::{slice, str};
use std::io;
use std::ptr;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};

use serde_json;

use data;
use data::{RowData, Table, Value, DataType, TableData};
use diesel::pg::Pg;
use diesel::sql_types;
use diesel::deserialize::FromSql;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::serialize::IsNull;

use base64;


pub fn upsert_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //INSERT INTO "table_name" ("col_1_pk", "col_2", "col_3")
        //    VALUES (1, 2, 3)
        //    ON CONFLICT (col_1_pk) DO UPDATE SET col_2 = EXCLUDED.col_2, col_3 = EXCLUDED.col_3;
        //TODO: insert multiple values at once
        let row_column_names: Vec<String> = item.keys().cloned().map(|x| format!("\"{}\"", x)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_insert_into = format!(
            "INSERT INTO \"{}\" ({})",
            table_name, row_column_names.join(", "));
        let query_values = format!(
            "VALUES ({})",
            (0..row_column_names.len())
                .map(|x| format!("${}", x+1))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let query_on_conflict = if column_names_without_key.len() > 0 {
            format!(
                "ON CONFLICT ({}) DO UPDATE SET {}",
                key_column_name,
                column_names_without_key.iter().map(|x| format!("{} = EXCLUDED.{}", x, x))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            "".to_string()
        };
        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{}\n{};", query_insert_into, query_values, query_on_conflict, query_returning);
        println!("query: {}", query);

        println!("values: {:?}", values);
        let result = exec(&internal_conn, &query, values)?;

        println!("printing errors: ");
        result.print_error();

        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}

pub fn insert_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData,
    ignore_on_failure: bool,
) -> Result<TableData, Error> {
    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //INSERT INTO "table_name" ("col_1_pk", "col_2", "col_3")
        //    VALUES (1, 2, 3)
        //    ON CONFLICT (col_1_pk) DO UPDATE SET col_2 = EXCLUDED.col_2, col_3 = EXCLUDED.col_3;
        //TODO: insert multiple values at once
        let row_column_names: Vec<String> = item.keys().cloned().map(|x| format!("\"{}\"", x)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<String> = row_column_names.iter()
            .filter(|&x| x.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_insert_into = format!(
            "INSERT INTO \"{}\" ({})",
            table_name, row_column_names.join(", "));
        let query_values = format!(
            "VALUES ({})",
            (0..row_column_names.len())
                .map(|x| format!("${}", x+1))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let query_on_conflict = if ignore_on_failure {
            format!(
                "ON CONFLICT ({}) DO NOTHING",
                key_column_name,
            )
        } else {
            "".to_string()
        };
        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{}\n{};", query_insert_into, query_values, query_on_conflict, query_returning);
        println!("query: {}", query);

        println!("values: {:?}", values);
        let result = exec(&internal_conn, &query, values)?;
        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}

pub fn update_rows(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    to_insert: TableData,
) -> Result<TableData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);


    let mut rows: Vec<Vec<Value>> = vec![];
    for item in to_insert.into_rows_data_vec() {
        //UPDATE "table_name" SET "col_2" = 2, "col_3" = 3
        //    WHERE "col_1_pk" = 1;
        //TODO: insert multiple values at once
        let row_column_names: Vec<(String, usize)> = item.keys().cloned().enumerate()
            .map(|(i, x)| (format!("\"{}\"", x), i+1)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
        let column_names_without_key: Vec<(String, usize)> = row_column_names.iter()
            .filter(|&x| x.0.to_owned() != key_column_name)
            .cloned()
            .collect();
        let column_names_only_key: Vec<(String, usize)> = row_column_names.iter()
            .filter(|&x| x.0.to_owned() == key_column_name)
            .cloned()
            .collect();

        if column_names_only_key.len() != 1 {
            return Err(generate_error("no primary key given for data insertion"));
        }

        let values: Vec<Value> = item.values().cloned().collect();

        let query_update_value = format!(
            "UPDATE \"{}\" SET {}",
            table_name,
            column_names_without_key.iter()
                .map(|(x, i)| format!("{} = ${}", x, i))
                .collect::<Vec<String>>()
                .join(", "));
        let where_clause = format!(
            "WHERE {}",
            column_names_only_key.iter()
                .map(|(x, i)| format!("{} = ${}", x, i))
                .collect::<Vec<String>>()
                .join(" AND ")
        );

        let query_returning = format!(
            "RETURNING {}",
            column_names.join(", ")
        );
        let query = format!("{}\n{}\n{};", query_update_value, where_clause, query_returning);
        println!("query: {}", query);

        println!("values: {:?}", values);
        let result = exec(&internal_conn, &query, values)?;
        let mut curr_row = result.get_rows_data()?;

        rows.append(&mut curr_row);
    }

    let table_data = TableData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: rows
    };

    Ok(table_data)
}


pub fn update_row_with_key(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    row_data: RowData,
    key_value: String,
) -> Result<RowData, Error> {

    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);

    //UPDATE "table_name" SET "col_2" = 2, "col_3" = 3
    //    WHERE "col_1_pk" = 1;
    let item = row_data.into_row_data_vec();
    let row_column_names: Vec<(String, usize)> = item.keys().enumerate()
        .map(|(i, x)| (format!("\"{}\"", x), i+1)).collect(); //TODO: SQL INJECTION!!!!!!!!!!!!!!!!!!!
    let mut values: Vec<Value> = item.values().cloned().collect();

    let query_update_value = format!(
        "UPDATE \"{}\" SET {}",
        table_name,
        row_column_names.iter()
            .map(|(x, i)| format!("{} = ${}", x, i))
            .collect::<Vec<String>>()
            .join(", "));
    let where_clause = format!(
        "WHERE {} = ${}",
        key_column_name,
        row_column_names.len() + 1
    );

    let query_returning = format!(
        "RETURNING {}",
        column_names.join(", ")
    );
    let query = format!("{}\n{}\n{};", query_update_value, where_clause, query_returning);
    println!("query: {}", query);

    values.push(Value::String(key_value));
    println!("value: {:?}", values);
    let result = exec(&internal_conn, &query, values)?;
    let rows = result.get_rows_data()?;
    let row: Vec<Value> = match rows.first() {
        None => Err(generate_error("no row")),
        Some(x) => Ok(x.to_owned()),
    }?;

    let row_data = RowData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: row,
    };

    Ok(row_data)
}

pub fn delete_row_with_key(
    conn: &PooledConnection<ConnectionManager<PgConnection>>,
    table: &Table,
    key_value: String, //TODO: this should be value based on type
) -> Result<RowData, Error> {


    let db: &PgConnection = conn.deref();
    let internal_conn = conn_ptr(&db);

    let table_name = &table.name;
    let columns = &table.schema.columns;
    let column_names = columns.iter()
        .map(|x| format!("\"{}\"", x.name))
        .collect::<Vec<String>>();
    let key = table.get_key().ok_or(generate_error("no key"))?;
    let key_column_name = format!("\"{}\"", key);

    //DELETE FROM "table_name" WHERE "col_1_pk" = 1;

    let delete_from = format!(
        "DELETE FROM \"{}\" ",
        table_name
    );
    let where_clause = format!(
        "WHERE {} = $1",
        key_column_name
    );

    let query_returning = format!(
        "RETURNING {}",
        column_names.join(", ")
    );
    let query = format!("{}\n{}\n{};", delete_from, where_clause, query_returning);
    println!("query: {}", query);

    let values = vec![Value::String(key_value)];
    println!("values: {:?}", values);
    let result = exec(&internal_conn, &query, values)?;
    let rows = result.get_rows_data()?;
    let row: Vec<Value> = match rows.first() {
        None => Err(generate_error("no row")),
        Some(x) => Ok(x.to_owned()),
    }?;

    let row_data = RowData::RowsFlatData {
        columns: columns.iter().map(|x| x.name.to_owned()).collect(),
        data: row,
    };

    Ok(row_data)
}