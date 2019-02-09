use diesel::RunQueryDsl;
use model::state::ActionState;
use model::auth::error::UserManagementError;
use std::error::Error;
use diesel::sql_types::BigInt;
use data::dbdata::RawPermission;
use model::state::StateFunctions;
use connection::executor::Conn;


pub struct PermissionStore<'a> {
    pub conn: &'a Conn,
}

pub trait PermissionStoreFunctions {
    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<RawPermission>, UserManagementError>;
    fn get_all_permissions(&self) -> Result<Vec<RawPermission>, UserManagementError>;
}

impl<'a> PermissionStoreFunctions for PermissionStore<'a> {
    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<RawPermission>, UserManagementError> {
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

        let result: Vec<RawPermission> = diesel::sql_query(query)
            .bind::<BigInt, _>(user_id)
            .load(self.conn)
            .or_else(|err| Err(UserManagementError::InternalError(err.description().to_string())))?;

        Ok(result)
    }

    fn get_all_permissions(&self) -> Result<Vec<RawPermission>, UserManagementError> {

        let query = r#"
        SELECT
            DISTINCT ON("permission"."permission_id")
            * FROM "permission";
        "#;

        let result: Vec<RawPermission> = diesel::sql_query(query)
            .load(self.conn)
            .or_else(|err| Err(UserManagementError::InternalError(err.description().to_string())))?;

        Ok(result)
    }
}
