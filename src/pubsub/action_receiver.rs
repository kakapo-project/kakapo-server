use model::state::PubSubOps;
use view::action_wrapper::PublishCallback;
use data::channels::Channels;
use model::state::error::BroadcastError;

impl PubSubOps for PublishCallback {
    fn publish(&self, channels: Vec<Channels>, action_name: String, action_result: &serde_json::Value) -> Result<(), BroadcastError> {
        info!("publishing: to channels: {:?}", &channels);
        debug!("publishing results: {:?} => {:?}", &action_name, &action_result);
        //TODO: ...
        Ok(())
    }

    fn subscribe(&self, channels: Vec<Channels>) -> Result<(), BroadcastError> {
        info!("subscribing: to channels: {:?}", &channels);
        //TODO: ...
        Ok(())
    }
}