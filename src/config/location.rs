use email_type_rs::email::Email;
use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Location {
    pub name: NonBlankString,
    pub email: Email,
    pub template: String,
    pub signature: String,
}