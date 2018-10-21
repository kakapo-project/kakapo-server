extern crate diesel;
extern crate diesel_dynamic_schema;

use diesel::*;
use diesel::sql_types::*;
use diesel::sqlite::Sqlite;
use diesel_dynamic_schema::{schema, table, VecColumn, ValueList};

#[test]
fn querying_basic_schemas() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users DEFAULT VALUES")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.column::<Integer, _>("id");
    let ids = users.select(id).load::<i32>(&conn);
    assert_eq!(Ok(vec![1]), ids);
}

#[test]
fn querying_multiple_types() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.column::<Integer, _>("id");
    let name = users.column::<Text, _>("name");
    let users = users.select((id, name)).load::<(i32, String)>(&conn);
    assert_eq!(Ok(vec![(1, "Sean".into()), (2, "Tess".into())]), users);
}

#[test]
fn columns_used_in_where_clause() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.column::<Integer, _>("id");
    let name = users.column::<Text, _>("name");
    let users = users
        .select((id, name))
        .filter(name.eq("Sean"))
        .load::<(i32, String)>(&conn);
    assert_eq!(Ok(vec![(1, "Sean".into())]), users);
}

/*
#[test]
fn querying_basic_schemas_with_dynamic_return_type() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (name) VALUES ('Lenny')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.column::<Integer, _>("id");
    let name = users.column::<Text, _>("name");
    let ids = users
        .select((id, name))
        .load::<(AnyValue, AnyValue)>(&conn);

    assert_eq!(Ok(vec![
        (AnyValue::Integer(1), AnyValue::Text("Lenny".into()))
    ]), ids);
}

#[test]
fn querying_basic_schemas_with_dynamic_return_type_and_dynamic_input_type() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (name) VALUES ('Carl')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id: DynamicColumn<_, _> = users.dynamic_column("id", DynamicType::Integer);
    let name: DynamicColumn<_, _> = users.dynamic_column("name", DynamicType::Text);
    let ids = users
        .select((id, name))
        .load::<(AnyValue, AnyValue)>(&conn);

    assert_eq!(Ok(vec![
        (AnyValue::Integer(1), AnyValue::Text("Carl".into()))
    ]), ids);
}
*/

/*
#[test]
fn querying_basic_schemas_with_dynamically_sized_return_type_and_dynamic_input_type() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (name) VALUES ('Carl')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.dynamic_column("id", DynamicType::Integer);
    let name = users.dynamic_column("name", DynamicType::Text);
    let ids = users
        .select((id, name))
        .load::<Vec<AnyValue>>(&conn);

    assert_eq!(Ok(vec![
        vec![AnyValue::Integer(1), AnyValue::Text("Carl".into())]
    ]), ids);
}
*/

#[test]
fn querying_basic_schemas_with_dynamically_sized_return_type_and_dynamically_sized_input_type() {
    let conn = establish_connection();
    sql_query("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, first_name TEXT NOT NULL, last_name TEXT NOT NULL)")
        .execute(&conn)
        .unwrap();
    sql_query("INSERT INTO users (first_name, last_name) VALUES ('Carl', 'Carlson')")
        .execute(&conn)
        .unwrap();

    let users = table("users");
    let id = users.column::<Text, _>("first_name");
    let name = users.column::<Text, _>("last_name");
    let query = users
        .select(VecColumn::new(vec![id, name]));

    let debug_print = diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query);
    println!("QUERY: {:?}", debug_print);

    let ids = users
        .select(VecColumn::new(vec![id, name]))
        .load::<ValueList>(&conn);

    let row = ids.map(|x| x[0].clone().to_vector());

    assert_eq!(Ok(
        vec!["Carl".into(), "Carlson".into()]
    ), row);
}


#[test]
fn providing_custom_schema_name() {
    let table = schema("information_schema").table("users");
    let sql = debug_query::<Sqlite, _>(&table);
    assert_eq!("`information_schema`.`users` -- binds: []", sql.to_string());
}

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(":memory:").unwrap()
}
