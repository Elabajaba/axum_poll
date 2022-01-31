use sqlx::sqlite::SqlitePool;

pub async fn db() -> SqlitePool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL env not be setted");
    SqlitePool::connect(&database_url).await.unwrap()
}