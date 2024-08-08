use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,PartialEq,Clone,Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoryRecord {
    pub id: u32,
    pub counter_name: String,
    pub month: String,
    pub year: u16,
    pub value: String,
    pub created: i64,
}