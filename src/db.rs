use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_db_pool(url: String) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .expect("can't connect to database")
}
