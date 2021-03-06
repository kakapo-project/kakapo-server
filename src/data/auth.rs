
use chrono;
use data::claims::AuthClaims;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String, //TODO: don't have all the fields as pub
    pub email: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: i64,
    pub username: String, //TODO: don't have all the fields as pub
    pub email: String,
    pub display_name: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub username: String, //TODO: don't have all the fields as pub
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub name: String,
    pub description: Option<String>,
}

impl Role {
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationToken {
    pub email: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tokenType")]
pub enum SessionToken {
    #[serde(rename_all = "camelCase")]
    Bearer {
        access_token: String, //jwt
        expires_in: u32,
        refresh_token: String,
    }
}
