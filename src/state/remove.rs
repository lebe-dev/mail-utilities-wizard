use log::info;
use sqlx::{Error, Pool, Sqlite};

pub async fn remove_history_record(pool: &Pool<Sqlite>,
                                   record_id: u32) -> Result<(), Error> {
    info!("remove history record by id {record_id}");

    sqlx::query(r#"DELETE FROM history WHERE id=$1;"#)
        .bind(record_id)
        .execute(pool)
        .await?;

    Ok(())
}