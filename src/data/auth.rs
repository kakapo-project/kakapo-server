
pub use model::auth::permissions::Permission;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
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
    name: String,
}

impl Role {
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}
