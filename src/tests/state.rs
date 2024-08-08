use crate::state::{create_db_tables, get_db_connection};
use sqlx::{Pool, Sqlite};

pub async fn prepare_test_memory_db() -> Pool<Sqlite> {
    let pool = get_db_connection("sqlite::memory:").await.unwrap();
    create_db_tables(&pool).await.unwrap();
    pool
}