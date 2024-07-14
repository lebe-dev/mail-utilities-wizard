use serde::Deserialize;

use crate::config::defaults::DefaultsConfig;
use crate::config::location::Location;
use crate::config::mail::MailConfig;

pub mod location;
pub mod defaults;
pub mod mail;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub locations: Vec<Location>,
    pub defaults_config: DefaultsConfig,
    pub mail_config: MailConfig
}