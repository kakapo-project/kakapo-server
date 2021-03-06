use std::collections::HashSet;
use std::iter::FromIterator;

use data::permissions::Permission;
use data::claims::AuthClaims;

use metastore::dbdata::RawPermission;
use diesel::sql_types::BigInt;
use diesel::RunQueryDsl;
use connection::executor::Conn;

use state::Authorization;
use state::authorization::AuthorizationOps;
use state::error::UserManagementError;


impl<'a> AuthorizationOps for Authorization<'a> {

    fn is_logged_in(&self) -> bool {
        self.claims.is_some()
    }

    //TODO: maybe ths should be in authentication, but a lot of these should be moved around between authentication, authorization and user management
    fn user_id(&self) -> Option<i64> {
        self.claims
            .to_owned()
            .map(|x| x.get_user_id())
    }

    fn is_admin(&self) -> bool {
        self.claims.to_owned().map(|x| x.is_user_admin()).unwrap_or(false)
    }

    fn permissions(&self) -> HashSet<Permission> {
        let user_id = match self.user_id() {
            Some(x) => x,
            None => return HashSet::new()
        };

        let raw_permissions_result = self.get_user_permissions(user_id);
        let raw_permissions = match raw_permissions_result {
            Ok(res) => res,
            Err(err) => {
                error!("encountered an error when trying to get all permissions: {:?}", err);
                vec![]
            }
        };

        HashSet::from_iter(raw_permissions)
    }

    fn all_permissions(&self) -> HashSet<Permission> {
        let raw_permissions_result = self.get_all_permissions();
        let raw_permissions = match raw_permissions_result {
            Ok(res) => res,
            Err(err) => {
                error!("encountered an error when trying to get all permissions: {:?}", err);
                vec![]
            }
        };

        HashSet::from_iter(raw_permissions)
    }

    fn username(&self) -> Option<String> {
        self.claims.to_owned().map(|x| x.get_username())
    }
}

impl<'a> Authorization<'a> {
    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<Permission>, UserManagementError> {
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
            .or_else(|err| Err(UserManagementError::InternalError(err.to_string())))?;

        let permissions: Vec<Permission> = result.into_iter().flat_map(|x| x.as_permission()).collect();
        Ok(permissions)
    }

    fn get_all_permissions(&self) -> Result<Vec<Permission>, UserManagementError> {

        let query = r#"
        SELECT
            DISTINCT ON("permission"."permission_id")
            * FROM "permission";
        "#;

        let result: Vec<RawPermission> = diesel::sql_query(query)
            .load(self.conn)
            .or_else(|err| Err(UserManagementError::InternalError(err.to_string())))?;

        let permissions: Vec<Permission> = result.into_iter().flat_map(|x| x.as_permission()).collect();
        Ok(permissions)
    }
}