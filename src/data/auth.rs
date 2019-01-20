
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    email: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    name: String,
}
