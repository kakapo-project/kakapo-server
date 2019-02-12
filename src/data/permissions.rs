
use model::state::ActionState;
use model::entity::RawEntityTypes;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    #[serde(rename_all = "camelCase")]
    HasRole {
        rolename: String
    },

    #[serde(rename_all = "camelCase")]
    GetEntity {
        type_name: String,
        entity_name: String,
    },
    #[serde(rename_all = "camelCase")]
    CreateEntity {
        type_name: String,
    },
    #[serde(rename_all = "camelCase")]
    ModifyEntity {
        type_name: String,
        entity_name: String,
    },

    #[serde(rename_all = "camelCase")]
    GetTableData {
        table_name: String,
    },
    #[serde(rename_all = "camelCase")]
    ModifyTableData {
        table_name: String,
    },
    #[serde(rename_all = "camelCase")]
    RunQuery {
        query_name: String,
    },
    #[serde(rename_all = "camelCase")]
    RunScript {
        script_name: String,
    },

    #[serde(rename_all = "camelCase")]
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
    pub fn has_role(name: String) -> Self { //TODO: this doesn't make any sense
        Permission::HasRole {
            rolename: name
        }
    }

    pub fn read_entity<T>(name: String) -> Self
        where T: RawEntityTypes
    {
        Permission::GetEntity {
            type_name: T::TYPE_NAME.to_string(),
            entity_name: name,
        }
    }

    pub fn create_entity<T>() -> Self
        where T: RawEntityTypes
    {
        Permission::CreateEntity {
            type_name: T::TYPE_NAME.to_string(),
        }
    }

    pub fn modify_entity<T>(name: String) -> Self
        where T: RawEntityTypes
    {
        Permission::ModifyEntity {
            type_name: T::TYPE_NAME.to_string(),
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


