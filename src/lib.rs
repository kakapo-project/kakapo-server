#![allow(proc_macro_derive_resolution_fallback)]

// Crates
extern crate actix;
extern crate actix_broker;
extern crate actix_web;
extern crate base64;
extern crate bcrypt;
extern crate bigdecimal;
extern crate byteorder;
extern crate bytes;
extern crate chrono;
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

// Mods
mod view;
mod model;
mod scripting;
mod data;
mod database;
mod connection;

// Extenal dependencies
use actix::prelude::*;
use actix_web::middleware::cors::CorsBuilder;

use log::LevelFilter;
use env_logger::{Builder, Target};

// Internal dependencies
use view::server;
use scripting::ScriptFunctions;

pub use connection::AppStateBuilder as KakapoStateBuilder;
pub use connection::AppState as KakapoState;
pub use connection::GetAppState as GetKakapoState;
pub use connection::Auth as KakapoAuth;
pub use connection::Broadcaster as KakapoBroadcaster;


pub trait KakapoRouter<S, A, B>
    where
        S: GetKakapoState<A, B> + 'static,
        A: KakapoAuth,
        B: KakapoBroadcaster,
{
    fn kakapo_routes(&mut self) -> &mut Self;
}

impl<S, A, B> KakapoRouter<S, A, B> for CorsBuilder<S>
    where
        S: GetKakapoState<A, B> + 'static,
        A: KakapoAuth,
        B: KakapoBroadcaster,
{
    fn kakapo_routes(&mut self) -> &mut CorsBuilder<S> {
        server::router::<S, A, B>(self)
    }
}