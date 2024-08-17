use serde::Serialize;

use crate::config::location::Location;
use crate::config::page::PageConfig;
use crate::state::history::HistoryRecord;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub page: PageConfig,
    pub locations: Vec<Location>,
    pub history_records: Vec<HistoryRecord>
}

#[derive(PartialEq, Serialize, Clone, Debug)]
pub struct MailTemplateDto {
    pub subject: String,
    pub body: String
}