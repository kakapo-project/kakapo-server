
pub mod error;

use Channels;
use serde::Serialize;
use model::broadcast::error::BroadcastError;

#[derive(Clone, Debug)]
pub struct Broadcaster {}

pub trait BroadcasterOps {
    fn publish<R>(&self, channels: Vec<Channels>, action_name: String, action_result: &R) -> Result<(), BroadcastError>
        where R: Serialize;
}

impl BroadcasterOps for Broadcaster {
    fn publish<R>(&self, channels: Vec<Channels>, action_name: String, action_result: &R) -> Result<(), BroadcastError>
        where R: Serialize
    {
        let payload = serde_json::to_value(action_result)
            .map_err(|err| BroadcastError::SerializationError)?;

        //TODO: broadcast

        Ok(())
    }
}