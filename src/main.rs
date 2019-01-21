#![allow(proc_macro_derive_resolution_fallback)]

/// Crates
extern crate actix;
extern crate actix_broker;
extern crate actix_web;
extern crate base64;
extern crate bcrypt;
extern crate bigdecimal;
extern crate byteorder;
extern crate bytes;
extern crate chrono;
extern crate clap;
extern crate cpython;
extern crate dotenv;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate http;
extern crate json;
extern crate jsonwebtoken;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate openssl;
extern crate tokio;
extern crate tokio_core;

/// Mods
mod view;
mod model;
mod scripting;
mod data;
mod database;
mod connection;

/// Extenal dependencies
use log::LevelFilter;
use env_logger::{Builder, Target};
use clap::{Arg, App};

/// Internal dependencies
use view::server;
use scripting::ScriptFunctions;

fn main() {

    let matches = App::new("Kakapo")
        .version("0.1.0")
        .author("Atta Z. <atta.h.zadeh@gmail.com>")
        .about("Database utility and Crud app creator")
        .arg(Arg::with_name("Verbosity")
            .short("v")
            .long("verbose")
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("Reconfigure")
            .long("reconfigure")
            .help("Set up the initial configuration again"))
        .arg(Arg::with_name("No Auth")
            .long("no-auth")
            .help("Do not authenticate user, [WARNING: don't use this in production]"))
        .get_matches();

    //std::env::set_var("RUST_LOG", "warn,actix_web=info,kakapo=all");
    //std::env::set_var("RUST_BACKTRACE","1");
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