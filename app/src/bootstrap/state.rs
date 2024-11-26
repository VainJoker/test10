use std::sync::Arc;

use utils::Dber;

use crate::service::manager::{
    Runnable as _,
    Services,
};

pub struct State {
    pub db:       Dber,
    // pub redis: Redisor,
    pub services: Services,
}

impl State {
    pub async fn init() -> Self {
        Self {
            db:       Dber::init().await,
            services: Services::init().await,
        }
    }

    pub async fn serve(self: Arc<Self>) {
        // self.services.serve(self).await;
    }
}
