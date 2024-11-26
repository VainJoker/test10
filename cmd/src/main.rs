#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    utils::config::init("./config/config.toml");
    utils::logger::init();
    tracing::info!("Application started");
    app::serve().await;
    tracing::info!("Application stopped");
}
