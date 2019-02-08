
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;

use model::state::ActionState;
use model::state::GetConnection;
use model::auth::permissions::*;
use model::actions::decorator::*;

use model::auth::auth_modifier::Auth;
use model::auth::auth_modifier::AuthFunctions;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::auth::permissions::GetUserInfo;
use model::state::GetBroadcaster;
use model::state::GetSecrets;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use model::auth::send_mail::EmailSender;
use model::state::StateFunctions;

#[derive(Debug)]
pub struct Authenticate<S = ActionState> {
    user_identifier: String,
    password: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> Authenticate<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + StateFunctions<'a>,
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

impl<S> Action<S> for Authenticate<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + StateFunctions<'a>,
{
    type Ret = Option<()>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .authenticate(&self.user_identifier, &self.password)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("Authenticate", Some(())))
    }
}



/// User Auth: Get All users
#[derive(Debug)]
pub struct GetAllUsers<S = ActionState> {
    phantom_data: PhantomData<(S)>,
}

impl<S> GetAllUsers<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for GetAllUsers<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = AllUsersResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .get_all_users()
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("GetAllUsers", AllUsersResult(res)))
    }
}

/// User Auth: Add user with password
/// Usually, this isn't used, instead use invitation
#[derive(Debug)]
pub struct AddUser<S = ActionState> {
    user: data::auth::NewUser,
    phantom_data: PhantomData<(S)>,
}

impl<S> AddUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for AddUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .add_user(&self.user)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AddUser", UserResult(res)))
    }
}

/// User Auth: Remove User
#[derive(Debug)]
pub struct RemoveUser<S = ActionState> {
    user_identifier: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> RemoveUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for RemoveUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .remove_user(&self.user_identifier)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("RemoveUser", UserResult(res)))
    }
}

/// User Auth: Email user for invitation
#[derive(Debug)]
pub struct InviteUser<S = ActionState> {
    email: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> InviteUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + EmailSender + StateFunctions<'a>,
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

impl<S> Action<S> for InviteUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + EmailSender + StateFunctions<'a>,
{
    type Ret = InvitationResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let invitation_token = state.get_auth_functions()
            .create_user_token(&self.email).map_err(Error::UserManagement)?;
        let invitation = state.send_email(invitation_token).map_err(Error::EmailError)?;
        ActionRes::new("InviteUser", InvitationResult(invitation))
    }
}

/// Add User with an invitation token
#[derive(Debug)]
pub struct SetupUser<S = ActionState> {
    user: data::auth::NewUser,
    phantom_data: PhantomData<(S)>,
}

impl<S> SetupUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for SetupUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .add_user(&self.user)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("SetupUser", UserResult(res)))
    }
}


/// User Auth: Set user password
#[derive(Debug)]
pub struct SetUserPassword<S = ActionState> {
    user_identifier: String,
    password: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> SetUserPassword<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for SetUserPassword<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .modify_user_password(&self.user_identifier, &self.password)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("SetUserPassword", UserResult(res)))
    }
}

/// Role Auth: Add Role
#[derive(Debug)]
pub struct AddRole<S = ActionState> {
    role: data::auth::Role,
    phantom_data: PhantomData<(S)>,
}

impl<S> AddRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for AddRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .add_role(&self.role)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AddRole", RoleResult(res)))
    }
}

/// Role Auth: Remove role
#[derive(Debug)]
pub struct RemoveRole<S = ActionState> {
    rolename: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> RemoveRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for RemoveRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .remove_role(&self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("RemoveRole", RoleResult(res)))
    }
}

/// Role Auth: get all role
#[derive(Debug)]
pub struct GetAllRoles<S = ActionState> {
    phantom_data: PhantomData<(S)>,
}

impl<S> GetAllRoles<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for GetAllRoles<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = AllRolesResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .get_all_roles()
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("GetAllRoles", AllRolesResult(res)))
    }
}

/// Role Auth: add permission
#[derive(Debug)]
pub struct AttachPermissionForRole<S = ActionState> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(S)>,
}

impl<S> AttachPermissionForRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for AttachPermissionForRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .attach_permission_for_role(&self.permission, &self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AttachPermissionForRole", RoleResult(res)))
    }
}

/// Role Auth: remove permission
#[derive(Debug)]
pub struct DetachPermissionForRole<S = ActionState> {
    rolename: String,
    permission: Permission,
    phantom_data: PhantomData<(S)>,
}

impl<S> DetachPermissionForRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for DetachPermissionForRole<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .detach_permission_for_role(&self.permission, &self.rolename)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("DetachPermissionForRole", RoleResult(res)))
    }
}

/// Role Auth: add role for user
#[derive(Debug)]
pub struct AttachRoleForUser<S = ActionState> {
    role: data::auth::Role,
    user_identifier: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> AttachRoleForUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for AttachRoleForUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .attach_role_for_user(&self.role, &self.user_identifier)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("AttachRoleForUser", UserResult(res)))
    }
}

/// Role Auth: remove role for user
#[derive(Debug)]
pub struct DetachRoleForUser<S = ActionState> {
    role: data::auth::Role,
    user_identifier: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> DetachRoleForUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
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

impl<S> Action<S> for DetachRoleForUser<S>
    where for<'a> S: GetConnection + GetUserInfo + GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .detach_role_for_user(&self.role, &self.user_identifier)
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
    use Channels;
    use connection::executor::Secrets;
    use model::auth::error::UserManagementError;
    use data::auth::Invitation;
    use model::auth::send_mail::EmailError;
    use data::auth::InvitationToken;
    use model::auth::permission_store::PermissionStoreFunctions;
    use std::collections::HashSet;
    use serde::Serialize;
    use data::auth::Role;
    use data::auth::User;
    use data::auth::NewUser;
    use model::auth::permission_store::PermissionStore;
    use data::dbdata::RawPermission;

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

    #[derive(Debug)]
    struct MockState(ActionState);
    impl<'a> StateFunctions<'a> for MockState {
        type AuthFunctions = <ActionState as StateFunctions<'a>>::AuthFunctions;

        fn get_auth_functions(&'a self) -> <Self as StateFunctions<'a>>::AuthFunctions {
            self.0.get_auth_functions()
        }

        type EntityRetrieverFunctions = <ActionState as StateFunctions<'a>>::EntityRetrieverFunctions;
        fn get_entity_retreiver_functions(&'a self) -> <Self as StateFunctions<'a>>::EntityRetrieverFunctions {
            self.0.get_entity_retreiver_functions()
        }

        type EntityModifierFunctions = <ActionState as StateFunctions<'a>>::EntityModifierFunctions;
        fn get_entity_modifier_function(&'a self) -> <Self as StateFunctions<'a>>::EntityModifierFunctions {
            self.0.get_entity_modifier_function()
        }
    }
    impl GetConnection for MockState {
        type Connection = <ActionState as GetConnection>::Connection;
        fn get_conn(&self) -> &Self::Connection {
            self.0.get_conn()
        }

        fn transaction<G, E, F>(&self, f: F) -> Result<G, E>
            where
                F: FnOnce() -> Result<G, E>,
                E: From<diesel::result::Error>
        { self.0.transaction(f) }
    }

    impl GetUserInfo for MockState {
        const ADMIN_USER_ID: i64 = ActionState::ADMIN_USER_ID;

        fn get_user_id(&self) -> Option<i64> { self.0.get_user_id() }

        fn is_admin(&self) -> bool { self.0.is_admin() }

        fn get_permissions<AS>(&self) -> Option<HashSet<Permission>>
            where AS: PermissionStoreFunctions<ActionState>
        { self.0.get_permissions::<AS>() }

        fn get_all_permissions<AS>(&self) -> HashSet<Permission>
            where AS: PermissionStoreFunctions<ActionState>
        { self.0.get_all_permissions::<AS>() }

        fn get_username(&self) -> Option<String> { self.0.get_username() }
    }

    impl GetSecrets for MockState {
        fn get_token_secret(&self) -> String { self.0.get_token_secret() }

        fn get_password_secret(&self) -> String { self.0.get_password_secret() }
    }

    impl GetBroadcaster for MockState {
        fn publish<R>(&self, channels: Vec<Channels>, action_name: String, action_result: &R) -> Result<(), Error>
            where R: Serialize
        {
            self.0.publish(channels, action_name, action_result)
        }
    }

    impl PermissionStoreFunctions<MockState> for PermissionStore {
        fn get_user_permissions(state: &MockState, user_id: i64) -> Result<Vec<RawPermission>, UserManagementError> {
            PermissionStore::get_user_permissions(&state.0, user_id)
        }

        fn get_all_permissions(state: &MockState) -> Result<Vec<RawPermission>, UserManagementError> {
            PermissionStore::get_all_permissions(&state.0)
        }
    }

    impl EmailSender for MockState {
        fn send_email(&self, invitation_token: InvitationToken) -> Result<Invitation, EmailError> {
            self.0.send_email(invitation_token)
        }
    }


    fn with_state<F>(f: F)
        where F: FnOnce(&MockState) -> ()
    {
        let script_path = "./path/to/scripts".to_string();
        let conn_url = "postgres://test:password@localhost:5432/test".to_string();
        let conn_manager: ConnectionManager<PgConnection> = ConnectionManager::new(conn_url);
        let pool = Pool::new(conn_manager).unwrap();
        let pooled_conn = pool.get().unwrap();

        let claims_json = json!({ "iss": "https://doesntmatter.com", "sub": 1, "iat": 0, "exp": -1, "username": "Admin", "isAdmin": true, "role": null });
        let claims: AuthClaims = serde_json::from_value(claims_json).unwrap();
        let broadcaster = Arc::new(TestBroadcaster);
        let secrets = Secrets {
            token_secret: "A".to_string(),
            password_secret: "B".to_string(),
        };

        let state = ActionState::new(pooled_conn, Scripting::new(script_path), Some(claims), broadcaster, secrets);

        let mock_state = MockState(state);
        let conn = mock_state.0.get_conn();

        conn.test_transaction::<(), Error, _>(|| {
            f(&mock_state);

            Ok(())
        });
    }

    #[test]
    fn test_add_user() {
        with_state(|state| {
            let name = format!("bob_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();
            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

            let name = format!("bob_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2",
                "displayName": "Bob"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();
            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, "Bob");
        });
    }

    #[test]
    fn test_add_user_already_exists() {
        with_state(|state| {
            let name = format!("bob_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();
            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

            let another_name = format!("bob_{}", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": another_name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state).unwrap_err();
            assert_eq!(result, Error::UserManagement(UserManagementError::AlreadyExists));

            let another_email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": another_email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state).unwrap_err();
            assert_eq!(result, Error::UserManagement(UserManagementError::AlreadyExists));
        });
    }

    #[test]
    fn test_add_user_remove_username() {
        with_state(|state| {
            let name = format!("bob_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();
            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

            let create_action = RemoveUser::<MockState>::new(name.to_owned());

            let UserResult(result) = create_action.call(&state).unwrap().get_data();
            assert_eq!(result.email, email.to_owned());
            assert_eq!(result.username, name.to_owned());
            assert_eq!(result.display_name, name.to_owned());

            let create_action = RemoveUser::<MockState>::new(name.to_owned());

            let result= create_action.call(&state).unwrap_err();
            assert_eq!(result, Error::UserManagement(UserManagementError::NotFound));
            println!("result: {:?}", &result);
        });
    }

    #[test]
    fn test_add_user_remove_email() {
        with_state(|state| {
            let name = format!("bob_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();
            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

            let create_action = RemoveUser::<MockState>::new(email.to_owned());

            let UserResult(result) = create_action.call(&state).unwrap().get_data();
            assert_eq!(result.email, email.to_owned());
            assert_eq!(result.username, name.to_owned());
            assert_eq!(result.display_name, name.to_owned());
        });
    }

    #[derive(Debug, Clone)]
    struct MockEmailer;
    impl EmailSender for MockEmailer {
        fn send_email(&self, invitation_token: InvitationToken) -> Result<Invitation, EmailError> {
            unimplemented!()
        }
    }

    #[test]
    fn test_invite_user() {
        with_state(|state| {

            let email = format!("stuff{}@example.com", random_identifier());
            let emailer = MockEmailer;
            let create_action = InviteUser::<MockState>::new(email);

            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            println!("data: {:?}", &data);

        });
    }
}