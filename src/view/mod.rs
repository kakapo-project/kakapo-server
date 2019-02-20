


pub mod error;
pub mod websocket;

pub mod procedure;
pub mod routes;
pub mod action_wrapper;
pub mod extensions;

use std::result::Result;
use std::result::Result::Ok;
use actix_web::test::TestApp;

// current module
use model::actions;

use view::procedure::NoQuery;
use view::extensions::ProcedureExt;
use data;
use actix_web::middleware::cors::CorsBuilder;
use model::actions::Action;
use serde_json::Value;
use serde_json::Error;
use serde_json::from_value;
use connection::AppStateLike;



#[cfg(test)]
mod test {
    use super::*;
    use view::routes::*;

    #[test]
    fn test_get_all_entities() {
        let data = json!({});
        let query = json!({
            "showDeleted": true
        });
        let _all_tables_action = manage::get_all_tables(data.to_owned(), query.to_owned()).unwrap();
        let _all_queries_action = manage::get_all_queries(data.to_owned(), query.to_owned()).unwrap();
        let _all_scripts_action = manage::get_all_scripts(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_get_entity() {
        let data = json!({});
        let query = json!({
            "name": "foo"
        });
        let _get_table_action = manage::get_table(data.to_owned(), query.to_owned()).unwrap();
        let _get_query_action = manage::get_query(data.to_owned(), query.to_owned()).unwrap();
        let _get_script_action = manage::get_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_create_entity() {
        let query = json!({});
        let mut data = json!({
            "name": "table_name",
            "description": "this is a really cool table",
            "schema": {
                "columns": [
                    {
                        "name": "col_a",
                        "dataType": "integer"
                    }
                ],
                "constraint": []
            }
        });
        let _create_table_action = manage::create_table(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "query_name",
            "description": "this is a really cool query",
            "statement": "SELECT * FROM awesome_table"
        });
        let _create_query_action = manage::create_query(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "script_name",
            "description": "this is a really cool script",
            "text": "print('hello world')"
        });
        let _create_script_action = manage::create_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_update_entity() {
        let query = json!({
            "name": "foo"
        });

        let mut data = json!({
            "name": "table_name",
            "description": "this is a really cool table",
            "schema": {
                "columns": [
                    {
                        "name": "col_a",
                        "dataType": "integer"
                    }
                ],
                "constraint": []
            }
        });
        let _update_table_action = manage::update_table(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "query_name",
            "description": "this is a really cool query",
            "statement": "SELECT * FROM awesome_table"
        });
        let _update_query_action = manage::update_query(data.to_owned(), query.to_owned()).unwrap();

        data = json!({
            "name": "script_name",
            "description": "this is a really cool script",
            "text": "print('hello world')"
        });
        let _update_script_action = manage::update_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_delete_entity() {
        let data = json!({});
        let query = json!({
            "name": "foo"
        });
        let _delete_table_action = manage::delete_table(data.to_owned(), query.to_owned()).unwrap();
        let _delete_query_action = manage::delete_query(data.to_owned(), query.to_owned()).unwrap();
        let _delete_script_action = manage::delete_script(data.to_owned(), query.to_owned()).unwrap();
    }

    #[test]
    fn test_query_table_data() {

    }

    #[test]
    fn test_insert_table_data() {

    }

    #[test]
    fn test_modify_table_data() {

    }

    #[test]
    fn test_remove_table_data() {

    }
}