
use serde_json;
use data::channels::Channels;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum WsInputData {
    #[serde(rename_all = "camelCase")]
    Authenticate {
        token: String,
    },
    #[serde(rename_all = "camelCase")]
    SubscribeTo {
        channel: Channels,
    },
    #[serde(rename_all = "camelCase")]
    UnsubscribeFrom {
        channel: Channels,
    },
    #[serde(rename_all = "camelCase")]
    ListSubscribers {
        channel: Channels,
    },
    #[serde(rename_all = "camelCase")]
    Call {
        procedure: String,
        params: serde_json::Value,
        data: serde_json::Value,
    },

}