use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

use crate::config::defaults::DefaultsConfig;
use crate::config::location::Location;
use crate::config::mail::MailConfig;
use crate::config::page::PageConfig;

pub mod location;
pub mod defaults;
pub mod mail;
pub mod counter;
pub mod file;
pub mod page;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub port: u16,
    pub log_level: NonBlankString,
    pub locations: Vec<Location>,
    pub defaults: DefaultsConfig,
    pub page: PageConfig,
    pub mail: MailConfig
}