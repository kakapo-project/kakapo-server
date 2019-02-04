
pub mod permissions;
pub mod error;
pub mod auth_store;
use data::auth::User;
use data::auth::NewUser;
use model::auth::error::UserManagementError;
use model::state::GetConnection;
use data::auth::Role;
use model::auth::permissions::Permission;
use model::auth::permissions::GetUserInfo;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Auth;
pub trait AuthFunctions<S>
    where
        Self: Send + Debug,
        S: GetConnection + GetUserInfo,
{
    fn authenticate(state: &S, user_identifier: &str, password: &str) -> Result<bool, UserManagementError>;
    fn add_user(state: &S, user: &NewUser) -> Result<User, UserManagementError>;
    fn remove_user(state: &S, user_identifier: &str) -> Result<User, UserManagementError>;
    fn get_all_users(state: &S) -> Result<Vec<User>, UserManagementError>;

    fn add_role(state: &S, rolename: &Role) -> Result<Role, UserManagementError>;
    fn remove_role(state: &S, rolename: &str) -> Result<Role, UserManagementError>;
    fn get_all_roles(state: &S) -> Result<Vec<Role>, UserManagementError>;

    fn invite_user(state: &S, email: &str) -> Result<String, UserManagementError>;

    fn modify_user_password(state: &S, user_identifier: &str, password: &str) -> Result<User, UserManagementError>;

    fn attach_permission_for_role(state: &S, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;
    fn detach_permission_for_role(state: &S, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;

    fn attach_role_for_user(state: &S, role: &Role, user_identifier: &str) -> Result<User, UserManagementError>;
    fn detach_role_for_user(state: &S, role: &Role, user_identifier: &str) -> Result<User, UserManagementError>;
}

impl<S> AuthFunctions<S> for Auth
    where
        S: GetConnection + GetUserInfo,
{
    fn authenticate(state: &S, user_identifier: &str, password: &str) -> Result<bool, UserManagementError> {
        unimplemented!()
    }
    fn add_user(state: &S, user: &NewUser) -> Result<User, UserManagementError> {
        unimplemented!()
    }
    fn remove_user(state: &S, user_identifier: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }
    fn get_all_users(state: &S) -> Result<Vec<User>, UserManagementError> {
        unimplemented!()
    }

    fn add_role(state: &S, rolename: &Role) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn remove_role(state: &S, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn get_all_roles(state: &S) -> Result<Vec<Role>, UserManagementError> {
        unimplemented!()
    }

    fn invite_user(state: &S, email: &str) -> Result<String, UserManagementError> {
        unimplemented!()
    }

    fn modify_user_password(state: &S, user_identifier: &str, password: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }

    fn attach_permission_for_role(state: &S, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }
    fn detach_permission_for_role(state: &S, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError> {
        unimplemented!()
    }

    fn attach_role_for_user(state: &S, role: &Role, user_identifier: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }
    fn detach_role_for_user(state: &S, role: &Role, user_identifier: &str) -> Result<User, UserManagementError> {
        unimplemented!()
    }
}