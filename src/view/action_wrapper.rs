
use actix::prelude::*;

use connection::executor::Executor;
use actix::dev::MessageResponse;

use model::actions::Action;
use model::state::ActionState;
use data::claims::AuthClaims;
use model::actions::ActionResult;
use model::actions::error::Error;
use scripting::Scripting;
use std::sync::Arc;
use std::str;
use jsonwebtoken;
use std::fmt;

const BEARER: &'static str = "Bearer ";

pub struct ActionWrapper<A: Action> {
    action: Result<A, serde_json::Error>,
    auth_header: Option<Vec<u8>>,
}

impl<A> fmt::Debug for ActionWrapper<A>
    where A: Action,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.action)
    }
}

impl<A: Action + Send> ActionWrapper<A> {
    pub fn new(auth_header: Option<&[u8]>, action: Result<A, serde_json::Error>) -> Self {
        Self {
            action,
            auth_header: auth_header.map(|x| x.to_owned()),
        }
    }

    fn parse_bearer_token(data: String) -> Option<String> {
        let is_bearer = data.starts_with(BEARER);
        if !is_bearer {
            error!("must be a Bearer token");
            None
        } else {
            let (_, token_str) = data.split_at(BEARER.len());

            Some(token_str.to_string())
        }
    }

    fn decode_token(&self, token_secret: String) -> Option<AuthClaims> {
        let auth_header = self.auth_header.to_owned();

        auth_header
            .and_then(|bytes| str::from_utf8(&bytes).ok().map(|x| x.to_string()))
            .and_then(|data| Self::parse_bearer_token(data))
            .and_then(|auth| {
                let decoded = jsonwebtoken::decode::<AuthClaims>(
                    &auth,
                    token_secret.as_ref(),
                    &jsonwebtoken::Validation::default());

                match decoded {
                    Ok(x) => Some(x),
                    Err(err) => {
                        error!("encountered error trying to decode token: {:?}", &err);
                        None
                    }
                }
            })
            .and_then(|token_data| Some(token_data.claims))
    }

    fn get_action(self) -> Result<A, Error> {
        self.action.or_else(|err| Err(Error::SerializationError(err.to_string())))
    }
}

impl<A: Action + Send> Message for ActionWrapper<A>
    where
        A::Ret: 'static,
        ActionResult<A::Ret>: 'static,
{
    type Result = ActionResult<A::Ret>;
}

impl<A: Action + Send> Handler<ActionWrapper<A>> for Executor
    where
        A::Ret: 'static,
        ActionResult<A::Ret>: MessageResponse<Executor, ActionWrapper<A>> + 'static,
{
    type Result = ActionResult<A::Ret>;

    fn handle(&mut self, msg: ActionWrapper<A>, _: &mut Self::Context) -> Self::Result {
        debug!("handling call : {:?}", &msg);

        let auth_claims = msg.decode_token(self.get_token_secret());

        let action_req = msg.get_action()?;

        let conn = self.get_connection();
        let scripting = Scripting::new(self.get_scripts_path());
        let secrets = self.get_secrets();


        let state = ActionState::new(conn, scripting, auth_claims, secrets);
        let result = action_req.call(&state);
        debug!("action result: {:?}", &result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use connection::AppStateBuilder;
    use connection::AppState;
    use connection::AppStateLike;
    use futures::Future;
    use model::actions::ActionRes;
    use data::channels::Channels;

    #[derive(Debug, Clone)]
    struct TestAction;
    impl<S> Action<S> for TestAction {
        type Ret = String;

        fn call(&self, state: &S) -> ActionResult<Self::Ret> {
            ActionRes::new("TestAction", "Hello World!".to_string())
        }
    }

    fn mock_executor() -> AppState {
        AppStateBuilder::new()
            .host("localhost")
            .port(5432)
            .user("test")
            .pass("password")
            .num_threads(1)
            .done()
    }

    #[test]
    fn test_handle_action() {
        actix::System::run(|| {
            let executor = mock_executor();
            let action = TestAction;

            let f = executor
                .connect()
                .send(ActionWrapper::new(None, Ok(action)))
                .map_err(|_| ())
                .map(|res| {
                    assert_eq!(res.unwrap().get_data(), "Hello World!");

                    actix::System::current().stop();
                });

            tokio::spawn(f);
        });
    }

    #[test]
    fn test_parse_bearer_token() {
        let input = "Bearer MY_🐻_TOKEN_HERE";
        let output = ActionWrapper::<TestAction>::parse_bearer_token(input.to_string());

        assert_eq!(output.unwrap(), "MY_🐻_TOKEN_HERE");

        let input = "Basic usename_and_password_here";
        let output = ActionWrapper::<TestAction>::parse_bearer_token(input.to_string());

        assert_eq!(output, None);

        let input = "..";
        let output = ActionWrapper::<TestAction>::parse_bearer_token(input.to_string());

        assert_eq!(output, None);
    }
}