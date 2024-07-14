use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct MailConfig {
    pub from: NonBlankString,
    pub host: NonBlankString,
    pub port: u16,
    pub username: String,
    pub password: String,
}