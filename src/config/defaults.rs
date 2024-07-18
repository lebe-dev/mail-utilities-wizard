use email_type_rs::email::Email;
use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DefaultsConfig {
    pub email_copy: Email,
    pub mail_subject_template: NonBlankString,
    pub mail_body_template_file: NonBlankString,
    pub signature: NonBlankString
}