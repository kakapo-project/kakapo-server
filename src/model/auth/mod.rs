use data::claims::AuthClaims;
use data::permissions::Permission;
use metastore::permission_store::PermissionStoreFunctions;
use std::collections::HashSet;
use std::iter::FromIterator;

pub mod error;
pub mod send_mail;
pub mod tokens;


pub struct UserInfo<'a, P> {
    pub permission_store: P,
    pub claims: &'a Option<AuthClaims>,
}

pub trait GetUserInfo {
    fn user_id(&self) -> Option<i64>;

    fn is_admin(&self) -> bool;

    /// returns a hashset of permissions if the user is logged in
    /// otherwise returns none
    fn permissions(&self) -> Option<HashSet<Permission>>;

    fn all_permissions(&self) -> HashSet<Permission>;

    fn username(&self) -> Option<String>;

}

/// Note that the permissions here are grabbed from either the jwt, or the
/// database
impl<'a, P> GetUserInfo for UserInfo<'a, P>
    where P: PermissionStoreFunctions
{
    fn user_id(&self) -> Option<i64> {
        self.claims.to_owned().map(|x| x.get_user_id())
    }

    fn is_admin(&self) -> bool {
        self.claims.to_owned().map(|x| x.is_user_admin()).unwrap_or(false)
    }

    fn permissions(&self) -> Option<HashSet<Permission>> {
        self.user_id().map(|user_id| {
            let raw_permissions_result = self.permission_store.get_user_permissions(user_id);
            let raw_permissions = match raw_permissions_result {
                Ok(res) => res,
                Err(err) => {
                    error!("encountered an error when trying to get all permissions: {:?}", err);
                    vec![]
                }
            };

            let permissions = raw_permissions.into_iter()
                .flat_map(|raw_permission| {
                    raw_permission.as_permission()
                });

            HashSet::from_iter(permissions)
        })
    }

    fn all_permissions(&self) -> HashSet<Permission> {
        let raw_permissions_result = self.permission_store.get_all_permissions();
        let raw_permissions = match raw_permissions_result {
            Ok(res) => res,
            Err(err) => {
                error!("encountered an error when trying to get all permissions: {:?}", err);
                vec![]
            }
        };

        let permissions = raw_permissions.into_iter()
            .flat_map(|raw_permission| {
                raw_permission.as_permission()
            });

        HashSet::from_iter(permissions)
    }

    fn username(&self) -> Option<String> {
        self.claims.to_owned().map(|x| x.get_username())
    }
}