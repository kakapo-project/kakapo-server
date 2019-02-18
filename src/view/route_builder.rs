use std::result::Result;
use std::result::Result::Ok;
use actix_web::test::TestApp;

// current module
use model::actions;

use view::procedure::NoQuery;
use view::extensions::ProcedureExt;
use data;
use actix_web::middleware::cors::CorsBuilder;
use model::actions::Action;
use serde_json::Value;
use serde_json::Error;
use serde_json::from_value;
use connection::AppStateLike;
use AppState;
use view::routes::manage;
use view::routes::users;
use connection::executor::Executor;
use actix_web::dev::Handler;
use view::action_wrapper::ActionWrapper;
use view::procedure::ProcedureBuilder;
use std::fmt::Debug;
use actix_web::FromRequest;
use actix_web::dev::JsonConfig;
use actix_web::Query;
use serde::Serialize;
use actix_web::Json;
use view::procedure::ProcedureBuilderContainer;


type ProcBuilder = ProcedureBuilderContainer<AppState>;

#[derive(Debug, Clone)]
pub struct RouteBuilder {
    procedures: Vec<ProcBuilder>
}

/// Build routes for rpc calls
impl RouteBuilder {

    /// Create an RPC call
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn procedure<JP, QP, A, PB>(&mut self, path: &str, procedure_builder: PB) -> &mut Self
            where
                A: Action + Send + 'static,
                PB: ProcedureBuilder<AppState, JP, QP, A> + Clone + 'static,
                JP: Debug + 'static,
                QP: Debug + 'static,
                Json<JP>: FromRequest<AppState, Config = JsonConfig<AppState>>,
                Query<QP>: FromRequest<AppState>,
    {
        self.procedures.push(procedure_builder.to_container());
        self
    }

    /// Create an RPC call and register for use with websockets
    ///
    /// # Arguments
    /// * `path` - A string representing the url path
    /// * `action` - A string representing the action
    /// * `procedure_builder` - An object extending `ProcedureBuilder` for building a message
    ///
    fn action_procedure<JP, QP, A, PB>(&mut self, path: &str, action: &str, procedure_builder: PB) -> &mut Self
        where
            A: Action + Send + 'static,
            PB: ProcedureBuilder<AppState, JP, QP, A> + Clone + 'static,
            JP: Debug + 'static,
            QP: Debug + 'static,
            Json<JP>: FromRequest<AppState, Config = JsonConfig<AppState>>,
            Query<QP>: FromRequest<AppState>,
    {
        self
    }

    pub fn empty() -> Self {
        RouteBuilder {
            procedures: vec![],
        }
    }

    pub fn build(state: &mut AppState) -> Self {

        RouteBuilder::empty()
            .action_procedure("/manage/getAllTables", "getAllTables", manage::get_all_tables)
            .action_procedure("/manage/getAllQueries", "getAllQueries", manage::get_all_queries)
            .action_procedure("/manage/getAllScripts", "getAllScripts", manage::get_all_scripts)

            .action_procedure("/manage/getTable", "getTable", manage::get_table)
            .action_procedure("/manage/getQuery", "getQuery", manage::get_query)
            .action_procedure("/manage/getScript", "getScript", manage::get_script)

            .action_procedure("/manage/createTable", "createTable", manage::create_table)
            .action_procedure("/manage/createQuery", "createQuery", manage::create_query)
            .action_procedure("/manage/createScript", "createScript", manage::create_script)

            .action_procedure("/manage/updateTable", "updateTable", manage::update_table)
            .action_procedure("/manage/updateQuery", "updateQuery", manage::update_query)
            .action_procedure("/manage/updateScript", "updateScript", manage::update_script)

            .action_procedure("/manage/deleteTable", "deleteTable", manage::delete_table)
            .action_procedure("/manage/deleteQuery", "deleteQuery", manage::delete_query)
            .action_procedure("/manage/deleteScript", "deleteScript", manage::delete_script)

            .action_procedure("/manage/queryTableData", "queryTableData", manage::query_table_data)
            .action_procedure("/manage/insertTableData", "insertTableData", manage::insert_table_data)
            .action_procedure("/manage/modifyTableData", "modifyTableData", manage::modify_table_data)
            .action_procedure("/manage/removeTableData", "removeTableData", manage::remove_table_data)

            .action_procedure("/manage/runQuery", "runQuery", manage::run_query)
            .action_procedure("/manage/runScript", "runScript", manage::run_script)

            .procedure("/users/login", users::login)
            .procedure("/users/refresh", users::refresh)
            .procedure("/users/logout", users::logout)
            .procedure("/users/getAllUsers", users::get_all_users)

            .procedure("/users/addUser", users::add_user)
            .procedure("/users/removeUser", users::remove_user)
            .procedure("/users/inviteUser", users::invite_user)
            .procedure("/users/setupUser", users::setup_user)
            .procedure("/users/setUserPassword", users::set_user_password)

            .procedure("/users/addRole", users::add_role)
            .procedure("/users/removeRole", users::remove_role)
            .procedure("/users/getAllRoles", users::get_all_roles)

            .procedure("/users/attachPermissionForRole", users::attach_permission_for_role)
            .procedure("/users/detachPermissionForRole", users::detach_permission_for_role)

            .procedure("/users/attachRoleForUser", users::attach_role_for_user)
            .procedure("/users/detachRoleForUser", users::detach_role_for_user)
            .to_owned()

    }
}