

use jsonwebtoken::{decode, encode, Header, Validation};
use bcrypt::verify;

use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};


use super::dbdata::User;
use super::schema::user_account;
use chrono::{Duration, Local};

/*

pub fn verify_token(token: Token) -> bool {

}

pub fn renew_token(token: Token) -> Token {

}


pub fn token_has_permission(token: Token /*, permission: Permission*/) -> bool {
    if verify_token(token) == false {
        return false;
    }
    true //TODO: implement authorization
}

*/

pub enum AuthError {
    AuthenticationFailure,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
    username: String,
    email: String,
    roles: Vec<String>,
}

impl Token {

    fn build_token(user: User) -> Self {
        Token {
            iss: "KakapoAuth".to_string(),
            sub: "KakapoAuth".to_string(),
            username: user.username,
            roles: vec![],
            email: user.email,
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }

    pub fn create_new(
        conn: &PooledConnection<ConnectionManager<PgConnection>>,
        username: String,
        password: String,
    ) -> Result<Self, AuthError> {

        let mut items = user_account::table
            .filter(user_account::email.eq(&username))
            .or_filter(user_account::username.eq(&username))
            .load::<User>(conn)
            .or_else(|err| Err(AuthError::AuthenticationFailure))?;

        if let Some(user) = items.pop() {
            match verify(&password, &user.password) {
                Ok(matching) => if matching {
                    return Ok(Self::build_token(user));
                },
                Err(_) => (),
            }
        }
        Err(AuthError::AuthenticationFailure)
    }
}

pub fn get_current_user() -> i64 {
    return 1;
}