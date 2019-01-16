
use actix::prelude::*;

use actix_web::{
    AsyncResponder, Error as ActixError,
    dev::Handler as MsgHandler, http,
    FromRequest, Json,
    HttpRequest, HttpResponse, ws,
};

use serde_json;
use actix::dev::MessageResponse;

use actix_web::middleware::cors::CorsBuilder;
use futures::Future;


use server::state::AppState;
use server::actions;
use server::actions::Action;
use futures::Async;


use diesel::pg::PgConnection;

use actix::prelude::*;
use diesel::{r2d2::ConnectionManager, r2d2::PooledConnection};
use diesel::r2d2::Pool;
use actix::sync::SyncArbiter;
use num_cpus;

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DatabaseExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl DatabaseExecutor {
    pub fn get_connection(&self) -> Conn {
        self.0.get()
            .expect("Could not get connection")
    }
}

impl Actor for DatabaseExecutor {
    type Context = SyncContext<Self>;
}

pub struct Connector {
    host_name: Option<String>,
    port_name: Option<u16>,
    user_name: Option<String>,
    pass_name: Option<String>,
    db_name: Option<String>,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            host_name: None,
            port_name: None,
            user_name: None,
            pass_name: None,
            db_name: None,
        }
    }

    pub fn host(mut self, param: String) -> Self {
        self.host_name = Some(param);
        self
    }

    pub fn port(mut self, param: u16) -> Self {
        self.port_name = Some(param);
        self
    }

    pub fn user(mut self, param: String) -> Self {
        self.user_name = Some(param);
        self
    }

    pub fn pass(mut self, param: String) -> Self {
        self.pass_name = Some(param);
        self
    }

    pub fn db(mut self, param: String) -> Self {
        self.db_name = Some(param);
        self
    }

    pub fn done(mut self) -> Addr<DatabaseExecutor> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user_name.unwrap_or_default(),
            self.pass_name.unwrap_or_default(),
            self.host_name.unwrap_or_default(),
            self.port_name.unwrap_or_default(),
            self.db_name.unwrap_or_default(),
        );
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Could not start connection");

        SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()))
    }
}


pub struct ActionWrapper<A: Action + Send> {
    action: A
}

impl<A: Action + Send> ActionWrapper<A> {
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A: Action + Send> Message for ActionWrapper<A>
    where
        A::Ret: 'static,
        Result<A::Ret, actions::Error>: 'static,
{
    type Result = Result<A::Ret, actions::Error>;
}

impl<A: Action + Send> Handler<ActionWrapper<A>> for DatabaseExecutor
    where
        A::Ret: 'static,
        Result<A::Ret, actions::Error>: MessageResponse<DatabaseExecutor, ActionWrapper<A>> + 'static,
{
    type Result = Result<A::Ret, actions::Error>;

    fn handle(&mut self, msg: ActionWrapper<A>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection();
        let result = msg.action.call(&conn);
        result
    }
}