use log::info;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Error, Pool, Sqlite};

pub mod history;
pub mod find;

pub async fn get_db_connection(db_cnn: &str) -> Result<Pool<Sqlite>, Error> {
    info!("connecting to database '{db_cnn}'..");
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(db_cnn)
        .await
}

pub async fn create_db_tables(pool: &Pool<Sqlite>) -> Result<(), Error> {
    info!("creating tables in db..");

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS history (
  id INTEGER NOT NULL PRIMARY KEY,
  counter_name VARCHAR,
  month VARCHAR,
  year INTEGER,
  value VARCHAR,
  created INTEGER,
  UNIQUE(counter_name, month, year)
);"#,
    ).execute(pool)
        .await?;

    Ok(())
}