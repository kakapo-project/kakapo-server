
pub mod management;

pub mod types;
pub mod rows;
pub mod query;
pub mod table;
pub mod schema;
pub mod repository;

pub fn initialize() {

}

fn setup_database() {
    println!("Initializing database");

    //let table = Table::new("_meta_table");
    //create_table(&table);
}

fn is_database_setup() -> bool {
    false //TODO: implement
}

fn setup_database_if_not_initialized() {
    if !is_database_setup() {
        setup_database();
    }
}