use state::error::UserManagementError;
use data::auth::NewUser;
use data::auth::InvitationToken;
use data::auth::User;
use data::auth::UserInfo;
use data::auth::Role;
use data::permissions::Permission;

pub trait UserManagementOps {
    fn get_user(&self, user_identifier: &str, password: &str) -> Result<UserInfo, UserManagementError>;
    fn add_user(&self, user: &NewUser) -> Result<User, UserManagementError>;
    fn remove_user(&self, user_identifier: &str) -> Result<User, UserManagementError>;

    fn create_user_token(&self, email: &str) -> Result<InvitationToken, UserManagementError>;
    //TODO: all modifications
    fn modify_user_password(&self, user_identifier: &str, password: &str) -> Result<User, UserManagementError>;
    fn get_all_users(&self) -> Result<Vec<User>, UserManagementError>;

    fn add_role(&self, rolename: &Role) -> Result<Role, UserManagementError>;
    fn rename_role(&self, oldname: &str, newname: &str) -> Result<Role, UserManagementError>;
    fn remove_role(&self, name: &str) -> Result<Role, UserManagementError>;
    fn get_all_roles(&self) -> Result<Vec<Role>, UserManagementError>;

    fn add_permission(&self, permission: &Permission) -> Result<Permission, UserManagementError>;
    fn rename_permission(&self, old_permission: &Permission, new_permission: &Permission) -> Result<Permission, UserManagementError>;
    fn remove_permission(&self, permission: &Permission) -> Result<Permission, UserManagementError>;
    //get_all_permission + get_permissions_for_user in the permission_store

    fn attach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;
    fn detach_permission_for_role(&self, permission: &Permission, rolename: &str) -> Result<Role, UserManagementError>;

    fn attach_role_for_user(&self, rolename: &str, user_identifier: &str) -> Result<User, UserManagementError>;
    fn detach_role_for_user(&self, rolename: &str, user_identifier: &str) -> Result<User, UserManagementError>;
}