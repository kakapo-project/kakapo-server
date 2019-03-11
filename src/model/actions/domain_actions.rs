
use std::marker::PhantomData;

use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::Named;
use data::channels::Channels;
use data::permissions::*;

use model::actions::results::*;
use model::actions::error::Error;
use model::actions::decorator::*;
use model::actions::Action;
use model::actions::ActionRes;
use model::actions::ActionResult;

use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::RawEntityTypes;
use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use model::entity::update_state::UpdateActionFunctions;

use state::StateFunctions;
use state::ActionState;
use state::authorization::AuthorizationOps;
use data::DomainInfo;
use state::domain_management::DomainManagementOps;

///get all tables
#[derive(Debug, Clone)]
pub struct GetAllDomains<S = ActionState> {
    pub phantom_data: PhantomData<(S)>,
}

impl<S> GetAllDomains<S>
    where for<'a> S: StateFunctions<'a>,
{
    pub fn new() -> WithLoginRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action = WithTransaction::new(action);
        let action = WithLoginRequired::new(action);

        action
    }
}

impl<S> Action<S> for GetAllDomains<S>
    where for<'a> S: StateFunctions<'a>,
{
    type Ret = Vec<DomainInfo>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        let data = state
            .get_domain_management()
            .get_all_domains()
            .map_err(|err| Error::DomainManagement(err))?;

        ActionRes::new("GetAllEntities", data)
    }
}

#[derive(Debug, Clone)]
pub struct ModifyDomain<S = ActionState> {
    pub phantom_data: PhantomData<(S)>,
}

impl<S> ModifyDomain<S>
    where for<'a> S: StateFunctions<'a>,
{
    pub fn new() -> WithLoginRequired<WithTransaction<Self, S>, S> {
        let action = Self {
            phantom_data: PhantomData,
        };

        let action = WithTransaction::new(action);
        let action = WithLoginRequired::new(action); //TODO: shouldn't be login

        action
    }
}

impl<S> Action<S> for ModifyDomain<S>
    where for<'a> S: StateFunctions<'a>,
{
    type Ret = Option<i32>;
    fn call(&self, state: &S) -> ActionResult<Self::Ret> {
        //TODO: fix this
        ActionRes::new("ModifyDomain", None)
    }
}
