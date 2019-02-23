use data::permissions::Permission;
use data::channels::Channels;
use model::actions::ActionResult;
use std::collections::HashSet;
use data;

//WARNING: make sure this matches the permissions in the action decorations
//TODO: we can get better type safety guarantees by trait objects for channels, but I'm not sure what kind of challenges that would have
// I would imagine it would look something like this:
// trait Channel {
//     type Data;
// }
// impl Channel for AllTablesChannel {
//     type Data = data::Table;
// }
pub trait SessionHasPermission {
    fn has_permission_to_subscribe(&self, permissions: &HashSet<Permission>) -> bool;

    fn has_permission_to_receive(&self, data: serde_json::Value, permissions: &HashSet<Permission>) -> bool;
}


impl SessionHasPermission for Channels {
    fn has_permission_to_subscribe(&self, permissions: &HashSet<Permission>) -> bool {
        match self {
            Channels::AllTables => true,
            Channels::AllQueries => true,
            Channels::AllScripts => true,
            Channels::Table(name) => {
                permissions.contains(&Permission::read_entity::<data::Table>(name.to_owned()))
            },
            Channels::Query(name) => {
                permissions.contains(&Permission::read_entity::<data::Query>(name.to_owned()))
            },
            Channels::Script(name) => {
                permissions.contains(&Permission::read_entity::<data::Script>(name.to_owned()))
            },
            Channels::TableData(name) => {
                permissions.contains(&Permission::get_table_data(name.to_owned()))
            },
        }
    }

    fn has_permission_to_receive(&self, data: serde_json::Value, permissions: &HashSet<Permission>) -> bool {
        match self {
            Channels::AllTables => {
                if let Some(name) = data.get("name").and_then(|x| x.as_str()) {
                    permissions.contains(&Permission::read_entity::<data::Table>(name.to_string()))
                } else {
                    error!("Could not find the name field in the return json");
                    false
                }
            },
            Channels::AllQueries => {
                if let Some(name) = data.get("name").and_then(|x| x.as_str()) {
                    permissions.contains(&Permission::read_entity::<data::Query>(name.to_string()))
                } else {
                    error!("Could not find the name field in the return json");
                    false
                }
            },
            Channels::AllScripts => {
                if let Some(name) = data.get("name").and_then(|x| x.as_str()) {
                    permissions.contains(&Permission::read_entity::<data::Script>(name.to_string()))
                } else {
                    error!("Could not find the name field in the return json");
                    false
                }
            },
            Channels::Table(name) => {
                permissions.contains(&Permission::read_entity::<data::Table>(name.to_owned()))
            },
            Channels::Query(name) => {
                permissions.contains(&Permission::read_entity::<data::Query>(name.to_owned()))
            },
            Channels::Script(name) => {
                permissions.contains(&Permission::read_entity::<data::Script>(name.to_owned()))
            },
            Channels::TableData(name) => {
                permissions.contains(&Permission::get_table_data(name.to_owned()))
            },
        }
    }
}