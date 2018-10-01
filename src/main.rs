extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate log;
extern crate env_logger;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate json;
#[macro_use]
extern crate objekt;

mod database;
mod controller;
mod server;

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