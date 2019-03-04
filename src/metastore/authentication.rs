
use std::collections::HashSet;
use std::iter::FromIterator;

use argonautica::Hasher;
use argonautica::Verifier;

use chrono::Utc;
use chrono::Duration;
use chrono::NaiveDateTime;

use diesel::RunQueryDsl;

use data::claims::AuthClaims;
use data::permissions::Permission;
use data::auth::NewUser;
use data::auth::User;
use data::auth::UserInfo;
use data::auth::SessionToken;

use state::authentication::AuthenticationOps;
use state::user_management::UserManagementOps;
use state::Authentication;
use state::error::UserManagementError;

use metastore::schema;
use auth::tokens::Token;
use metastore::dbdata;
use metastore;
use diesel::prelude::*;
use diesel::result::Error;


impl<'a> AuthenticationOps for Authentication<'a> {

    fn verify_password(&self, hashed_password: &str, raw_password: &str) -> Result<bool, UserManagementError> {
        let time = Utc::now();
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(hashed_password)
            .with_password(raw_password)
            .with_secret_key(&self.password_secret)
            .verify()
            .map_err(|err| {
                error!("Could not verify user password with argon2");
                UserManagementError::HashError(err.to_string())
            })?;
        debug!("Verifying user took: {:?}", Utc::now() - time);

        Ok(is_valid)
    }

    fn hash_password(&self, raw_password: &str) -> Result<String, UserManagementError> {
        let time = Utc::now();
        let mut hasher = Hasher::default();
        let hashed_pass = hasher
            .with_password(raw_password)
            .with_secret_key(&self.password_secret)
            .hash()
            .map_err(|err| {
                error!("Could not hash user password with argon2");
                UserManagementError::HashError(err.to_string())
            })?;

        debug!("Hashing password took: {:?}", Utc::now() - time);

        Ok(hashed_pass)
    }

    fn create_session(&self, user: UserInfo) -> Result<SessionToken, UserManagementError> {

        let refresh_token = Token::new()
            .map_err(|err| {
                error!("could not create a random refresh token");
                UserManagementError::Unknown
            })?;
        let now = Utc::now();
        let duration = self.jwt_duration;
        let refresh_duration = self.jwt_refresh_duration;

        let token_string = refresh_token.as_string();

        let session_token = dbdata::NewRawSessionToken {
            user_id: user.user_id,
            token: token_string,
            created_at: NaiveDateTime::from_timestamp(now.timestamp(), 0),
            expires_at: NaiveDateTime::from_timestamp((now + Duration::seconds(refresh_duration)).timestamp(), 0),
        };

        let session = diesel::insert_into(schema::session::table)
            .values(&session_token)
            .get_result::<dbdata::RawSessionToken>(self.conn)
            .map_err(|err| {
                error!("Could not create session err: {:?}", &err);

                UserManagementError::InternalError(err.to_string())
            })?;


        self.build_jwt_token(now, user, session_token.token)
    }

    fn refresh_session(&self, token_string: String) -> Result<SessionToken, UserManagementError> {

        let now = Utc::now();
        let naive_datetime_now = NaiveDateTime::from_timestamp(now.timestamp(), 0);

        let token = schema::session::table
            .filter(schema::session::columns::token.eq(&token_string))
            .filter(schema::session::columns::expires_at.gt(naive_datetime_now))
            .get_result::<dbdata::RawSessionToken>(self.conn)
            .map_err(|err| match err {
                Error::NotFound => {
                    warn!("Token not found for {:?}", &token_string);
                    UserManagementError::NotFound
                },
                _ => {
                    error!("Could not get token value: {:?}", &err);
                    UserManagementError::InternalError(err.to_string())
                },
            })?;

        let user = schema::user::table
            .filter(schema::user::columns::user_id.eq(token.user_id))
            .get_result::<dbdata::RawUser>(self.conn)
            .map_err(|err| {
                error!("Could not get user: {:?}", &err);
                UserManagementError::InternalError(err.to_string())
            })?;


        let duration = self.jwt_duration;

        let user = UserInfo {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
        };

        self.build_jwt_token(now, user, token_string)
    }

    fn delete_session(&self, user_id: i64) -> Result<(), UserManagementError> {
        let _tokens = diesel::delete(schema::session::table)
            .filter(schema::session::columns::user_id.eq(&user_id))
            .get_results::<dbdata::RawSessionToken>(self.conn)
            .map_err(|err| {
                error!("Could not get token value: {:?}", &err);
                UserManagementError::InternalError(err.to_string())
            })?;

        info!("All tokens for user id {} removed", user_id);

        Ok(())
    }

}


impl<'a> Authentication<'a>  {
    fn build_jwt_token(&self, now: chrono::DateTime<Utc>, user: UserInfo, refresh_token_string: String) -> Result<SessionToken, UserManagementError> {
        let duration = self.jwt_duration;
        let refresh_duration = self.jwt_refresh_duration;

        let is_admin = user.user_id == metastore::ADMIN_USER_ID;
        let claims = AuthClaims {
            iss: self.jwt_issuer.to_owned(),
            sub: user.user_id,
            iat: now.timestamp(),
            exp: (now + Duration::seconds(duration)).timestamp(),
            username: user.username,
            is_admin: is_admin,
            role: None, //TODO: make sure the role is here
        };

        let jwt = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, self.jwt_secret.as_ref())
            .map_err(|err| UserManagementError::AuthenticationError(err.to_string()))?;

        Ok(SessionToken::Bearer {
            access_token: jwt,
            expires_in: duration as u32,
            refresh_token: refresh_token_string,
        })
    }
}