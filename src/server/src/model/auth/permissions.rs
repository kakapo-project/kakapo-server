use model::state::State;
use model::state::ChannelBroadcaster;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadTableInfo(String),
}

pub struct AuthPermissions;

pub trait AuthPermissionFunctions<B> //TODO: the ChannelBroadcast shouldn't be here
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn get_permissions(state: &State<B>) -> HashSet<Permission>;
}

impl<B> AuthPermissionFunctions<B> for AuthPermissions
    where
        B: ChannelBroadcaster + Send + 'static,
{
    fn get_permissions(state: &State<B>) -> HashSet<Permission> {
        unimplemented!()
    }
}