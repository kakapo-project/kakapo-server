#![allow(proc_macro_derive_resolution_fallback)]

// Crates
extern crate actix;
extern crate actix_web;
extern crate argonautica;
extern crate base64;
extern crate bcrypt;
extern crate bigdecimal;
extern crate byteorder;
extern crate bytes;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dirs;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate json;
extern crate jsonwebtoken;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate openssl;
extern crate tempfile;
#[macro_use]
extern crate time_test;
extern crate tokio;
extern crate tokio_core;
extern crate uuid;

// Mods
mod auth;
mod view;
mod model;
mod scripting;
mod data;
mod database;
mod connection;
mod metastore;
mod pubsub;
mod server;

//#[cfg(test)]
pub mod test_common;


// Extenal dependencies
use actix_web::middleware::cors::CorsBuilder;


// Internal dependencies
pub use connection::AppStateBuilder;
pub use connection::AppState;
pub use connection::AppStateLike;
use actix_web::test::TestApp;

use env_logger::Builder;
use env_logger::Target;
use log::LevelFilter;


pub fn run() {

    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Warn)
        .filter_module("kakapo", LevelFilter::Debug)
        .filter_module("actix_web", LevelFilter::Info)
        .init();

    let sys = actix::System::new("Kakapo");

    server::serve();

    // loop
    sys.run();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_run_server() {
        run();
    }
}