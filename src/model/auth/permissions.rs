use std::collections::HashSet;

use model::auth::internal::PermissionMgr;
use model::auth::internal::PermissionMgrFunctions;
use model::state::State;
use std::marker::PhantomData;
use model::state::GetConnection;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    HasRole {
        rolename: String
    },

    GetEntity {
        type_name: &'static str,
        entity_name: String,
    },
    CreateEntity {
        type_name: &'static str,
    },
    ModifyEntity {
        type_name: &'static str,
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
            type_name: "temporary...", //TODO: this should be a const
            entity_name: name,
        }
    }

    pub fn create_entity<T>() -> Self {
        Permission::CreateEntity {
            type_name: "temporary...", //TODO: this should be a const
        }
    }

    pub fn modify_entity<T>(name: String) -> Self {
        Permission::ModifyEntity {
            type_name: "temporary...", //TODO: this should be a const
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
    where Self: Send + GetConnection
{
    const ADMIN_USER_ID: i64;

    fn get_user_id(&self) -> Option<i64>;

    fn is_admin(&self) -> bool;

    /// returns a hashset of permissions if the user is logged in
    /// otherwise returns none
    fn get_permissions(&self) -> Option<HashSet<Permission>>;

    fn get_all_permissions(&self) -> HashSet<Permission>;

    fn get_db_user(&self) -> String;

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

    fn get_permissions(&self) -> Option<HashSet<Permission>> {
        unimplemented!()
    }

    fn get_all_permissions(&self) -> HashSet<Permission> {
        let raw_permissions_result = PermissionMgr::get_all_permissions(self);
        let raw_permissions = match raw_permissions_result {
            Ok(res) => res,
            Err(err) => {
                error!("encountered an error when trying to get all permissions: {:?}", err);
                vec![]
            }
        };

        unimplemented!()
    }

    fn get_db_user(&self) -> String {
        "my_user".to_string()
    }
}