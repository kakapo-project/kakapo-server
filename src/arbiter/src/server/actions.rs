
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::pg::PgConnection;


pub type State = PooledConnection<ConnectionManager<PgConnection>>;

/// Used for converting values received from the action into json format
pub trait Serializable {
    const ACTION_NAME: &'static str = "NoAction";
    fn into_serialize(self) -> serde_json::Value;
}

#[derive(Debug, Fail, Serialize)]
pub enum Error {
    #[fail(display = "An unknown error occurred")]
    Unknown,
}

pub trait Action<S = State>: Send
    where
        Self::Ret: Send
{
    type Ret;
    fn call(&self, state: &S) -> Result<Self::Ret, Error>;
}