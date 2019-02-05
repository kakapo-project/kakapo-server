#![allow(proc_macro_derive_resolution_fallback)]

// Crates
extern crate actix;
extern crate actix_broker;
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
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate openssl;
extern crate tokio;
extern crate tokio_core;
extern crate uuid;

// Mods
mod view;
mod model;
mod scripting;
mod data;
mod database;
mod connection;

//#[cfg(test)]
pub mod test_common;


// Extenal dependencies
use actix_web::middleware::cors::CorsBuilder;


// Internal dependencies
pub use connection::AppStateBuilder as KakapoStateBuilder;
pub use connection::AppState as KakapoState;
pub use connection::GetAppState as GetKakapoState;
pub use connection::Broadcaster as KakapoBroadcaster;
pub use connection::Channels;
pub use connection::BroadcasterError;
use actix_web::test::TestApp;


pub trait KakapoRouter<S, B>
    where
        S: GetKakapoState<B> + 'static,
        B: KakapoBroadcaster,
{
    fn kakapo_routes(&mut self) -> &mut Self;
}

impl<S, B> KakapoRouter<S, B> for CorsBuilder<S>
    where
        S: GetKakapoState<B> + 'static,
        B: KakapoBroadcaster,
{
    fn kakapo_routes(&mut self) -> &mut Self {
        view::Router::<S, B>::router(self)
    }
}

impl<S, B> KakapoRouter<S, B> for TestApp<S>
    where
        S: GetKakapoState<B> + 'static,
        B: KakapoBroadcaster,
{
    fn kakapo_routes(&mut self) -> &mut Self {
        view::Router::<S, B>::router(self)
    }
}