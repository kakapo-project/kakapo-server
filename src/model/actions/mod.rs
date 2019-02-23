
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

use std::result::Result;
use std::result::Result::Ok;
use model::actions::error::Error;
use model::state::ActionState;
use std::fmt::Debug;

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

#[derive(Debug, Clone)]
pub struct ActionRes;
impl ActionRes {
    pub fn new<R>(name: &str, data: R) -> ActionResult<R>
        where R: Send
    {
        Ok(OkAction { name: name.to_string(), data })
    }

}

pub trait Action<S = ActionState>
    where
        Self: Send + Debug,
        Self::Ret: Send + Debug + Serialize,
{
    type Ret;
    fn call(&self, state: &S) -> ActionResult<Self::Ret>;
}

