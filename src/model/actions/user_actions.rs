
use std::marker::PhantomData;

use data;

use model::actions::results::*;
use model::actions::error::Error;

use model::state::ActionState;
use data::permissions::*;
use model::actions::decorator::*;

use metastore::auth_modifier::AuthFunctions;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;
use model::state::GetBroadcaster;
use model::state::GetSecrets;

use model::state::StateFunctions;
use model::auth::send_mail::EmailOps;

#[derive(Debug)]
pub struct Authenticate<S = ActionState> {
    user_identifier: String,
    password: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> Authenticate<S>
    where for<'a> S: GetSecrets + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + StateFunctions<'a>,
{
    type Ret = Option<()>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .authenticate(&self.user_identifier, &self.password)
            .or_else(|err| Ok(false))
            .and_then(|res| {
                ActionRes::new(
                    "Authenticate",
                if res {
                        Some(())
                    } else {
                        None
                    })
            })


    }
}



/// User Auth: Get All users
#[derive(Debug)]
pub struct GetAllUsers<S = ActionState> {
    phantom_data: PhantomData<(S)>,
}

impl<S> GetAllUsers<S>
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = AllUsersResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .get_all_users()
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .add_user(&self.user)
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .remove_user(&self.user_identifier)
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = InvitationResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let invitation_token = state
            .get_auth_functions()
            .create_user_token(&self.email)
            .map_err(Error::UserManagement)?;

        let invitation = state
            .get_email_sender()
            .send_email(invitation_token)
            .map_err(Error::EmailError)?;

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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .add_user(&self.user)
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
            .modify_user_password(&self.user_identifier, &self.password)
            .or_else(|err| Err(Error::UserManagement(err)))
            .and_then(|res| ActionRes::new("SetUserPassword", UserResult(res)))
    }
}

//TODO: Change user password / image

/// Role Auth: Add Role
#[derive(Debug)]
pub struct AddRole<S = ActionState> {
    role: data::auth::Role,
    phantom_data: PhantomData<(S)>,
}

impl<S> AddRole<S>
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
            .add_role(&self.role)
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = AllRolesResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
            .attach_permission_for_role(&self.permission, &self.rolename)
            .map_err(Error::UserManagement)
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = RoleResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
            .detach_permission_for_role(&self.permission, &self.rolename)
            .map_err(Error::UserManagement)
            .and_then(|res| ActionRes::new("DetachPermissionForRole", RoleResult(res)))
    }
}

/// Role Auth: add role for user
#[derive(Debug)]
pub struct AttachRoleForUser<S = ActionState> {
    rolename: String,
    user_identifier: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> AttachRoleForUser<S>
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    pub fn new(user_identifier: String, rolename: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(rolename.to_owned()),
        ];
        let action = Self {
            rolename,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state
            .get_auth_functions()
            .attach_role_for_user(&self.rolename, &self.user_identifier)
            .map_err(Error::UserManagement)
            .and_then(|res| ActionRes::new("AttachRoleForUser", UserResult(res)))
    }
}

/// Role Auth: remove role for user
#[derive(Debug)]
pub struct DetachRoleForUser<S = ActionState> {
    rolename: String,
    user_identifier: String,
    phantom_data: PhantomData<(S)>,
}

impl<S> DetachRoleForUser<S>
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    pub fn new(user_identifier: String, rolename: String) -> WithPermissionRequired<WithTransaction<Self, S>, S> {
        let required_permissions = vec![
            Permission::user_admin(),
            Permission::has_role(rolename.to_owned()),
        ];
        let action = Self {
            rolename,
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
    where for<'a> S: GetSecrets + GetBroadcaster + StateFunctions<'a>,
{
    type Ret = UserResult;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        state.get_auth_functions()
            .detach_role_for_user(&self.rolename, &self.user_identifier)
            .map_err(Error::UserManagement)
            .and_then(|res| ActionRes::new("DetachRoleForUser", UserResult(res)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use model::actions::results::UserResult;
    use test_common::random_identifier;
    use serde_json::from_value;
    use model::auth::error::UserManagementError;
    use test_common::*;

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
        });
    }


    #[test]
    fn test_add_user_with_display_name() {
        with_state(|state| {
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

    #[test]
    #[ignore]
    fn test_invite_user() {
        with_state(|state| {

            let email = format!("stuff{}@example.com", random_identifier());
            let create_action = InviteUser::<MockState>::new(email);

            let result = create_action.call(&state);
            let data = result.unwrap().get_data();

            println!("data: {:?}", &data);
            unimplemented!()
        });
    }

    #[test]
    fn test_setup_user() {
        with_state(|state| {
            let name = format!("Bobby_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_user: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = SetupUser::<MockState>::new(new_user);

            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();

            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

        });
    }

    #[test]
    fn test_authenticate() {
        with_state(|state| {
            let name = format!("Bobby_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_user: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_user);
            let result = create_action.call(&state);

            let create_action
            = Authenticate::<MockState>::new(name.to_owned(), "hunter2".to_string());
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();
            assert_eq!(data, Some(()));

            let create_action
            = Authenticate::<MockState>::new(name.to_owned(), "wrong_password".to_string());
            let result = create_action.call(&state);
            let data = result.unwrap().get_data();
            assert_eq!(data, None);
        })
    }

    #[test]
    #[ignore]
    fn test_change_user_password() {
        with_state(|state| {
            let name = format!("Bobby_{}", random_identifier());
            let email = format!("stuff{}@example.com", random_identifier());
            let new_user: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_user);
            let result = create_action.call(&state);


            let create_action
            = SetUserPassword::<MockState>::new(name.to_owned(), "AV3ry$ecureP@assword".to_string());
            let result = create_action.call(&state);
            println!("data: {:?}", &result);

            let create_action
            = Authenticate::<MockState>::new(name, "AV3ry$ecureP@assword".to_string());
            let result = create_action.call(&state);
            println!("data: {:?}", &result);

            unimplemented!()
        });
    }

    #[test]
    fn test_add_role() {
        with_state(|state| {
            let rolename = format!("sector7G_{}", random_identifier());
            let role: data::auth::Role = from_value(json!({
                "name": rolename
            })).unwrap();
            let create_action = AddRole::<MockState>::new(role);
            let result = create_action.call(&state);
            let RoleResult(data) = result.unwrap().get_data();
            assert_eq!(data.name, rolename);
        });
    }

    #[test]
    fn test_add_role_remove_role() {
        with_state(|state| {
            let rolename = format!("sector7G_{}", random_identifier());
            let role: data::auth::Role = from_value(json!({
                "name": rolename
            })).unwrap();
            let create_action = AddRole::<MockState>::new(role);
            let result = create_action.call(&state);
            let RoleResult(data) = result.unwrap().get_data();
            assert_eq!(data.name, rolename.to_owned());

            let create_action = RemoveRole::<MockState>::new(rolename.to_owned());
            let result = create_action.call(&state);
            let RoleResult(data) = result.unwrap().get_data();
            assert_eq!(data.name, rolename.to_owned());

            //deleting already deleted
            let create_action = RemoveRole::<MockState>::new(rolename.to_owned());
            let result = create_action.call(&state).unwrap_err();
            assert_eq!(result, Error::UserManagement(UserManagementError::NotFound));
        });
    }

    #[test]
    fn test_get_all_roles() {
        with_state(|state| {
            let id = random_identifier();
            let roles: Vec<data::auth::Role> = vec![
                from_value(json!({"name": format!("A{}", id) })).unwrap(),
                from_value(json!({"name": format!("B{}", id) })).unwrap(),
                from_value(json!({"name": format!("C{}", id) })).unwrap(),
                from_value(json!({"name": format!("D{}", id) })).unwrap(),
            ];
            for role in roles.to_owned() {
                let create_action = AddRole::<MockState>::new(role);
                let result = create_action.call(&state);
            }

            let create_action = GetAllRoles::<MockState>::new();
            let result = create_action.call(&state);
            let AllRolesResult(data) = result.unwrap().get_data();
            let final_rolenames: Vec<String> = data.into_iter().map(|x| x.name).collect();

            for role in roles {
                assert!(final_rolenames.contains(&role.name));
            }
        });
    }

    #[test]
    fn test_attach_permission_for_role() {
        with_state(|state| {
            let id = random_identifier();
            let rolename = format!("a_role{}", id);

            let role = from_value(json!({"name": rolename })).unwrap();
            let create_action = AddRole::<MockState>::new(role);
            let result = create_action.call(&state);

            let permission: data::permissions::Permission = from_value(json!({
                "runScript": {
                    "scriptName": format!("some_script{}", id),
                },
            })).unwrap();
            let create_action
            = AttachPermissionForRole::<MockState>::new(rolename.to_owned(), permission.to_owned());
            let result = create_action.call(&state);
            let RoleResult(data) = result.unwrap().get_data();

            assert_eq!(data.name, rolename);

            let create_action
            = DetachPermissionForRole::<MockState>::new(rolename.to_owned(), permission.to_owned());
            let result = create_action.call(&state);
            let RoleResult(data) = result.unwrap().get_data();

            assert_eq!(data.name, rolename);
        });
    }

    #[test]
    fn test_attach_role_for_user() {
        with_state(|state| {
            let id = random_identifier();

            //add user
            let name = format!("bob_{}", id);
            let email = format!("stuff{}@example.com", random_identifier());
            let new_query: data::auth::NewUser = from_value(json!({
                "username": name,
                "email": email,
                "password": "hunter2"
            })).unwrap();
            let create_action = AddUser::<MockState>::new(new_query);
            let result = create_action.call(&state);

            //add role
            let rolename = format!("a_role{}", id);
            let role = from_value(json!({"name": rolename })).unwrap();
            let create_action = AddRole::<MockState>::new(role);
            let result = create_action.call(&state);

            //attach role for user
            let permission: data::permissions::Permission = from_value(json!({
                "runScript": {
                    "scriptName": format!("some_script{}", id),
                },
            })).unwrap();
            let create_action
            = AttachRoleForUser::<MockState>::new(name.to_owned(), rolename.to_owned());
            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();

            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);

            let create_action
            = DetachRoleForUser::<MockState>::new(name.to_owned(), rolename.to_owned());
            let result = create_action.call(&state);
            let UserResult(data) = result.unwrap().get_data();

            assert_eq!(data.email, email);
            assert_eq!(data.username, name);
            assert_eq!(data.display_name, name);
        });
    }

}