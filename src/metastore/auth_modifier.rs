
use std::fmt::Debug;
use std::io::Cursor;
use std::marker::PhantomData;

use byteorder::BigEndian;
use byteorder::ReadBytesExt;

use argonautica::Hasher;
use argonautica::Verifier;

use diesel::prelude::*;
use diesel;
use diesel::result::Error as DbError;
use diesel::result::DatabaseErrorKind as DbErrKind;

use data::auth::User;
use data::auth::InvitationToken;
use data::auth::Invitation;
use data::auth::NewUser;
use data::auth::Role;
use data::schema;

use connection::executor::Conn;

use model::auth::error::UserManagementError;
use data::permissions::Permission;
use model::state::GetSecrets;
use model::auth::tokens::Token;
use model::state::StateFunctions;
use model::state::ActionState;

use metastore::dbdata;
use chrono::Utc;


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
        debug!("Authenticating user: {:?}", user_identifier);
        let user = schema::user::table
            .filter(schema::user::columns::username.eq(&user_identifier))
            .or_filter(schema::user::columns::email.eq(&user_identifier))
            .get_result::<dbdata::RawUser>(self.conn)
            .map_err(|err| {
                info!("Could not find user: {:?}", &user_identifier);
                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        let time = Utc::now();
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(&user.password)
            .with_password(password)
            .with_secret_key(&self.password_secret)
            .verify()
            .map_err(|err| {
                error!("Could not verify user password with argon2 [{:?}]", &user.username);
                UserManagementError::HashError(err)
            })?;
        println!("Verifying user took: {:?}", Utc::now() - time);

        if is_valid {
            info!("Password authentication passed for {:?}", &user.username);
        } else {
            info!("Password authentication failed for {:?}", &user.username);
        }

        Ok(is_valid)
    }

    fn add_user(&self, user: &NewUser) -> Result<User, UserManagementError> {
        let time = Utc::now();
        let mut hasher = Hasher::default();
        let hashed_pass = hasher
            .with_password(&user.password)
            .with_secret_key(&self.password_secret)
            .hash()
            .map_err(|err| {
                error!("Could not hash user password with argon2 [{:?}]", &user.username);
                UserManagementError::HashError(err)
            })?;

        println!("Hashing password took: {:?}", Utc::now() - time);

        let raw_user = dbdata::NewRawUser {
            username: user.username.to_owned(),
            email: user.email.to_owned(),
            display_name: user.display_name.to_owned()
                .unwrap_or_else(|| user.username.to_owned()),
            password: hashed_pass,
        };

        let user = diesel::insert_into(schema::user::table)
            .values(&raw_user)
            .get_result::<dbdata::RawUser>(self.conn)
            .map_err(|err| {
                println!("Could not insert new user {}[{}] {} err: {:?}", &raw_user.username, &raw_user.display_name, &raw_user.email, &err);

                match err {
                    DbError::DatabaseError(DbErrKind::UniqueViolation, _) => UserManagementError::AlreadyExists,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        info!("inserted new user {}[{}] {}", &user.username, &user.display_name, &user.email);
        Ok(User {
            username: user.username,
            email: user.email,
            display_name: user.display_name,
        })

    }

    fn remove_user(&self, user_identifier: &str) -> Result<User, UserManagementError> {
        info!("deleting user: {:?}", &user_identifier);
        /* FIXME: .or_filter not working for diesel */
        /*
        let result = diesel::delete(schema::user::table)
            .filter(schema::user::columns::username.eq(&user_identifier))
            .or_filter(schema::user::columns::email.eq(&user_identifier))
        */
        let user = diesel::sql_query(r#"DELETE FROM "user" WHERE "username" = $1 OR "email" = $2 RETURNING *;"#)
            .bind::<diesel::sql_types::Text, _>(user_identifier)
            .bind::<diesel::sql_types::Text, _>(user_identifier)
            .get_result::<dbdata::RawUser>(self.conn)
            .map_err(|err| {
                info!("Could not delete user: {:?}", &user_identifier);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        info!("inserted new user {}[{}] {}", &user.username, &user.display_name, &user.email);
        Ok(User {
            username: user.username,
            email: user.email,
            display_name: user.display_name,
        })
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

    //TODO: check with old password
    fn modify_user_password(&self, user_identifier: &str, password: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }

    fn get_all_users(&self) -> Result<Vec<User>, UserManagementError> {
        unimplemented!()
    }

    fn add_role(&self, rolename: &Role) -> Result<Role, UserManagementError> {
        info!("Adding new role {:?}", &rolename);
        let raw_role = dbdata::NewRawRole::new(
            rolename.name.to_owned(),
            rolename.description.to_owned().unwrap_or_default());
        let role = diesel::insert_into(schema::role::table)
            .values(&raw_role)
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not insert new role {} err: {:?}", &raw_role.name, &err);

                match err {
                    DbError::DatabaseError(DbErrKind::UniqueViolation, _) => UserManagementError::AlreadyExists,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        info!("inserted new role {}", &role.name);
        Ok(Role {
            name: role.name,
            description: Some(role.description),
        })
    }
    fn remove_role(&self, rolename: &str) -> Result<Role, UserManagementError> {
        info!("Deleting role {:?}", &rolename);
        let role = diesel::delete(schema::role::table)
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not insert new role {} err: {:?}", &rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        info!("deleting role {}", &rolename);
        Ok(Role {
            name: role.name,
            description: Some(role.description),
        })
    }
    fn get_all_roles(&self) -> Result<Vec<Role>, UserManagementError> {

        info!("listing roles");
        let raw_roles = schema::role::table
            .get_results::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not list all roles err: {:?}", &err);
                UserManagementError::InternalError(err.to_string())
            })?;

        let roles = raw_roles
            .into_iter()
            .map(|role| {
                Role {
                    name: role.name,
                    description: Some(role.description),
                }
            })
            .collect();

        Ok(roles)
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