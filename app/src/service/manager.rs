use std::sync::Arc;

use crate::bootstrap::State;

#[derive(Clone)]
pub struct Services {
    // pub exchange_rate: exchange_rate::Server,
    // pub miner_stat: miner_stat::Server,
    // pub message_queue: message_queue::Server,
    // pub mqtt: mqtt_service::Server,
}

pub trait Runnable {
    async fn init() -> Self;
    async fn serve(&mut self, state: Arc<State>);
    async fn shutdown(&self);
}

impl Runnable for Services {
    async fn init() -> Self {
        Self {}
    }

    async fn serve(&mut self, state: Arc<State>) {}

    async fn shutdown(&self) {}
}
