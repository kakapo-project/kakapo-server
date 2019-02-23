use actix_web::ws;
use broker::WsClientSession;
use view::procedure::ProcedureBuilder;
use AppStateLike;
use model::actions::Action;
use view::routes::manage;
use view::routes::pubsub;

pub struct CallParams<'a, S>
    where
        S: AppStateLike + 'static,
{
    pub data: serde_json::Value,
    pub params: serde_json::Value,
    pub ctx: &'a mut ws::WebsocketContext<WsClientSession<S>, S>,
}

pub trait CallAction<S> {
    fn call<'a, PB, A>(&mut self, procedure_builder: PB, call_params: &mut CallParams<'a, S>)
        where
            PB: ProcedureBuilder<S, serde_json::Value, serde_json::Value, A> + Clone + 'static,
            S: AppStateLike + 'static,
            A: Action + 'static;

    fn error<'a>(&mut self, call_params: &mut CallParams<'a, S>)
        where
            S: AppStateLike + 'static;
}

pub fn call_procedure<'a, CB, S>(procedure: &str, cb: &mut CB, call_params: &mut CallParams<'a, S>)
    where
        S: AppStateLike + 'static,
        CB: CallAction<S>,
{
    //TODO: put this in a macro, we are using this in the routes as well
    match procedure {
        "getAllTables" => cb.call(manage::get_all_tables, call_params),
        "getAllQueries" => cb.call(manage::get_all_queries, call_params),
        "getAllScripts" => cb.call(manage::get_all_scripts, call_params),

        "getTable" => cb.call(manage::get_table, call_params),
        "getQuery" => cb.call(manage::get_query, call_params),
        "getScript" => cb.call(manage::get_script, call_params),

        "createTable" => cb.call(manage::create_table, call_params),
        "createQuery" => cb.call(manage::create_query, call_params),
        "createScript" => cb.call(manage::create_script, call_params),

        "updateTable" => cb.call(manage::update_table, call_params),
        "updateQuery" => cb.call(manage::update_query, call_params),
        "updateScript" => cb.call(manage::update_script, call_params),

        "deleteTable" => cb.call(manage::delete_table, call_params),
        "deleteQuery" => cb.call(manage::delete_query, call_params),
        "deleteScript" => cb.call(manage::delete_script, call_params),

        "queryTableData" => cb.call(manage::query_table_data, call_params),
        "insertTableData" => cb.call(manage::insert_table_data, call_params),
        "modifyTableData" => cb.call(manage::modify_table_data, call_params),
        "removeTableData" => cb.call(manage::remove_table_data, call_params),

        "runQuery" => cb.call(manage::run_query, call_params),
        "runScript" => cb.call(manage::run_script, call_params),

        "subscribeTo" => cb.call(pubsub::subscribe_to, call_params),
        "unsubscribeFrom" => cb.call(pubsub::unsubscribe_from, call_params),
        "getSubscribers" => cb.call(pubsub::get_subscribers, call_params),
        "getMessages" => cb.call(pubsub::get_messages, call_params),

        _ => cb.error(call_params),
    }

}