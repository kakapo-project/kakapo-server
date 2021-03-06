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
extern crate inflector;
extern crate json;
extern crate jsonwebtoken;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_redis;
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
mod connection;
mod metastore;
mod broker;
mod server;
mod state;

pub mod kakapo_postgres; //TODO: move this outside
pub mod kakapo_redis; //TODO: move this outside

pub mod plugins;

//#[cfg(test)]
pub mod test_common;


// Extenal dependencies
use actix_web::middleware::cors::CorsBuilder;


// Internal dependencies
pub use connection::AppStateBuilder;
pub use connection::AppState;
pub use connection::AppStateLike;
pub use metastore::setup_admin;
pub use server::Server;

use actix_web::test::TestApp;
use env_logger::Builder;
use env_logger::Target;
use log::LevelFilter;
use diesel::prelude::*;
use std::net::ToSocketAddrs;

#[cfg(test)]
mod test {
    use super::*;
    use std::net;
    use std::io;

    #[test]
    fn test_run_server() {

        Builder::new()
            .target(Target::Stdout)
            .filter_level(LevelFilter::Warn)
            .filter_module("kakapo", LevelFilter::Debug)
            .filter_module("actix_web", LevelFilter::Info)
            .init();

        let database_url = format!(
            "postgres://{user}:{pass}@{host}:{port}/{db}", //TODO: ...
            user = "test",
            pass = "password",
            host = "localhost",
            port = 5432,
            db = "test",
        );
        let result = setup_admin(&database_url, "admin", "admin@example.com", "Admin", "password");
        if let Err(error) = result {
            panic!();
            return;
        }

        let plugin = kakapo_postgres::KakapoPostgres::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .db("test");

        let state = AppStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .num_threads(1)
            .password_secret("Hello World Hello Wold")
            .token_secret("Hello World Hello Wold")
            .add_plugin("Sirocco", plugin);

        Server::new()
            .host("127.0.0.1")
            .port(1845)
            .run(state);
    }
}