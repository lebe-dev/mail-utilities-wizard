use log::info;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Sqlite};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecord {
    pub id: u32,
    pub location: String,
    pub account_id: String,
    pub counter_name: String,
    pub month: String,
    pub year: u16,
    pub value: String,
    pub created: i64,
}

pub async fn insert_history_record(pool: &Pool<Sqlite>,
                                   record: &HistoryRecord) -> Result<(), Error> {
    info!("insert history record '{:?}'", record);

    sqlx::query(r#"INSERT INTO history (location, account_id, counter_name, month, year, value, created) VALUES ($1, $2, $3, $4, $5, $6, $7);"#)
        .bind(&record.location)
        .bind(&record.account_id)
        .bind(&record.counter_name)
        .bind(&record.month)
        .bind(&record.year)
        .bind(&record.value)
        .bind(&record.created)
        .execute(pool)
        .await?;

    Ok(())
}