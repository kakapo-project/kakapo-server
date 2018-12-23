

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use data::api;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;

use data;

type State = PooledConnection<ConnectionManager<PgConnection>>;
type Error = data::api::Error;

pub type ActionResult = Result<serde_json::Value, Error>;

pub trait Action {
    type Result;
    fn call(&self, state: &State) -> Self::Result;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTablesAction {
    #[serde(default)]
    pub detailed: bool,
}

impl Action for GetTablesAction {
    type Result = ActionResult;
    fn call(&self, state: &State) -> ActionResult {
        Ok(serde_json::to_value(&json!({ "success": "get table procedure" })).unwrap())
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoAction;


impl Action for NoAction {
    type Result = ActionResult;
    fn call(&self, state: &State) -> ActionResult {
        Ok(serde_json::to_value(&json!({ "success": "nothing" })).unwrap())
    }
}