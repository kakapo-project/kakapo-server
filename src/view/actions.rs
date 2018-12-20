

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;

use data::api;

/* TODO: put in actions */
pub struct _State {
    database: String,
    user: String,
}

pub struct _Error {

}

pub trait Action {
    type Return;
    fn call(&self, state: _State) -> Result<Self::Return, _Error>;
}

#[derive(Deserialize, Debug, Message)]
#[serde(rename_all = "camelCase")]
#[rtype(result="Result<serde_json::Value, api::Error>")]
pub struct GetTablesAction {
    #[serde(default)]
    pub detailed: bool,
}

impl Action for GetTablesAction {
    type Return = serde_json::Value;
    fn call(&self, state: _State) -> Result<Self::Return, _Error> {
        Ok(serde_json::to_value(&json!({ "success": "get table procedure" })).unwrap())
    }

}

/* END */