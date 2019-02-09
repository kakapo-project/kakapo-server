
use data::auth::User;
use data::auth::InvitationToken;
use data::auth::Invitation;

use data::auth::NewUser;
use model::auth::error::UserManagementError;
use data::auth::Role;
use model::auth::permissions::Permission;
use std::fmt::Debug;
use model::state::GetSecrets;
use argonautica::Hasher;
use metastore::dbdata;
use data::schema;

use diesel::prelude::*;
use diesel;
use diesel::result::Error as DbError;
use diesel::result::DatabaseErrorKind as DbErrKind;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::io::Cursor;
use model::auth::tokens::Token;
use std::marker::PhantomData;
use connection::executor::Conn;
use model::state::StateFunctions;
use model::state::ActionState;

pub struct Auth<'a> {
    conn: &'a Conn,
    password_secret: String,
}

impl<'a> Auth<'a> {
    pub fn new(conn: &'a Conn, password_secret: String) -> Self {
        Self { conn, password_secret }
    }
}

pub trait AuthFunctions {
    fn authenticate(&self, user_identifier: &str, password: &str) -> Result<bool, UserManagementError>;
    fn add_user(&self, user: &NewUser) -> Result<User, UserManagementError>;
    fn remove_user(&self, user_identifier: &str) -> Result<User, UserManagementError>;

    fn create_user_token(&self, email: &str) -> Result<InvitationToken, UserManagementError>;
    //TODO: all modifications
    fn modify_user_password(&self, user_identifier: &str, password: &str) -> Result<User, UserManagementError>;
    fn get_all_users(&self) -> Result<Vec<User>, UserManagementError>;

    fn add_role(&self, rolename: &Role) -> Result<Role, UserManagementError>;
    fn remove_role(&self, rolename: &str) -> Result<Role, UserManagementError>;
    fn get_all_roles(&self) -> Result<Vec<Role>, UserManagementError>;

    fn attach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;
    fn detach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;

    fn attach_role_for_user(&self, role: &Role, user_identifier: &str) -> Result<User, UserManagementError>;
    fn detach_role_for_user(&self, role: &Role, user_identifier: &str) -> Result<User, UserManagementError>;
}

impl<'a> AuthFunctions for Auth<'a> {
    fn authenticate(&self, user_identifier: &str, password: &str) -> Result<bool, UserManagementError> {
        unimplemented!()
    }

    fn add_user(&self, user: &NewUser) -> Result<User, UserManagementError> {
        let mut hasher = Hasher::default();
        let hashed_pass = hasher
            .with_password(&user.password)
            .with_secret_key(&self.password_secret)
            .hash()
            .or_else(|err| {
                error!("Could not hash user password with argno2 [{:?}]", &user.username);
                Err(UserManagementError::HashError(err))
            })?;

        let raw_user = dbdata::NewRawUser {
            username: user.username.to_owned(),
            email: user.email.to_owned(),
            display_name: user.display_name.to_owned()
                .unwrap_or_else(|| user.username.to_owned()),
            password: hashed_pass,
        };

        let result = diesel::insert_into(schema::user::table)
            .values(&raw_user)
            .get_result::<dbdata::RawUser>(self.conn);

        match result {
            Ok(user) => {
                info!("inserted new user {}[{}] {}", &user.username, &user.display_name, &user.email);

                Ok(User {
                    username: user.username,
                    email: user.email,
                    display_name: user.display_name,
                })
            }
            Err(err) => {
                println!("Could not insert new user {}[{}] {} err: {:?}", &raw_user.username, &raw_user.display_name, &raw_user.email, &err);

                match err {
                    DbError::DatabaseError(DbErrKind::UniqueViolation, _) => Err(UserManagementError::AlreadyExists),
                    _ => Err(UserManagementError::InternalError(err.to_string())),
                }
            }
        }
    }

    fn remove_user(&self, user_identifier: &str) -> Result<User, UserManagementError> {
        info!("deleting user: {:?}", &user_identifier);
        /* FIXME: .or_filter not working for diesel */
        /*
        let result = diesel::delete(schema::user::table)
            .filter(schema::user::columns::username.eq(&user_identifier))
            .or_filter(schema::user::columns::email.eq(&user_identifier))
            .execute(state.get_conn());
        */
        let result = diesel::sql_query(r#"DELETE FROM "user" WHERE "username" = $1 OR "email" = $2 RETURNING *;"#)
            .bind::<diesel::sql_types::Text, _>(user_identifier)
            .bind::<diesel::sql_types::Text, _>(user_identifier)
            .get_result::<dbdata::RawUser>(self.conn);

        match result {
            Ok(user) => {
                info!("inserted new user {}[{}] {}", &user.username, &user.display_name, &user.email);

                Ok(User {
                    username: user.username,
                    email: user.email,
                    display_name: user.display_name,
                })
            }
            Err(err) => {
                info!("Could not delete user: {:?}", &user_identifier);

                match err {
                    DbError::NotFound => Err(UserManagementError::NotFound),
                    _ => Err(UserManagementError::InternalError(err.to_string())),
                }
            }
        }
    }

    fn create_user_token(&self, email: &str) -> Result<InvitationToken, UserManagementError> {
        info!("Creating token for: {}", email);
        let token = Token::new()
            .map_err(|err| UserManagementError::InternalError(err.to_string()))?;

        let delete_result = diesel::delete(schema::invitation::table)
            .filter(schema::invitation::columns::email.eq(email))
            .execute(&*self.conn);
        if delete_result.is_ok() {
            warn!("Old data exists for {}, pushing that row out", email);
        }

        let token_result = diesel::insert_into(schema::invitation::table)
            .values(dbdata::NewRawInvitation::new(email.to_string(), token.as_string()))
            .get_result::<dbdata::RawInvitation>(self.conn)
            .map_err(|err| {
                error!("Encountered error: {:?}", &err);
                UserManagementError::InternalError(err.to_string())
            })?;

        info!("created token for {:?}[{:?}]", token_result.email, token_result.invitation_id);
        let token = InvitationToken {
            email: token_result.email,
            token: token_result.token,
            expires_at: token_result.expires_at,
        };

        Ok(token)
    }

    fn modify_user_password(&self, user_identifier: &str, password: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }

    fn get_all_users(&self) -> Result<Vec<User>, UserManagementError> {
        unimplemented!()
    }

    fn add_role(&self, rolename: &Role) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn remove_role(&self, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn get_all_roles(&self) -> Result<Vec<Role>, UserManagementError> {
        unimplemented!()
    }

    fn attach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn detach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }

    fn attach_role_for_user(&self, role: &Role, user_identifier: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }
    fn detach_role_for_user(&self, role: &Role, user_identifier: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }
}