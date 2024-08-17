use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocaleConfig {
    // COMMON
    pub language: NonBlankString,

    pub title: NonBlankString,
    pub header: NonBlankString,

    #[serde(alias = "app-error-msg", alias = "appErrorMsg")]
    pub app_error_msg: NonBlankString,

    // MAIN PAGE

    #[serde(alias = "history-table-text", alias = "historyTableText")]
    pub history_table_text: NonBlankString,

    #[serde(alias = "history-record-date", alias = "historyRecordDate")]
    pub history_record_date: NonBlankString,

    #[serde(alias = "history-record-values", alias = "historyRecordValues")]
    pub history_record_values: NonBlankString,

    #[serde(alias = "history-record-counter", alias = "historyRecordCounter")]
    pub history_record_counter: NonBlankString,

    #[serde(alias = "history-record-location", alias = "historyRecordLocation")]
    pub history_record_location: NonBlankString,

    #[serde(alias = "history-record-period", alias = "historyRecordPeriod")]
    pub history_record_period: NonBlankString,

    #[serde(alias = "history-record-value", alias = "historyRecordValue")]
    pub history_record_value: NonBlankString,

    // SEND PAGE

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

    #[serde(alias = "account-id-hint", alias = "accountIdHint")]
    pub account_id_hint: NonBlankString,

    #[serde(alias = "email-label", alias = "emailLabel")]
    pub email_label: NonBlankString,

    #[serde(alias = "period-label", alias = "periodLabel")]
    pub period_label: NonBlankString,

    #[serde(alias = "counter-value-label", alias = "counterValueLabel")]
    pub counter_value_label: NonBlankString,

    #[serde(alias = "mail-template-title", alias = "mailTemplateTitle")]
    pub mail_template_title: NonBlankString,

    #[serde(alias = "mail-template-to-label", alias = "mailTemplateToLabel")]
    pub mail_template_to_label: NonBlankString,

    #[serde(alias = "mail-template-subject-label", alias = "mailTemplateSubjectLabel")]
    pub mail_template_subject_label: NonBlankString,

    #[serde(alias = "mail-template-body-label", alias = "mailTemplateBodyLabel")]
    pub mail_template_body_label: NonBlankString,

    #[serde(alias = "mail-template-close-button", alias = "mailTemplateCloseButton")]
    pub mail_template_close_button: NonBlankString,

    #[serde(alias = "send-confirm-msg", alias = "sendConfirmMsg")]
    pub send_confirm_msg: NonBlankString,

    #[serde(alias = "send-button", alias = "sendButton")]
    pub send_button: NonBlankString,

    #[serde(alias = "show-letter-button", alias = "showLetterButton")]
    pub show_letter_button: NonBlankString,

    #[serde(alias = "back-button", alias = "backButton")]
    pub back_button: NonBlankString,

    #[serde(alias = "sending-msg", alias = "sendingMsg")]
    pub sending_msg: NonBlankString,

    #[serde(alias = "send-error-msg", alias = "sendErrorMsg")]
    pub send_error_msg: NonBlankString,

    // SEND SUCCESS PAGE
    #[serde(alias = "send-success-msg", alias = "sendSuccessMsg")]
    pub send_success_msg: NonBlankString,

    #[serde(alias = "send-more-button", alias = "sendMoreButton")]
    pub send_more_button: NonBlankString,
}