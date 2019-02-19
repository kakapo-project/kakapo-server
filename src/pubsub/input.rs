
use serde_json;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum WsInputData {
    #[serde(rename_all = "camelCase")]
    Call {
        procedure: String,
        params: serde_json::Value,
        data: serde_json::Value,
    },
}