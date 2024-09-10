use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct AuthConfig {
    pub enabled: bool,
    pub password: NonBlankString,
    pub secret: NonBlankString
}