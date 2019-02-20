use actix_web::ws;
use pubsub::WsClientSession;
use view::procedure::ProcedureBuilder;
use AppStateLike;
use model::actions::Action;
use view::routes::manage;

pub trait CallAction<S> {
    fn call<PB, A>(&mut self, procedure_builder: PB, ctx: &mut ws::WebsocketContext<WsClientSession<S>, S>)
        where
            PB: ProcedureBuilder<S, serde_json::Value, serde_json::Value, A> + Clone + 'static,
            S: AppStateLike + 'static,
            A: Action + 'static;

    fn error(&mut self, ctx: &mut ws::WebsocketContext<WsClientSession<S>, S>)
        where
            S: AppStateLike + 'static;
}

pub fn call_procedure<CB, S>(procedure: &str, cb: &mut CB, ctx: &mut ws::WebsocketContext<WsClientSession<S>, S>)
    where
        S: AppStateLike + 'static,
        CB: CallAction<S>,
{
    //TODO: put this in a macro, we are using this in the routes as well
    match procedure {
        "getAllTables" => cb.call(manage::get_all_tables, ctx),
        "getAllQueries" => cb.call(manage::get_all_queries, ctx),
        "getAllScripts" => cb.call(manage::get_all_scripts, ctx),

        "getTable" => cb.call(manage::get_table, ctx),
        "getQuery" => cb.call(manage::get_query, ctx),
        "getScript" => cb.call(manage::get_script, ctx),

        "createTable" => cb.call(manage::create_table, ctx),
        "createQuery" => cb.call(manage::create_query, ctx),
        "createScript" => cb.call(manage::create_script, ctx),

        "updateTable" => cb.call(manage::update_table, ctx),
        "updateQuery" => cb.call(manage::update_query, ctx),
        "updateScript" => cb.call(manage::update_script, ctx),

        "deleteTable" => cb.call(manage::delete_table, ctx),
        "deleteQuery" => cb.call(manage::delete_query, ctx),
        "deleteScript" => cb.call(manage::delete_script, ctx),

        "queryTableData" => cb.call(manage::query_table_data, ctx),
        "insertTableData" => cb.call(manage::insert_table_data, ctx),
        "modifyTableData" => cb.call(manage::modify_table_data, ctx),
        "removeTableData" => cb.call(manage::remove_table_data, ctx),

        "runQuery" => cb.call(manage::run_query, ctx),
        "runScript" => cb.call(manage::run_script, ctx),

        _ => cb.error(ctx),
    }

}