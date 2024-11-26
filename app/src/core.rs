use std::sync::Arc;

use tokio::net::TcpListener;
use utils::config;

use crate::{
    api::route,
    bootstrap::{
        State,
        shutdown::shutdown_signal,
    },
    service::manager::Runnable as _,
};

pub async fn serve() {
    let state = Arc::new(State::init().await);

    State::serve(state.clone()).await;

    api(state.clone()).await;

    // let state1 = state.clone();
    // let api_server = tokio::spawn(async move {
    //     api::Server::init(state1).serve().await;
    // });

    // let state2 = state.clone();
    // let abi_server = tokio::spawn(async move {
    //     abi::Server::init(state2).serve().await;
    // });

    // let _ = tokio::join!(api_server, abi_server);

    state.services.shutdown().await;
}

pub async fn api(state: Arc<State>) {
    let app = route::init(state.clone());
    let host = &config().app.api_host;
    let port = config().app.api_port;
    let listener = TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap_or_else(|e| {
            panic!("💥 Failed to connect bind TcpListener: {e:?}")
        });

    tracing::info!(
        "✨ listening on {}",
        listener.local_addr().unwrap_or_else(|e| panic!(
            "💥 Failed to connect bind TcpListener: {e:?}"
        ))
    );

    // Run the server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| panic!("💥 Failed to start API server: {e:?}"));
}
