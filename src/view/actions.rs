

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use data::api;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;

/* TODO: put in actions */
/*pub struct _State {
    database: String,
    user: String,
}
*/
type _State = PooledConnection<ConnectionManager<PgConnection>>;
type _Error = data::api::Error;

pub trait Action {
    type Return;
    fn call(&self, state: &_State) -> Self::Return;
}

#[derive(Deserialize, Debug, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTablesAction {
    #[serde(default)]
    pub detailed: bool,
}

impl Action for GetTablesAction {
    type Return = Result<serde_json::Value, _Error>;
    fn call(&self, state: &_State) -> Self::Return {
        Ok(serde_json::to_value(&json!({ "success": "get table procedure" })).unwrap())
    }

}

/* END */