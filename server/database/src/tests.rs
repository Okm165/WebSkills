use sqlx::PgPool;

#[tokio::test]
async fn test_connect_to_db() {
    let _database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();
}
