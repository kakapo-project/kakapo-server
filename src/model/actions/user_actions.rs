
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;

use model::state::State;
use model::state::GetConnection;
use model::auth::permissions::*;
use model::actions::decorator::*;

use model::auth::Auth;
use model::auth::AuthFunctions;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::auth::permissions::GetUserInfo;
use model::state::GetBroadcaster;

#[derive(Debug)]
pub struct Authenticate<S = State, AF = Auth> {
    user_identifier: String,
    password: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> Authenticate<S, AF>
    where
        S: GetConnection + GetUserInfo,
        AF: AuthFunctions<S>,
{
    pub fn new(user_identifier: String, password: String) -> WithTransaction<Self, S> {
        let action = Self {
            user_identifier,
            password,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);

        action_with_transaction
    }
}

impl<S, AF> Action<S> for Authenticate<S, AF>
    where
        S: GetConnection + GetUserInfo,
        AF: AuthFunctions<S>,
{
    type Ret = Option<()>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::authenticate(state, &self.user_identifier, &self.password)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("Authenticate", Some(())))
    }
}



/// User Auth: Get All users
#[derive(Debug)]
pub struct GetAllUsers<S = State, AF = Auth> {
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> GetAllUsers<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new() -> WithLoginRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);

        /* everyone who is logged in can access to the users */
        let action_with_permission = WithLoginRequired::new(action_with_transaction);

        action_with_permission
    }
}

impl<S, AF> Action<S> for GetAllUsers<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = AllUsersResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::get_all_users(state)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("GetAllUsers", AllUsersResult(res)))
    }
}

/// User Auth: Add user with password
/// Usually, this isn't used, instead use invitation
#[derive(Debug)]
pub struct AddUser<S = State, AF = Auth> {
    user: data::auth::NewUser,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> AddUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user: data::auth::NewUser) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            user,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin());

        action_with_permission
    }
}

impl<S, AF> Action<S> for AddUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::add_user(state, &self.user)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AddUser", UserResult(res)))
    }
}

/// User Auth: Remove User
#[derive(Debug)]
pub struct RemoveUser<S = State, AF = Auth> {
    user_identifier: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> RemoveUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user_identifier: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            user_identifier,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin());

        action_with_permission
    }
}

impl<S, AF> Action<S> for RemoveUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::remove_user(state, &self.user_identifier)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("RemoveUser", UserResult(res)))
    }
}

/// User Auth: Email user for invitation
#[derive(Debug)]
pub struct InviteUser<S = State, AF = Auth> {
    email: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> InviteUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(email: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            email,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin());

        action_with_permission
    }
}

impl<S, AF> Action<S> for InviteUser<S, AF>
    where
        S: GetConnection + GetUserInfo,
        AF: AuthFunctions<S>,
{
    type Ret = InvitationToken;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::invite_user(state, &self.email)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("InviteUser", InvitationToken(res)))
    }
}

/// Add User with an invitation token
#[derive(Debug)]
pub struct SetupUser<S = State, AF = Auth> {
    user: data::auth::NewUser,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> SetupUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user: data::auth::NewUser) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            user,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        //TODO: with the invitation token
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin());

        action_with_permission
    }
}

impl<S, AF> Action<S> for SetupUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::add_user(state, &self.user)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("SetupUser", UserResult(res)))
    }
}


/// User Auth: Set user password
#[derive(Debug)]
pub struct SetUserPassword<S = State, AF = Auth> {
    user_identifier: String,
    password: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> SetUserPassword<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user_identifier: String, password: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user(user_identifier.to_owned()),
            Permission::user_email(user_identifier.to_owned())];

        let action = Self {
            user_identifier,
            password,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new_any_of(
                action_with_transaction,
                required_permissions);

        action_with_permission
    }
}

impl<S, AF> Action<S> for SetUserPassword<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::modify_user_password(state, &self.user_identifier, &self.password)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("SetUserPassword", UserResult(res)))
    }
}

/// Role Auth: Add Role
#[derive(Debug)]
pub struct AddRole<S = State, AF = Auth> {
    role: data::auth::Role,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> AddRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(role: data::auth::Role) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            role,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin()); //TODO: also needs the role, or maybe not, idk

        action_with_permission
    }
}

impl<S, AF> Action<S> for AddRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::add_role(state, &self.role)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AddRole", RoleResult(res)))
    }
}

/// Role Auth: Remove role
#[derive(Debug)]
pub struct RemoveRole<S = State, AF = Auth> {
    rolename: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> RemoveRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(rolename: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            rolename,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin()); //TODO: also needs to have the role, or maybe not idk

        action_with_permission
    }
}

impl<S, AF> Action<S> for RemoveRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::remove_role(state, &self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("RemoveRole", RoleResult(res)))
    }
}

/// Role Auth: get all role
#[derive(Debug)]
pub struct GetAllRoles<S = State, AF = Auth> {
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> GetAllRoles<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new() -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new(action_with_transaction, Permission::user_admin());

        action_with_permission
    }
}

impl<S, AF> Action<S> for GetAllRoles<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = AllRolesResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::get_all_roles(state)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("GetAllRoles", AllRolesResult(res)))
    }
}

/// Role Auth: add permission
#[derive(Debug)]
pub struct AttachPermissionForRole<S = State, AF = Auth> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> AttachPermissionForRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(rolename: String, permission: Permission) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(rolename.to_owned()),
            permission.to_owned(),
        ];
        let action = Self {
            rolename,
            permission,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new_all_of(action_with_transaction, required_permissions);

        action_with_permission
    }
}

impl<S, AF> Action<S> for AttachPermissionForRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::attach_permission_for_role(state, &self.permission, &self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AttachPermissionForRole", RoleResult(res)))
    }
}

/// Role Auth: remove permission
#[derive(Debug)]
pub struct DetachPermissionForRole<S = State, AF = Auth> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> DetachPermissionForRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(rolename: String, permission: Permission) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(rolename.to_owned()),
            permission.to_owned(),
        ];
        let action = Self {
            rolename,
            permission,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new_all_of(action_with_transaction, required_permissions);

        action_with_permission
    }
}

impl<S, AF> Action<S> for DetachPermissionForRole<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::detach_permission_for_role(state, &self.permission, &self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("DetachPermissionForRole", RoleResult(res)))
    }
}

/// Role Auth: add role for user
#[derive(Debug)]
pub struct AttachRoleForUser<S = State, AF = Auth> {
    role: data::auth::Role,
    user_identifier: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> AttachRoleForUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user_identifier: String, role: data::auth::Role) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(role.get_name()),
        ];
        let action = Self {
            role,
            user_identifier,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new_all_of(action_with_transaction, required_permissions);

        action_with_permission
    }
}

impl<S, AF> Action<S> for AttachRoleForUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::attach_role_for_user(state, &self.role, &self.user_identifier)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AttachRoleForUser", UserResult(res)))
    }
}

/// Role Auth: remove role for user
#[derive(Debug)]
pub struct DetachRoleForUser<S = State, AF = Auth> {
    role: data::auth::Role,
    user_identifier: String,
    phantom_data: PhantomData<(S, AF)>,
}

impl<S, AF> DetachRoleForUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    pub fn new(user_identifier: String, role: data::auth::Role) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(role.get_name()),
        ];
        let action = Self {
            role,
            user_identifier,
            phantom_data: PhantomData,
        };

        let action_with_transaction = WithTransaction::new(action);
        let action_with_permission =
            WithPermissionRequired::new_all_of(action_with_transaction, required_permissions);

        action_with_permission
    }
}

impl<S, AF> Action<S> for DetachRoleForUser<S, AF>
    where
        S: GetConnection + GetUserInfo + GetBroadcaster,
        AF: AuthFunctions<S>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        AF::detach_role_for_user(state, &self.role, &self.user_identifier)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("DetachRoleForUser", UserResult(res)))
    }
}

#[cfg(test)]
mod test {
    use super::*;


    use diesel::r2d2::ConnectionManager;
    use diesel::pg::PgConnection;
    use diesel::Connection;

    use serde_json::from_value;
    use scripting::Scripting;
    use diesel::r2d2::Pool;
    use model::state::AuthClaims;
    use connection::Broadcaster;
    use std::sync::Arc;
    use connection::BroadcasterError;
    use data;
    use model::actions::results::CreateEntityResult::Created;
    use model::actions::results::DeleteEntityResult::Deleted;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    struct TestBroadcaster;

    impl Broadcaster for TestBroadcaster {
        fn publish(&self, channels: Vec<Channels>, action_name: String, action_result: serde_json::Value) -> Result<(), BroadcasterError> {
            Ok(())
        }
    }

    fn random_identifier() -> String {
        let uuid = Uuid::new_v4();
        let res = base64::encode_config(&uuid.as_bytes(), base64::STANDARD_NO_PAD);

        res.replace("/", "_").replace("+", "$")
    }

    fn with_state<F>(f: F)
        where F: FnOnce(&State) -> ()
    {
        let script_path = "./path/to/scripts".to_string();
        let conn_url = "postgres://test:password@localhost:5432/test".to_string();
        let conn_manager: ConnectionManager<PgConnection> = ConnectionManager::new(conn_url);
        let pool = Pool::new(conn_manager).unwrap();
        let pooled_conn = pool.get().unwrap();

        let claims_json = json!({ "iss": "https://doesntmatter.com", "sub": 1, "iat": 0, "exp": -1, "username": "Admin", "isAdmin": true, "role": null });
        let claims: AuthClaims = serde_json::from_value(claims_json).unwrap();
        let broadcaster = Arc::new(TestBroadcaster);

        let state = State::new(pooled_conn, Scripting::new(script_path), Some(claims), broadcaster);
        let conn = state.get_conn();

        conn.test_transaction::<(), Error, _>(|| {
            f(&state);

            Ok(())
        });
    }

    #[test]
    fn test_add_user() {

    }

    #[test]
    fn test_invite_user() {

    }
}