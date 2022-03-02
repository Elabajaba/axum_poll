use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub async fn db() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not set");
    let connection_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Error parsing DATABASE_URL")
        .create_if_missing(true);
    SqlitePool::connect_with(connection_options).await.unwrap()
}
