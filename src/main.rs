
/// Crates
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate json;
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate objekt;

/// Mods
mod view;
mod model;
mod auth;


/// Extenal dependencies
use log::LevelFilter;
use env_logger::{Builder, Target};

/// Internal dependencies
use view::router;
use model::connection;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .init();

    let sys = actix::System::new("ninchy");

    /*
    // https://actix.rs/docs/server/
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file("cert.pem").unwrap();
    */

    actix_web::server::new(router::routes)
        .workers(num_cpus::get())
        .keep_alive(None)
        .bind("127.0.0.1:8080")
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");

    /// loop
    sys.run();
}

/*

pub mod controller;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use controller::schema;
use diesel::sql_types;

mod database;

use chrono::prelude::*;

#[derive(Debug, Queryable)]
struct Version {
    pub version_id: i32,
    pub version_update: String,
    pub update_at: NaiveDateTime,
}

fn main() {
    use controller::schema::version::dsl::*;

    let connection = database::establish_connection();
    let results = version
        .limit(5)
        .load::<Version>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post);
        println!("----------\n");
    }
}
*/