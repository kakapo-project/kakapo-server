use diesel::RunQueryDsl;
use model::state::ActionState;
use model::state::GetConnection;
use data::dbdata::RawPermission;
use model::auth::error::UserManagementError;
use std::error::Error;
use diesel::sql_types::BigInt;


pub struct PermissionStore;

pub trait PermissionStoreFunctions<S>
    where Self: Send
{

    fn get_user_permissions(state: &S, user_id: i64) -> Result<Vec<RawPermission>, UserManagementError>;
    fn get_all_permissions(state: &S) -> Result<Vec<RawPermission>, UserManagementError>;
}

impl PermissionStoreFunctions<ActionState> for PermissionStore {

    fn get_user_permissions(state: &ActionState, user_id: i64) -> Result<Vec<RawPermission>, UserManagementError> {
        let query = r#"
        SELECT
            DISTINCT ON("permission"."permission_id")
            * FROM "user"
        INNER JOIN "user_role"
            ON "user"."user_id" = "user_role"."user_id"
        INNER JOIN "role"
            ON "user_role"."role_id" = "role"."role_id"
        INNER JOIN "role_permission"
            ON "role"."role_id" = "role_permission"."role_id"
        INNER JOIN "permission"
            ON "role_permission"."permission_id" = "permission"."permission_id"
        WHERE "user"."user_id" = $1;
        "#;

        let conn = state.get_conn();
        let result: Vec<RawPermission> = diesel::sql_query(query)
            .bind::<BigInt, _>(user_id)
            .load(conn)
            .or_else(|err| Err(UserManagementError::InternalError(err.description().to_string())))?;

        Ok(result)
    }

    fn get_all_permissions(state: &ActionState) -> Result<Vec<RawPermission>, UserManagementError> {

        let query = r#"
        SELECT
            DISTINCT ON("permission"."permission_id")
            * FROM "permission";
        "#;

        let conn = state.get_conn();
        let result: Vec<RawPermission> = diesel::sql_query(query)
            .load(conn)
            .or_else(|err| Err(UserManagementError::InternalError(err.description().to_string())))?;

        Ok(result)
    }
}
