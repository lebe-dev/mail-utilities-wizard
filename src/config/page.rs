use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PageConfig {
    pub title: NonBlankString,
    pub header: NonBlankString,

    #[serde(alias = "select-location-label", alias = "sendLocationLabel")]
    pub select_location_label: NonBlankString,

    #[serde(alias = "select-location-dropdown", alias = "selectLocationDropdown")]
    pub select_location_dropdown: NonBlankString,

    #[serde(alias = "select-counter-label", alias = "sendCounterLabel")]
    pub select_counter_label: NonBlankString,

    #[serde(alias = "select-counter-dropdown", alias = "selectCounterDropdown")]
    pub select_counter_dropdown: NonBlankString,

    #[serde(alias = "account-id-label", alias = "accountIdLabel")]
    pub account_id_label: NonBlankString,

    #[serde(alias = "account-id-placeholder", alias = "accountIdPlaceholder")]
    pub account_id_placeholder: NonBlankString,

    #[serde(alias = "email-label", alias = "emailLabel")]
    pub email_label: NonBlankString,

    #[serde(alias = "period-label", alias = "periodLabel")]
    pub period_label: NonBlankString,

    #[serde(alias = "counter-value-label", alias = "counterValueLabel")]
    pub counter_value_label: NonBlankString,

    #[serde(alias = "counter-default-value", alias = "counterDefaultValue")]
    pub counter_default_value: NonBlankString,

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

    #[serde(alias = "app-error-msg", alias = "appErrorMsg")]
    pub app_error_msg: NonBlankString,

    #[serde(alias = "sending-msg", alias = "sendingMsg")]
    pub sending_msg: NonBlankString,

    #[serde(alias = "send-success-msg", alias = "sendSuccessMsg")]
    pub send_success_msg: NonBlankString,

    #[serde(alias = "send-error-msg", alias = "sendErrorMsg")]
    pub send_error_msg: NonBlankString,
}