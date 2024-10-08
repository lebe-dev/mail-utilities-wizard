use crate::state::history::HistoryRecord;
use log::{debug, info};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Row, Sqlite};

pub async fn find_history_record(pool: &Pool<Sqlite>, counter_name: &str,
                 account_id: &str, year: u16, month: &str) -> Result<Option<HistoryRecord>, Error> {
    let select_query = sqlx::query(
        "SELECT * FROM history WHERE counter_name = $1 and account_id = $2 and year = $3 and month = $4");
    let result: Option<HistoryRecord> = select_query
        .bind(&counter_name)
        .bind(&account_id)
        .bind(&year)
        .bind(&month)
        .map(|row: SqliteRow| get_history_record_from_row(&row))
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn find_history_records(pool: &Pool<Sqlite>) -> Result<Vec<HistoryRecord>, Error> {
    info!("find history records..");

    let select_query = sqlx::query("SELECT * FROM history ORDER BY created DESC");
    let results: Vec<HistoryRecord> = select_query
        .map(|row: SqliteRow| get_history_record_from_row(&row))
        .fetch_all(pool)
        .await?;

    debug!("==========[HISTORY RECORDS]==========");
    debug!("{:?}", results);
    debug!("==========[/HISTORY RECORDS]==========");

    Ok(results)
}

fn get_history_record_from_row(row: &SqliteRow) -> HistoryRecord {
    HistoryRecord {
        id: row.get("id"),
        location: row.get("location"),
        account_id: row.get("account_id"),
        counter_name: row.get("counter_name"),
        month: row.get("month"),
        year: row.get("year"),
        value: row.get("value"),
        created: row.get("created")
    }
}

#[cfg(test)]
mod tests {
    use crate::state::find::{find_history_record, find_history_records};
    use crate::state::history::{insert_history_record, HistoryRecord};
    use crate::state::remove::remove_history_record;
    use crate::tests::state::prepare_test_memory_db;
    use crate::tests::{get_random_string, init_logging};
    use chrono::Local;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn crud() {
        init_logging();
        let pool = prepare_test_memory_db().await;

        let record1 = get_random_history_record();

        let record2 = get_random_history_record();

        insert_history_record(&pool, &record1).await.unwrap();
        insert_history_record(&pool, &record2).await.unwrap();

        let results = find_history_records(&pool).await.unwrap();

        let result1 = results.iter().find(|h|h.counter_name == record1.counter_name).unwrap();
        let result2 = results.iter().find(|h|h.counter_name == record2.counter_name).unwrap();

        entities_are_equal_except_id(&record1, result1);
        entities_are_equal_except_id(&record2, result2);

        //

        let result = find_history_record(&pool, &record1.counter_name, &record1.account_id, record1.year, &record1.month).await.unwrap().unwrap();

        assert_eq!(result1, &result);

        //

        remove_history_record(&pool, result1.id).await.unwrap();

        let results = find_history_records(&pool).await.unwrap();

        assert!(
            results.iter().find(|h|h.id == result1.id).is_none()
        );
    }

    #[tokio::test]
    async fn last_items_should_come_first() {
        init_logging();
        let pool = prepare_test_memory_db().await;

        let record1 = get_random_history_record();
        thread::sleep(Duration::from_secs(1));
        let record2 = get_random_history_record();

        insert_history_record(&pool, &record1).await.unwrap();
        insert_history_record(&pool, &record2).await.unwrap();

        let results = find_history_records(&pool).await.unwrap();

        let result = results.first().unwrap();

        assert_eq!(result.location, record2.location);
        assert_eq!(result.counter_name, record2.counter_name);
        assert_eq!(result.value, record2.value);
    }

    fn get_random_history_record() -> HistoryRecord {
        HistoryRecord {
            id: 0,
            location: get_random_string(),
            account_id: get_random_string(),
            counter_name: get_random_string(),
            month: get_random_string(),
            year: 2024,
            value: get_random_string(),
            created: Local::now().timestamp(),
        }
    }

    fn entities_are_equal_except_id(entity1: &HistoryRecord, entity2: &HistoryRecord) {
        assert_eq!(entity1.counter_name, entity2.counter_name);
        assert_eq!(entity1.month, entity2.month);
        assert_eq!(entity1.year, entity2.year);
        assert_eq!(entity1.value, entity2.value);
        assert_eq!(entity1.created, entity2.created);
    }
}