use serde::Serialize;

use crate::config::location::Location;
use crate::config::page::PageConfig;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub page: PageConfig,
    pub locations: Vec<Location>
}

#[derive(PartialEq, Serialize, Clone, Debug)]
pub struct MailTemplateDto {
    pub template: String
}