use std::collections::HashSet;

use model::auth::permission_store::PermissionStoreFunctions;
use model::state::State;
use model::state::GetConnection;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    HasRole {
        rolename: String
    },

    GetEntity {
        type_name: String,
        entity_name: String,
    },
    CreateEntity {
        type_name: String,
    },
    ModifyEntity {
        type_name: String,
        entity_name: String,
    },

    GetTableData {
        table_name: String,
    },
    ModifyTableData {
        table_name: String,
    },
    RunQuery {
        query_name: String,
    },
    RunScript {
        script_name: String,
    },

    User { // manage user can detach roles
        username: String,
    },
    UserEmail {
        email: String,
    },
    UserAdmin, //can add or remove users,
    // and add roles if the user has that role
    // and add permission to role if the user has that role and permission

}

impl Permission {
    pub fn has_role(name: String) -> Self {
        Permission::HasRole {
            rolename: name
        }
    }

    pub fn read_entity<T>(name: String) -> Self {
        Permission::GetEntity {
            type_name: "temporary...".to_string(), //TODO: this should be a const
            entity_name: name,
        }
    }

    pub fn create_entity<T>() -> Self {
        Permission::CreateEntity {
            type_name: "temporary...".to_string(), //TODO: this should be a const
        }
    }

    pub fn modify_entity<T>(name: String) -> Self {
        Permission::ModifyEntity {
            type_name: "temporary...".to_string(), //TODO: this should be a const
            entity_name: name,
        }
    }

    pub fn get_table_data(name: String) -> Self {
        Permission::GetTableData {
            table_name: name
        }
    }

    pub fn modify_table_data(name: String) -> Self {
        Permission::ModifyTableData {
            table_name: name
        }
    }

    pub fn run_query(name: String) -> Self {
        Permission::RunQuery {
            query_name: name
        }
    }

    pub fn run_script(name: String) -> Self {
        Permission::RunScript {
            script_name: name
        }
    }

    pub fn user_admin() -> Self {
        Permission::UserAdmin
    }

    pub fn user(username: String) -> Self {
        Permission::User {
            username,
        }
    }

    pub fn user_email(email: String) -> Self {
        Permission::UserEmail {
            email,
        }
    }
}


pub trait GetUserInfo
    where
        Self: Send + Sized + GetConnection,
{
    const ADMIN_USER_ID: i64;

    fn get_user_id(&self) -> Option<i64>;

    fn is_admin(&self) -> bool;

    /// returns a hashset of permissions if the user is logged in
    /// otherwise returns none
    fn get_permissions<AS>(&self) -> Option<HashSet<Permission>>
        where AS: PermissionStoreFunctions<State>;

    fn get_all_permissions<AS>(&self) -> HashSet<Permission>
        where AS: PermissionStoreFunctions<State>;

    fn get_username(&self) -> Option<String>;

}

/// Note that the permissions here are grabbed from either the jwt, or the
/// database
impl GetUserInfo for State {
    const ADMIN_USER_ID: i64 = 1;

    fn get_user_id(&self) -> Option<i64> {
        self.claims.to_owned().map(|x| x.get_user_id())
    }

    fn is_admin(&self) -> bool {
        self.claims.to_owned().map(|x| x.is_user_admin()).unwrap_or(false)
    }

    fn get_permissions<AS>(&self) -> Option<HashSet<Permission>>
        where AS: PermissionStoreFunctions<State>
    {
        self.get_user_id().map(|user_id| {
            let raw_permissions_result = AS::get_user_permissions(self, user_id);
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

    fn get_all_permissions<AS>(&self) -> HashSet<Permission>
        where AS: PermissionStoreFunctions<State>
    {
        let raw_permissions_result = AS::get_all_permissions(self);
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

    fn get_username(&self) -> Option<String> {
        self.claims.to_owned().map(|x| x.get_username())
    }
}