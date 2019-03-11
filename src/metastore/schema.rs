table! {
    channel (channel_id) {
        channel_id -> Int8,
        data -> Jsonb,
    }
}

table! {
    domain (domain_id) {
        domain_id -> Int8,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        description -> Varchar,
        domain_info -> Json,
        created_at -> Timestamp,
    }
}

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
    entity_usage (entity_usage_id) {
        entity_usage_id -> Int8,
        entity_id -> Int8,
        used_at -> Timestamp,
        used_by -> Int8,
    }
}

table! {
    invitation (invitation_id) {
        invitation_id -> Int8,
        email -> Varchar,
        token -> Varchar,
        token_info -> Json,
        sent_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    message (message_id) {
        message_id -> Int8,
        channel_id -> Int8,
        data -> Jsonb,
        sent_at -> Timestamp,
    }
}

table! {
    permission (permission_id) {
        permission_id -> Int8,
        data -> Jsonb,
    }
}

table! {
    query (query_id) {
        query_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        statement -> Varchar,
        query_info -> Json,
        is_deleted -> Bool,
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
        description -> Varchar,
        script_language -> Varchar,
        script_text -> Varchar,
        script_info -> Json,
        is_deleted -> Bool,
        modified_at -> Timestamp,
        modified_by -> Int8,
    }
}

table! {
    session (session_id) {
        session_id -> Int8,
        token -> Varchar,
        user_id -> Int8,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

table! {
    table_schema (table_schema_id) {
        table_schema_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        table_data -> Json,
        is_deleted -> Bool,
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
    user (user_id) {
        user_id -> Int8,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        display_name -> Varchar,
        user_info -> Json,
        joined_at -> Timestamp,
    }
}

table! {
    user_channel (user_channel_id) {
        user_channel_id -> Int8,
        user_id -> Int8,
        channel_id -> Int8,
        subscribed_at -> Timestamp,
    }
}

table! {
    user_role (user_role_id) {
        user_role_id -> Int8,
        user_id -> Int8,
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

table! {
    view (view_id) {
        view_id -> Int8,
        entity_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        view_state -> Json,
        view_info -> Json,
        is_deleted -> Bool,
        modified_at -> Timestamp,
        modified_by -> Int8,
    }
}

joinable!(entity -> scope (scope_id));
joinable!(entity -> user (created_by));
joinable!(entity_tag -> entity (entity_id));
joinable!(entity_tag -> tag (tag_id));
joinable!(entity_usage -> entity (entity_id));
joinable!(entity_usage -> user (used_by));
joinable!(message -> channel (channel_id));
joinable!(query -> entity (entity_id));
joinable!(query -> user (modified_by));
joinable!(role_permission -> permission (permission_id));
joinable!(role_permission -> role (role_id));
joinable!(script -> entity (entity_id));
joinable!(script -> user (modified_by));
joinable!(session -> user (user_id));
joinable!(table_schema -> entity (entity_id));
joinable!(table_schema -> user (modified_by));
joinable!(table_schema_transaction -> table_schema (table_schema_id));
joinable!(table_schema_transaction -> user (made_by));
joinable!(user_channel -> channel (channel_id));
joinable!(user_channel -> user (user_id));
joinable!(user_role -> role (role_id));
joinable!(user_role -> user (user_id));
joinable!(view -> entity (entity_id));
joinable!(view -> user (modified_by));

allow_tables_to_appear_in_same_query!(
    channel,
    domain,
    entity,
    entity_tag,
    entity_usage,
    invitation,
    message,
    permission,
    query,
    role,
    role_permission,
    scope,
    script,
    session,
    table_schema,
    table_schema_transaction,
    tag,
    user,
    user_channel,
    user_role,
    version,
    view,
);
