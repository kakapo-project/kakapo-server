
/// Crates
extern crate actix;
extern crate actix_broker;
extern crate actix_web;
extern crate byteorder;
extern crate bytes;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
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
extern crate tokio_core;

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

    let sys = actix::System::new("Kakapo");

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