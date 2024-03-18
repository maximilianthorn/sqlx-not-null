use std::fs;

use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Just some shitty .env url parsing
    let conn_str = fs::read_to_string(".env").unwrap();
    let url = &conn_str[..conn_str.len() - 2]
        .splitn(2, "=")
        .last()
        .unwrap()[1..];

    // Set up
    let pool = PgPool::connect(url).await.unwrap();

    // Actual bug. sqlx::query! should throw a compile-time error here.
    sqlx::query!(
        r#"INSERT INTO person (first_name)
        VALUES ($1)"#,
        "John"
    )
    .execute(&pool)
    .await
    .unwrap();
}
