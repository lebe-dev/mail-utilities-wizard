use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Counter {
    pub name: NonBlankString,
    pub email: NonBlankString,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub signature: String,
}