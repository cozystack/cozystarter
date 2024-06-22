use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config::Settings;

pub async fn create_db_pool(settings: Settings) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(settings.database_url.as_str())
        .await
}
