table! {
    entity (entity_id) {
        entity_id -> Int8,
        scope_id -> Int8,
        created_at -> Timestamp,
        created_by -> Int8,
    }
}

table! {
    entity_tag (entity_tag_id) {
        entity_tag_id -> Int8,
        entity_id -> Int8,
        tag_id -> Int8,
    }
}

table! {
    permission (permission_id) {
        permission_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        permission_info -> Json,
    }
}

table! {
    query (query_id) {
        query_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
    }
}

table! {
    query_history (query_history_id) {
        query_history_id -> Int8,
        query_id -> Int8,
        description -> Varchar,
        statement -> Varchar,
        query_info -> Json,
        modified_at -> Timestamp,
        modified_by -> Int8,
    }
}

table! {
    role (role_id) {
        role_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        role_info -> Json,
    }
}

table! {
    role_permission (role_permission_id) {
        role_permission_id -> Int8,
        role_id -> Int8,
        permission_id -> Int8,
    }
}

table! {
    scope (scope_id) {
        scope_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        scope_info -> Json,
    }
}

table! {
    script (script_id) {
        script_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
    }
}

table! {
    script_history (script_history_id) {
        script_history_id -> Int8,
        script_id -> Int8,
        description -> Varchar,
        script_language -> Varchar,
        script_text -> Varchar,
        script_info -> Json,
        modified_at -> Timestamp,
        modified_by -> Int8,
    }
}

table! {
    table_schema (table_schema_id) {
        table_schema_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
    }
}

table! {
    table_schema_history (table_schema_history_id) {
        table_schema_history_id -> Int8,
        table_schema_id -> Int8,
        description -> Varchar,
        modification -> Json,
        modified_at -> Timestamp,
        modified_by -> Int8,
    }
}

table! {
    table_schema_transaction (transaction_id) {
        transaction_id -> Int8,
        version -> Varchar,
        action_data -> Json,
        table_schema_id -> Int8,
        made_at -> Timestamp,
        made_by -> Int8,
    }
}

table! {
    tag (tag_id) {
        tag_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        tag_info -> Json,
    }
}

table! {
    user_account (user_account_id) {
        user_account_id -> Int8,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    user_account_role (user_account_role_id) {
        user_account_role_id -> Int8,
        user_account_id -> Int8,
        role_id -> Int8,
    }
}

table! {
    version (version_id) {
        version_id -> Int8,
        version_update -> Varchar,
        updated_at -> Timestamp,
    }
}

joinable!(entity -> scope (scope_id));
joinable!(entity -> user_account (created_by));
joinable!(entity_tag -> entity (entity_id));
joinable!(entity_tag -> tag (tag_id));
joinable!(query_history -> query (query_id));
joinable!(query_history -> user_account (modified_by));
joinable!(role_permission -> permission (permission_id));
joinable!(role_permission -> role (role_id));
joinable!(script_history -> script (script_id));
joinable!(script_history -> user_account (modified_by));
joinable!(table_schema_history -> table_schema (table_schema_id));
joinable!(table_schema_history -> user_account (modified_by));
joinable!(table_schema_transaction -> table_schema (table_schema_id));
joinable!(table_schema_transaction -> user_account (made_by));
joinable!(user_account_role -> role (role_id));
joinable!(user_account_role -> user_account (user_account_id));

allow_tables_to_appear_in_same_query!(
    entity,
    entity_tag,
    permission,
    query,
    query_history,
    role,
    role_permission,
    scope,
    script,
    script_history,
    table_schema,
    table_schema_history,
    table_schema_transaction,
    tag,
    user_account,
    user_account_role,
    version,
);
