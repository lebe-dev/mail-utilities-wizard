use serde::Serialize;

use crate::config::locale::LocaleConfig;
use crate::config::location::Location;
use crate::state::history::HistoryRecord;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub locale: LocaleConfig,
    pub locations: Vec<Location>,
    pub auth_enabled: bool,
    pub history_records: Vec<HistoryRecord>
}

#[derive(PartialEq, Serialize, Clone, Debug)]
pub struct MailTemplateDto {
    pub subject: String,
    pub body: String
}