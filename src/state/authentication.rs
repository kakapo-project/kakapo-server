use data::auth::NewUser;
use data::auth::User;
use data::auth::SessionToken;
use data::auth::UserInfo;

use state::error::UserManagementError;

pub trait AuthenticationOps {
    fn verify_password(&self, hashed_password: &str, raw_password: &str) -> Result<bool, UserManagementError>;

    fn hash_password(&self, raw_password: &str) -> Result<String, UserManagementError>;

    fn create_session(&self, user: UserInfo) -> Result<SessionToken, UserManagementError>;
}