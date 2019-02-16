
use actix::prelude::*;
use actix::SyncArbiter;
use futures::Future;

pub struct Executor {
}


impl Actor for Executor {
    type Context = Context<Self>;
}

