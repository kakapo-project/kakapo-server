use model::state::State;
use model::state::ChannelBroadcaster;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
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
    AddUser,
}

impl Permission {
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

    pub fn add_user() -> Self {
        Permission::AddUser
    }
}

pub struct AuthPermissions;

pub trait AuthPermissionFunctions<B> //TODO: the ChannelBroadcast shouldn't be here
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn get_permissions(state: &State<B>) -> HashSet<Permission>;
}

impl<B> AuthPermissionFunctions<B> for AuthPermissions
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn get_permissions(state: &State<B>) -> HashSet<Permission> {
        unimplemented!()
    }
}