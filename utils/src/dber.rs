use sqlx::{
    PgPool,
    postgres::PgPoolOptions,
};

use crate::config::config;

pub type DB = PgPool;

pub struct Dber {
    pub pool: DB,
}

impl Dber {
    pub async fn init() -> Self {
        let database_url = &config().database.url;
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
        {
            Ok(pool) => {
                tracing::info!("🚀 Connection to the database is successful!");
                Self { pool }
            }
            Err(err) => {
                panic!("💥 Failed to connect to the database: {err:?}");
            }
        }
    }
}
