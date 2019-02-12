use model::state::error::UserManagementError;
use data::permissions::Permission;

//TODO: why using RawPermission here?
pub trait PermissionStoreFunctions {
    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<Permission>, UserManagementError>;
    fn get_all_permissions(&self) -> Result<Vec<Permission>, UserManagementError>;
}
