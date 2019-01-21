
use actix::prelude::*;

use connection::executor::DatabaseExecutor;
use actix::dev::MessageResponse;

use model::actions::Action;
use model::state::State;
use model::actions::ActionResult;
use scripting::Scripting;


pub struct ActionWrapper<A: Action + Send> {
    action: A,
}

impl<A: Action + Send> ActionWrapper<A> {
    pub fn new(action: A) -> Self {
        Self {
            action,
        }
    }
}

impl<A: Action + Send> Message for ActionWrapper<A>
    where
        A::Ret: 'static,
        ActionResult<A::Ret>: 'static,
{
    type Result = ActionResult<A::Ret>;
}

impl<A: Action + Send> Handler<ActionWrapper<A>> for DatabaseExecutor
    where
        A::Ret: 'static,
        ActionResult<A::Ret>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
{
    type Result = ActionResult<A::Ret>;

    fn handle(&mut self, msg: ActionWrapper<A>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        let scripting = Scripting::new(self.get_scripts_path());
        let state = State::new(conn, scripting);
        let result = msg.action.call(&state);
        result
    }
}