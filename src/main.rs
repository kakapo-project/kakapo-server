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
#[macro_use]
extern crate json;
extern crate jsonwebtoken;
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
use clap::{Arg, App, SubCommand};

/// Internal dependencies
use view::server;
use model::connection;

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

    server::serve();

    /// loop
    sys.run();
}