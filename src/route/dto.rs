use serde::Serialize;

use crate::config::location::Location;
use crate::config::page::PageConfig;

#[derive(PartialEq, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigDto {
    pub page_config: PageConfig,
    pub locations: Vec<Location>
}