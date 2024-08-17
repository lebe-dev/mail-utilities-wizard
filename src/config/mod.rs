use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

use crate::config::defaults::DefaultsConfig;
use crate::config::locale::LocaleConfig;
use crate::config::location::Location;
use crate::config::mail::MailConfig;

pub mod location;
pub mod defaults;
pub mod mail;
pub mod counter;
pub mod file;
pub mod locale;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub bind: NonBlankString,
    pub db_cnn: NonBlankString,
    pub log_level: NonBlankString,
    pub locations: Vec<Location>,
    pub defaults: DefaultsConfig,
    pub locale: LocaleConfig,
    pub mail: MailConfig
}