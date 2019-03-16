

use data;
use data::auth::Invitation;
use data::channels::Channels;
use data::channels::Subscription;

#[derive(Debug, Clone, Serialize)]
pub struct GetAllEntitiesResult<T>(pub Vec<T>);

#[derive(Debug, Clone, Serialize)]
pub struct GetEntityResult<T>(pub T);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum CreateEntityResult<T> {
    Updated {
        old: T,
        new: T,
    },
    Created {
        new: T,
    },
    AlreadyExists {
        existing: T,
        requested: T,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum UpdateEntityResult<T> {
    Updated {
        id: String,
        old: T,
        new: T,
    },
    NotFound {
        id: String,
        requested: T,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "result")]
pub enum DeleteEntityResult<T> {
    Deleted {
        id: String,
        old: T,
    },
    NotFound {
        id: String,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTableDataResult(pub serde_json::Value);

#[derive(Debug, Clone, Serialize)]
pub struct InsertTableDataResult(pub serde_json::Value);

#[derive(Debug, Clone, Serialize)]
pub struct ModifyTableDataResult(pub serde_json::Value);

#[derive(Debug, Clone, Serialize)]
pub struct RemoveTableDataResult(pub serde_json::Value);

#[derive(Debug, Clone, Serialize)]
pub struct RunQueryResult(pub serde_json::Value);

#[derive(Debug, Clone, Serialize)]
pub struct RunScriptResult(pub serde_json::Value);


#[derive(Debug, Clone, Serialize)]
pub struct UserResult(pub data::auth::User);

#[derive(Debug, Clone, Serialize)]
pub struct AllUsersResult(pub Vec<data::auth::User>);

#[derive(Debug, Clone, Serialize)]
pub struct InvitationResult(pub Invitation);

#[derive(Debug, Clone, Serialize)]
pub struct RoleResult(pub data::auth::Role);

#[derive(Debug, Clone, Serialize)]
pub struct AllRolesResult(pub Vec<data::auth::Role>);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SubscriptionResult {
    Subscribed(Subscription),
    Unsubscribed(Subscription),
    UnsubscribedAll,
}