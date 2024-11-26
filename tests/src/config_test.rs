#[test]
fn test_config() {
    std::env::set_var(
        "DATABASE_URL",
        "postgres://postgres:postgres@localhost:5432/postgres",
    );
    std::env::set_var("RABBITMQ_URL", "amqp://guest:guest@localhost:5672/%2f");
    std::env::set_var("DRAGONFLY_URL", "http://localhost:8080");
    utils::config::init("../config/config.toml");
    let config = utils::config();
    println!("{config:#?}");
}
