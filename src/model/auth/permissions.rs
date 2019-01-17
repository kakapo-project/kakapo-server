use model::state::State;
use std::collections::HashSet;

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
}

pub struct AuthPermissions;

pub trait AuthPermissionFunctions {
    /// returns a hashset of permissions if the user is logged in
    /// otherwise returns none
    fn get_permissions(state: &State) -> Option<HashSet<Permission>>;

    fn is_admin(state: &State) -> bool;
}

impl AuthPermissionFunctions for AuthPermissions {
    fn get_permissions(state: &State) -> Option<HashSet<Permission>> {
        unimplemented!()
    }

    fn is_admin(state: &State) -> bool {
        unimplemented!()
    }
}

pub struct AllowAll;
impl AuthPermissionFunctions for AllowAll {
    fn get_permissions(state: &State) -> Option<HashSet<Permission>> {
        Some(HashSet::new())
    }

    fn is_admin(state: &State) -> bool {
        true
    }
}