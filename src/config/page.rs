use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageConfig {
    pub title: NonBlankString,
    pub header: NonBlankString,

    #[serde(alias = "select-location-text", alias = "sendLocationText")]
    pub select_location_text: NonBlankString,

    #[serde(alias = "select-location-dropdown", alias = "selectLocationDropdown")]
    pub select_location_dropdown: NonBlankString,

    #[serde(alias = "account-id-label", alias = "accountIdLabel")]
    pub account_id_label: NonBlankString,

    #[serde(alias = "account-id-placeholder", alias = "accountIdPlaceholder")]
    pub account_id_placeholder: NonBlankString,

    #[serde(alias = "email-label", alias = "emailLabel")]
    pub email_label: NonBlankString,

    #[serde(alias = "send-confirm-msg", alias = "sendConfirmMsg")]
    pub send_confirm_msg: NonBlankString,

    #[serde(alias = "send-confirm-yes", alias = "sendConfirmYes")]
    pub send_confirm_yes: NonBlankString,

    #[serde(alias = "send-confirm-no", alias = "sendConfirmNo")]
    pub send_confirm_no: NonBlankString,

    #[serde(alias = "send-button", alias = "sendButton")]
    pub send_button: NonBlankString,

    #[serde(alias = "send-more-button", alias = "sendMoreButton")]
    pub send_more_button: NonBlankString,

    #[serde(alias = "loading-msg", alias = "loadingMsg")]
    pub loading_msg: NonBlankString,

    #[serde(alias = "send-success-msg", alias = "sendSuccessMsg")]
    pub send_success_msg: NonBlankString,

    #[serde(alias = "send-error-msg", alias = "sendErrorMsg")]
    pub send_error_msg: NonBlankString,
}