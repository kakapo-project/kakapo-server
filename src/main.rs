extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate postgres;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
#[macro_use]
extern crate objekt;

mod actions;
mod database;
mod controller;
mod server;


/*



use log::LevelFilter;
use env_logger::{Builder, Target};

use server::router::routes;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();


    let sys = actix::System::new("ninchy");

    actix_web::server::new(routes)
        .bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
*/


use postgres::{Connection, TlsMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::connect("postgres://postgresrepo:passwordTEST@127.0.0.1:5432", TlsMode::None).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}: {}", person.id, person.name);
    }
}