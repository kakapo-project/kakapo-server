
pub mod results;
pub mod error;

mod user_actions;
mod entity_actions;
mod table_actions;
mod query_actions;
mod script_actions;

pub use self::user_actions::*;
pub use self::entity_actions::*;
pub use self::table_actions::*;
pub use self::query_actions::*;
pub use self::script_actions::*;

mod decorator;

use actix::prelude::*;

use serde_json;

use std::result::Result;
use std::result::Result::Ok;
use std::marker::PhantomData;

use data;

use connection::py::PyRunner;

use model::entity;
use model::entity::RetrieverFunctions;
use model::entity::ModifierFunctions;
use model::entity::error::EntityError;

use data::schema;

use model::actions::results::*;
use model::actions::error::Error;
use data::utils::OnDuplicate;

use data::utils::OnNotFound;
use data::conversion;
use data::dbdata::RawEntityTypes;

use model::entity::results::Upserted;
use model::entity::results::Created;
use model::entity::results::Updated;
use model::entity::results::Deleted;
use data::utils::TableDataFormat;

use model::table;
use model::table::TableActionFunctions;
use model::query;
use model::query::QueryActionFunctions;
use model::script;
use model::script::ScriptActionFunctions;

use connection::executor::Conn;
use model::state::State;
use model::state::GetConnection;
use model::state::Channels;
use model::auth::permissions::*;
use std::iter::FromIterator;

use model::actions::decorator::*;
use std::fmt::Debug;

use model::auth::Auth;
use model::auth::AuthFunctions;
use serde::Serialize;


#[derive(Debug, Clone)]
pub struct OkAction<R> {
    name: String,
    data: R,
}

impl<R> OkAction<R>
    where R: Send,
{

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_data_ref(&self) -> &R {
        &self.data
    }

    pub fn get_data(self) -> R {
        self.data
    }
}

pub type ActionResult<R> = Result<OkAction<R>, Error>;

pub struct ActionRes;
impl ActionRes {
    pub fn new<R>(name: &str, data: R) -> ActionResult<R>
        where R: Send
    {
        Ok(OkAction { name: name.to_string(), data })
    }

}

pub trait Action<S = State>
    where
        Self: Send,
        Self::Ret: Send + Debug + Serialize,
{
    type Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret>;
}


