
use actix::prelude::*;

use actix_web::{
    http,
    FromRequest, Json, Query,
    HttpRequest,
};

use actix_web::middleware::cors::CorsBuilder;
use actix_web::dev::JsonConfig;

use super::action_wrapper::ActionWrapper;

use super::procedure::ProcedureBuilder;
use super::procedure::ProcedureHandler;
use super::procedure::procedure_handler_function;
use super::procedure::procedure_bad_request_handler_function;

use model::actions::Action;
use std::fmt::Debug;
use serde::Serialize;
use connection::executor::Executor;
use actix_web::test::TestApp;
use connection::AppStateLike;
use AppState;
use view::route_builder::RouteBuilder;

// use actix_web::dev::QueryConfig; //NOTE: for some reason this can't be imported, probably actix_web issue


pub trait ProcedureExt {

    fn add_routes(&mut self, builder: &RouteBuilder) -> &mut Self;

}


impl<S> ProcedureExt for CorsBuilder<S>
    where S: AppStateLike
{
    fn add_routes(&mut self, builder: &RouteBuilder) -> &mut Self {
        /*
        self.resource(path, move |r| {
            r.method(http::Method::POST).with_config(
                move |(req, json_params, query_params): (HttpRequest<S>, Json<JP>, Query<QP>)| {
                    let proc = ProcedureHandler::<S, JP, QP, PB, A>::setup(&procedure_builder);
                    procedure_handler_function(proc, req, json_params, query_params)
                },
                |((_, json_cfg, _query_cfg),)| {
                    json_cfg
                        .error_handler(|err, _req| {
                            procedure_bad_request_handler_function(err)
                        });
                }
            );
        })
        */
        self
    }
}

impl<S> ProcedureExt for TestApp<S>
    where S: AppStateLike
{
    fn add_routes(&mut self, builder: &RouteBuilder) -> &mut Self {
        /*
        self.resource(path, move |r| {
            r.method(http::Method::POST).with_config(
                move |(req, json_params, query_params): (HttpRequest<S>, Json<JP>, Query<QP>)| {
                    let proc = ProcedureHandler::<S, JP, QP, PB, A>::setup(&procedure_builder);
                    procedure_handler_function(proc, req, json_params, query_params)
                },
                |((_, json_cfg, _query_cfg),)| {
                    json_cfg
                        .error_handler(|err, _req| {
                            procedure_bad_request_handler_function(err)
                        });
                }
            );
        })
        */

        self
    }
}
