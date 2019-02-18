
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
use view::routes::users;
use view::routes::manage;

// use actix_web::dev::QueryConfig; //NOTE: for some reason this can't be imported, probably actix_web issue

/// Build routes for rpc calls
pub trait ProcedureExt<S>
    where
        S: AppStateLike + 'static,
{
    /// Create an RPC call
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn add_route<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut Self
        where
            Executor: Handler<ActionWrapper<A>>,
            A: Action + Send + 'static,
            PB: ProcedureBuilder<S, JP, QP, A> + Clone + 'static,
            JP: Debug + 'static,
            QP: Debug + 'static,
            Json<JP>: FromRequest<S, Config = JsonConfig<S>>,
            Query<QP>: FromRequest<S>,
            <A as Action>::Ret: Send + Serialize;

    /// Add all the routes for the actix web server
    fn add_routes(&mut self) -> &mut Self;

}


impl<S> ProcedureExt<S> for CorsBuilder<S>
    where
        S: AppStateLike + 'static,
{
    fn add_route<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut Self
        where
            Executor: Handler<ActionWrapper<A>>,
            A: Action + Send + 'static,
            PB: ProcedureBuilder<S, JP, QP, A> + Clone + 'static,
            JP: Debug + 'static,
            QP: Debug + 'static,
            Json<JP>: FromRequest<S, Config = JsonConfig<S>>,
            Query<QP>: FromRequest<S>,
            <A as Action>::Ret: Send + Serialize,
    {
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
    }

    fn add_routes(&mut self) -> &mut Self {
        self
            .add_route("/manage/getAllTables", manage::get_all_tables)
            .add_route("/manage/getAllQueries", manage::get_all_queries)
            .add_route("/manage/getAllScripts", manage::get_all_scripts)

            .add_route("/manage/getTable", manage::get_table)
            .add_route("/manage/getQuery", manage::get_query)
            .add_route("/manage/getScript", manage::get_script)

            .add_route("/manage/createTable", manage::create_table)
            .add_route("/manage/createQuery", manage::create_query)
            .add_route("/manage/createScript", manage::create_script)

            .add_route("/manage/updateTable", manage::update_table)
            .add_route("/manage/updateQuery", manage::update_query)
            .add_route("/manage/updateScript", manage::update_script)

            .add_route("/manage/deleteTable", manage::delete_table)
            .add_route("/manage/deleteQuery", manage::delete_query)
            .add_route("/manage/deleteScript", manage::delete_script)

            .add_route("/manage/queryTableData", manage::query_table_data)
            .add_route("/manage/insertTableData", manage::insert_table_data)
            .add_route("/manage/modifyTableData", manage::modify_table_data)
            .add_route("/manage/removeTableData", manage::remove_table_data)

            .add_route("/manage/runQuery", manage::run_query)
            .add_route("/manage/runScript", manage::run_script)

            .add_route("/users/login", users::login)
            .add_route("/users/refresh", users::refresh)
            .add_route("/users/logout", users::logout)
            .add_route("/users/getAllUsers", users::get_all_users)

            .add_route("/users/addUser", users::add_user)
            .add_route("/users/removeUser", users::remove_user)
            .add_route("/users/inviteUser", users::invite_user)
            .add_route("/users/setupUser", users::setup_user)
            .add_route("/users/setUserPassword", users::set_user_password)

            .add_route("/users/addRole", users::add_role)
            .add_route("/users/removeRole", users::remove_role)
            .add_route("/users/getAllRoles", users::get_all_roles)

            .add_route("/users/attachPermissionForRole", users::attach_permission_for_role)
            .add_route("/users/detachPermissionForRole", users::detach_permission_for_role)

            .add_route("/users/attachRoleForUser", users::attach_role_for_user)
            .add_route("/users/detachRoleForUser", users::detach_role_for_user)
    }
}

impl<S> ProcedureExt<S> for TestApp<S>
    where
        S: AppStateLike + 'static,
{
    fn add_route<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut Self
        where
            Executor: Handler<ActionWrapper<A>>,
            A: Action + Send + 'static,
            PB: ProcedureBuilder<S, JP, QP, A> + Clone + 'static,
            JP: Debug + 'static,
            QP: Debug + 'static,
            Json<JP>: FromRequest<S, Config = JsonConfig<S>>,
            Query<QP>: FromRequest<S>,
            <A as Action>::Ret: Send + Serialize,
    {
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
    }

    fn add_routes(&mut self) -> &mut Self {
        self
            .add_route("/manage/getAllTables", manage::get_all_tables)
            .add_route("/manage/getAllQueries", manage::get_all_queries)
            .add_route("/manage/getAllScripts", manage::get_all_scripts)

            .add_route("/manage/getTable", manage::get_table)
            .add_route("/manage/getQuery", manage::get_query)
            .add_route("/manage/getScript", manage::get_script)

            .add_route("/manage/createTable", manage::create_table)
            .add_route("/manage/createQuery", manage::create_query)
            .add_route("/manage/createScript", manage::create_script)

            .add_route("/manage/updateTable", manage::update_table)
            .add_route("/manage/updateQuery", manage::update_query)
            .add_route("/manage/updateScript", manage::update_script)

            .add_route("/manage/deleteTable", manage::delete_table)
            .add_route("/manage/deleteQuery", manage::delete_query)
            .add_route("/manage/deleteScript", manage::delete_script)

            .add_route("/manage/queryTableData", manage::query_table_data)
            .add_route("/manage/insertTableData", manage::insert_table_data)
            .add_route("/manage/modifyTableData", manage::modify_table_data)
            .add_route("/manage/removeTableData", manage::remove_table_data)

            .add_route("/manage/runQuery", manage::run_query)
            .add_route("/manage/runScript", manage::run_script)

            .add_route("/users/login", users::login)
            .add_route("/users/refresh", users::refresh)
            .add_route("/users/logout", users::logout)
            .add_route("/users/getAllUsers", users::get_all_users)

            .add_route("/users/addUser", users::add_user)
            .add_route("/users/removeUser", users::remove_user)
            .add_route("/users/inviteUser", users::invite_user)
            .add_route("/users/setupUser", users::setup_user)
            .add_route("/users/setUserPassword", users::set_user_password)

            .add_route("/users/addRole", users::add_role)
            .add_route("/users/removeRole", users::remove_role)
            .add_route("/users/getAllRoles", users::get_all_roles)

            .add_route("/users/attachPermissionForRole", users::attach_permission_for_role)
            .add_route("/users/detachPermissionForRole", users::detach_permission_for_role)

            .add_route("/users/attachRoleForUser", users::attach_role_for_user)
            .add_route("/users/detachRoleForUser", users::detach_role_for_user)
    }
}
