use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    pub name: NonBlankString,

    #[serde(alias = "account-id", alias = "accountId")]
    pub account_id: NonBlankString,

    #[serde(default)]
    pub email: String,

    #[serde(alias = "email-copy", alias = "emailCopy")]
    #[serde(default)]
    pub email_copy: String,

    #[serde(default)]
    pub url: String,

    #[serde(default)]
    pub manual: String,

    #[serde(alias = "mail-subject-template", alias = "mailSubjectTemplate")]
    #[serde(default)]
    pub mail_subject_template: String,

    #[serde(alias = "mail-body-template-file", alias = "mailBodyTemplateFile")]
    #[serde(default)]
    pub mail_body_template_file: String,

    #[serde(default)]
    pub signature: String,
}