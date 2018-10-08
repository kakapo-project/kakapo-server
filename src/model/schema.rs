table! {
    entity (entity_id) {
        entity_id -> Int4,
        entity_type -> Nullable<Varchar>,
        scope_id -> Nullable<Int4>,
        created_at -> Timestamp,
        create_by -> Nullable<Int4>,
    }
}

table! {
    entity_tag (entity_tag_id) {
        entity_tag_id -> Int4,
        entity_id -> Nullable<Int4>,
        tag_id -> Nullable<Int4>,
    }
}

table! {
    permission (permission_id) {
        permission_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        permission_info -> Json,
    }
}

table! {
    query (query_id) {
        query_id -> Int4,
        entity_id -> Nullable<Int4>,
        entity_type -> Nullable<Varchar>,
    }
}

table! {
    query_history (query_history_id) {
        query_history_id -> Int4,
        query_id -> Nullable<Int4>,
        name -> Varchar,
        description -> Varchar,
        statement -> Varchar,
        query_info -> Json,
        modified_at -> Timestamp,
        modified_by -> Nullable<Int4>,
    }
}

table! {
    role (role_id) {
        role_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        role_info -> Json,
    }
}

table! {
    role_permission (role_permission_id) {
        role_permission_id -> Int4,
        role_id -> Nullable<Int4>,
        permission_id -> Nullable<Int4>,
    }
}

table! {
    scope (scope_id) {
        scope_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        scope_info -> Json,
    }
}

table! {
    script (script_id) {
        script_id -> Int4,
        entity_id -> Nullable<Int4>,
        entity_type -> Nullable<Varchar>,
    }
}

table! {
    script_history (script_history_id) {
        script_history_id -> Int4,
        script_id -> Nullable<Int4>,
        name -> Varchar,
        description -> Varchar,
        script_language -> Varchar,
        script_text -> Varchar,
        script_info -> Json,
        modified_at -> Timestamp,
        modified_by -> Nullable<Int4>,
    }
}

table! {
    spread_sheet (spread_sheet_id) {
        spread_sheet_id -> Int4,
        entity_id -> Nullable<Int4>,
        entity_type -> Nullable<Varchar>,
    }
}

table! {
    spread_sheet_history (spread_sheet_history_id) {
        spread_sheet_history_id -> Int4,
        spread_sheet_id -> Nullable<Int4>,
        name -> Varchar,
        description -> Varchar,
        spread_sheet_info -> Json,
        modified_at -> Timestamp,
        modified_by -> Nullable<Int4>,
    }
}

table! {
    spread_sheet_transaction (transaction_id) {
        transaction_id -> Int4,
        version -> Varchar,
        action_data -> Json,
        spread_sheet_id -> Nullable<Int4>,
        made_at -> Timestamp,
        made_by -> Nullable<Int4>,
    }
}

table! {
    tag (tag_id) {
        tag_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        tag_info -> Json,
    }
}

table! {
    user_account (user_account_id) {
        user_account_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

table! {
    user_account_role (user_account_role_id) {
        user_account_role_id -> Int4,
        user_account_id -> Nullable<Int4>,
        role_id -> Nullable<Int4>,
    }
}

table! {
    version (version_id) {
        version_id -> Int4,
        version_update -> Varchar,
        update_at -> Timestamp,
    }
}

joinable!(entity -> scope (scope_id));
joinable!(entity -> user_account (create_by));
joinable!(entity_tag -> entity (entity_id));
joinable!(entity_tag -> tag (tag_id));
joinable!(query_history -> query (query_id));
joinable!(query_history -> user_account (modified_by));
joinable!(role_permission -> permission (permission_id));
joinable!(role_permission -> role (role_id));
joinable!(script_history -> script (script_id));
joinable!(script_history -> user_account (modified_by));
joinable!(spread_sheet_history -> spread_sheet (spread_sheet_id));
joinable!(spread_sheet_history -> user_account (modified_by));
joinable!(spread_sheet_transaction -> spread_sheet (spread_sheet_id));
joinable!(spread_sheet_transaction -> user_account (made_by));
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
    spread_sheet,
    spread_sheet_history,
    spread_sheet_transaction,
    tag,
    user_account,
    user_account_role,
    version,
);
