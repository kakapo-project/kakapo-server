
use super::schema::{Schema, Reference, Constraint};
use super::types::DataType::*;


pub fn user_account() -> Schema {
    Schema::new("user_account").id_column()
        .column("username", &StringType)
        .column("password", &StringType)
        .column("email", &StringType)
}


pub fn scope() -> Schema {
    Schema::new("scope")
        .id_column()
        .column("name", &StringType)
        .column("description", &StringType)
        .column("scope_info", &JsonType)
        .reference(&Reference::table("scope"))
}


pub fn entity() -> Schema {
    Schema::new("entity")
        .id_column()
        .inherited_by(&vec!["table", "query", "script"])
        .column("created_at", &TimestampType)
        .reference(&Reference::table_on_column("user_account", "created_by"))
        .reference(&Reference::table("scope"))
}


pub fn table() -> Schema {
    Schema::new("table")
        .id_column()
        .inherits("entity")
}


pub fn table_history() -> Schema {
    Schema::new("table_history")
        .id_column()
        .reference(&Reference::table("table"))
        .column("name", &StringType)
        .column("description", &StringType)
        .column("table_info", &JsonType)
        .column("modified_at", &TimestampType)
        .reference(&Reference::table_on_column("user_account", "modified_by"))
}


pub fn query() -> Schema {
    Schema::new("query")
        .id_column()
        .inherits("entity")
}


pub fn query_history() -> Schema {
    Schema::new("query_history")
        .id_column()
        .reference(&Reference::table("query"))
        .column("name", &StringType)
        .column("description", &StringType)
        .column("statement", &StringType)
        .column("query_info", &JsonType)
        .column("modified_at", &TimestampType)
        .reference(&Reference::table_on_column("user_account", "modified_by"))
}


pub fn script() -> Schema {
    Schema::new("script")
        .id_column()
        .inherits("entity")
}


pub fn script_history() -> Schema {
    Schema::new("script_history")
        .id_column()
        .reference(&Reference::table("script"))
        .column("name", &StringType)
        .column("description", &StringType)
        .column("language", &StringType)
        .column("script", &StringType)
        .column("script_info", &JsonType)
        .column("modified_at", &TimestampType)
        .reference(&Reference::table_on_column("user_account", "modified_by"))
}


pub fn tag() -> Schema {
    Schema::new("tag")
        .id_column()
        .column("name", &StringType)
        .column("description", &StringType)
        .column("tag_info", &JsonType)
}


pub fn entity_tag() -> Schema {
    Schema::new("entity_tag")
        .id_column()
        .junction("entity", "tag")
}


pub fn role() -> Schema {
    Schema::new("role")
        .id_column()
        .column("name", &StringType)
        .column("description", &StringType)
        .column("role_info", &JsonType)
}


pub fn user_account_role() -> Schema {
    Schema::new("user_account_role")
        .id_column()
        .junction("user_account", "role")
}


pub fn permission() -> Schema {
    Schema::new("permission")
        .id_column()
        .column("name", &StringType)
        .column("description", &StringType)
        .column("permission_info", &JsonType)
}


pub fn role_permission() -> Schema {
    Schema::new("role_permission")
        .id_column()
        .junction("role", "permission")
}


pub fn transaction() -> Schema {
    Schema::new("transaction")
        .id_column()
        .column("version", &StringType)
        .column("action", &JsonType)
        .column("timestamp", &TimestampType)
        .reference(&Reference::table("table"))
        .reference(&Reference::table("user_account"))
}


pub fn version() -> Schema {
    Schema::new("version")
        .id_column()
        .column("version", &StringType)
        .column("timestamp", &TimestampType)
}