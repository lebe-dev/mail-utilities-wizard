use email_type_rs::email::Email;
use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    pub name: NonBlankString,
    #[serde(alias = "account-id", alias = "accountId")]
    pub account_id: NonBlankString,
    pub email: Email,
    #[serde(alias = "email-copy", alias = "emailCopy")]
    #[serde(default)]
    pub email_copy: String,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub signature: String,
}