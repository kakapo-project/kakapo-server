
use diesel::prelude::*;
use diesel;
use diesel::result::Error as DbError;
use diesel::result::DatabaseErrorKind as DbErrKind;

use chrono::Utc;
use serde_json;

use auth::tokens::Token;

use data::auth::InvitationToken;
use data::auth::Role;
use data::permissions::Permission;
use data::auth::NewUser;
use data::auth::UserInfo;
use data::auth::User;
use metastore::schema;

use metastore::dbdata;
use connection::executor::Conn;

use state::error::UserManagementError;
use state::authentication::AuthenticationOps;
use state::user_management::UserManagementOps;
use state::UserManagement;


impl<'a> UserManagementOps for UserManagement<'a> {
    fn get_user(&self, user_identifier: &str, password: &str) -> Result<UserInfo, UserManagementError> {
        debug!("Authenticating user: {:?}", user_identifier);
        let user = schema::user::table
            .filter(schema::user::columns::username.eq(&user_identifier))
            .or_filter(schema::user::columns::email.eq(&user_identifier))
            .get_result::<dbdata::RawUser>(self.conn)
            .map_err(|err| {
                info!("Could not find user: {:?}", &user_identifier);
                //TODO: a timining attack possible here?
                match err {
                    DbError::NotFound => UserManagementError::Unauthorized,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        let is_valid = self
            .authentication
            .verify_password(&user.password, password)?;

        if is_valid {
            info!("Password authentication passed for {:?}", &user.username);
            Ok(UserInfo {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                display_name: user.display_name,
            })
        } else {
            info!("Password authentication failed for {:?}", &user.username);
            Err(UserManagementError::Unauthorized)
        }
    }

    fn add_user(&self, user: &NewUser) -> Result<User, UserManagementError> {
        info!("Creating new user {:?}", &user);

        //TODO: test password complexity
        let hashed_pass = self
            .authentication
            .hash_password(&user.password)?;

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
                error!("Could not insert new user {}[{}] {} err: {:?}", &raw_user.username, &raw_user.display_name, &raw_user.email, &err);

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

    fn rename_role(&self, oldname: &'_ str, newname: &'_ str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }

    fn remove_role(&self, rolename: &str) -> Result<Role, UserManagementError> {
        info!("Deleting role {:?}", &rolename);
        let role = diesel::delete(schema::role::table)
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                error!("Could not insert new role {} err: {:?}", &rolename, &err);

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
                error!("Could not list all roles err: {:?}", &err);
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

    fn add_permission(&self, permission: &Permission) -> Result<Permission, UserManagementError> {
        let permission_json = serde_json::to_value(permission)
            .map_err(|err| {
                error!("Could not serialize value {:?} error: {:?}", &permission, &err);
                UserManagementError::Unknown
            })?;

        let permission_value = dbdata::NewRawPermission {
            data: permission_json,
        };

        let raw_permission = diesel::insert_into(schema::permission::table)
            .values(&permission_value)
            .get_result::<dbdata::RawPermission>(self.conn)
            .map_err(|err| {
                println!("Could not create permission err: {:?}", &err);

                UserManagementError::InternalError(err.to_string())
            })?;

        let permission: Permission = serde_json::from_value(raw_permission.data)
            .map_err(|err| {
                error!("Could not deserialize error: {:?}", &err);
                UserManagementError::Unknown
            })?;

        Ok(permission)
    }

    fn rename_permission(&self, old_permission: &Permission, new_permission: &Permission) -> Result<Permission, UserManagementError> {
        unimplemented!()
    }

    fn remove_permission(&self, permission: &Permission) -> Result<Permission, UserManagementError> {
        unimplemented!()
    }

    fn attach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {

        info!("attaching permission [{:?}] for role [{}]", &permission, &rolename);

        //Get the role
        let raw_role = schema::role::table
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not get role {} err: {:?}", rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        //Get the permission
        let raw_permission = get_or_create_permission(self.conn, permission)?;

        let role_permission = (
            schema::role_permission::columns::role_id.eq(raw_role.role_id),
            schema::role_permission::columns::permission_id.eq(raw_permission.permission_id),
        );

        //Attach the role to permission
        //WARNING: the role_permission table doesn't have a unique constraint so duplication is possible, this should probably be an insert or ignore
        let _ = diesel::insert_into(schema::role_permission::table)
            .values(&role_permission)
            .execute(self.conn)
            .map_err(|err| UserManagementError::InternalError(err.to_string()))?;

        info!("Done attaching permission [{:?}] for role [{}]", &permission, &rolename);

        Ok(Role {
            name: raw_role.name,
            description: Some(raw_role.description),
        })
    }
    fn detach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {

        info!("detaching permission [{:?}] for role [{}]", &permission, &rolename);

        //Get the role
        let raw_role = schema::role::table
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not get role {} err: {:?}", rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        //Get the permission
        let permission_json = serde_json::to_value(permission)
            .map_err(|err| {
                error!("Could not serialize value {:?} error: {:?}", &permission, &err);
                UserManagementError::Unknown
            })?;
        let raw_permission = schema::permission::table
            .filter(schema::permission::columns::data.eq(&permission_json))
            .get_result::<dbdata::RawPermission>(self.conn)
            .map_err(|err| {
                println!("Could not get permission {} err: {:?}", rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        //Detach the role to permission
        let _ = diesel::sql_query(r#"DELETE FROM "role_permission" WHERE "role_id" = $1 AND "permission_id" = $2;"#)
            .bind::<diesel::sql_types::BigInt, _>(raw_role.role_id)
            .bind::<diesel::sql_types::BigInt, _>(raw_permission.permission_id)
            .execute(self.conn)
            .map_err(|err| UserManagementError::InternalError(err.to_string()))?;

        info!("Done permission [{:?}] for role [{}]", &permission, &rolename);

        Ok(Role {
            name: raw_role.name,
            description: Some(raw_role.description),
        })
    }

    fn attach_role_for_user(&self, rolename: &str, user_identifier: &str) -> Result<User, UserManagementError> {

        info!("attaching role [{}] for user [{}]", &rolename, &user_identifier);

        //Get the user
        let raw_user = schema::user::table
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

        //Get the role
        let raw_role = schema::role::table
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not get role {} err: {:?}", rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        let user_role = (
            schema::user_role::columns::user_id.eq(raw_user.user_id),
            schema::user_role::columns::role_id.eq(raw_role.role_id),
        );

        //Attach the role to user
        //WARNING: the user_role table doesn't have a unique constraint so duplication is possible, this should probably be an insert or ignore
        let _ = diesel::insert_into(schema::user_role::table)
            .values(&user_role)
            .execute(self.conn)
            .map_err(|err| UserManagementError::InternalError(err.to_string()))?;

        info!("attaching role [{}] for user [{}]", &rolename, &user_identifier);

        Ok(User {
            username: raw_user.username,
            email: raw_user.email,
            display_name: raw_user.display_name,
        })
    }
    fn detach_role_for_user(&self, rolename: &str, user_identifier: &str) -> Result<User, UserManagementError> {

        info!("detaching role [{}] for user [{}]", &rolename, &user_identifier);

        //Get the user
        let raw_user = schema::user::table
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

        //Get the role
        let raw_role = schema::role::table
            .filter(schema::role::columns::name.eq(&rolename))
            .get_result::<dbdata::RawRole>(self.conn)
            .map_err(|err| {
                println!("Could not get role {} err: {:?}", rolename, &err);

                match err {
                    DbError::NotFound => UserManagementError::NotFound,
                    _ => UserManagementError::InternalError(err.to_string()),
                }
            })?;

        //Attach the role to user
        let _ = diesel::sql_query(r#"DELETE FROM "user_role" WHERE "user_id" = $1 AND "role_id" = $2;"#)
            .bind::<diesel::sql_types::BigInt, _>(raw_user.user_id)
            .bind::<diesel::sql_types::BigInt, _>(raw_role.role_id)
            .execute(self.conn)
            .map_err(|err| UserManagementError::InternalError(err.to_string()))?;

        info!("detaching role [{}] for user [{}]", &rolename, &user_identifier);

        Ok(User {
            username: raw_user.username,
            email: raw_user.email,
            display_name: raw_user.display_name,
        })
    }
}

fn get_or_create_permission(conn: &Conn, permission: &Permission) -> Result<dbdata::RawPermission, UserManagementError> {
    let permission_json = serde_json::to_value(permission)
        .map_err(|err| {
            error!("Could not serialize value {:?} error: {:?}", &permission, &err);
            UserManagementError::Unknown
        })?;
    let permission_value = dbdata::NewRawPermission {
        data: permission_json,
    };

    schema::permission::table
        .filter(schema::permission::columns::data.eq(&permission_value.data))
        .get_result::<dbdata::RawPermission>(conn)
        .or_else(|err| match err {
            DbError::NotFound => {
                diesel::insert_into(schema::permission::table)
                    .values(&permission_value)
                    .get_result::<dbdata::RawPermission>(conn)
            },
            _ => Err(err),
        })
        .map_err(|err| {
            println!("Could not get or create err: {:?}", &err);

            UserManagementError::InternalError(err.to_string())
        })
}