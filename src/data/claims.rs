#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthClaims {
    pub iss: String,
    pub sub: i64, // == user_id
    pub iat: i64,
    pub exp: i64,
    pub username: String,
    pub is_admin: bool,
    pub role: Option<String>, //the default role that the user is interacting with
}

impl AuthClaims {
    pub fn get_user_id(&self) -> i64 {
        self.sub
    }

    pub fn get_username(&self) -> String {
        self.username.to_owned()
    }

    pub fn get_role(&self) -> Option<String> {
        self.role.to_owned()
    }

    pub fn is_user_admin(&self) -> bool {
        self.is_admin
    }
}