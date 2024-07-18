use std::str::FromStr;

use anyhow::anyhow;
use email_type_rs::email::Email;
use fake::Fake;
use fake::faker::internet::en::FreeEmail;
use non_blank_string_rs::NonBlankString;
use non_blank_string_rs::utils::get_random_nonblank_string;

use crate::config::mail::MailConfig;
use crate::mail::MailSend;
use crate::tests::get_random_string;

pub fn get_random_subject() -> NonBlankString {
    NonBlankString::from_str(&format!("MUW test {}", get_random_string())).unwrap()
}

pub fn get_random_body() -> NonBlankString {
    get_random_nonblank_string()
}

pub fn get_random_email() -> Email {
    let random_email: String = FreeEmail().fake();
    Email::from_str(&random_email).unwrap()
}

pub struct MockSmtpMailSender {
    pub smtp_config: MailConfig,
    pub return_error: bool,
    pub expect_from: String,
    pub expect_cc: String,
    pub expect_to: String,
    pub expect_subject: String,
    pub expect_body: String
}

impl MockSmtpMailSender {
    pub fn new(config: &MailConfig, return_error: bool, expect_from: &str, expect_cc: &str, expect_to: &str,
               expect_subject: &str, expect_body: &str) -> MockSmtpMailSender {
        MockSmtpMailSender {
            smtp_config: config.clone(),
            return_error,
            expect_from: expect_from.to_string(),
            expect_cc: expect_cc.to_string(),
            expect_to: expect_to.to_string(),
            expect_subject: expect_subject.to_string(),
            expect_body: expect_body.to_string()
        }
    }
}

impl MailSend for MockSmtpMailSender {
    fn send_mail(&self, from: &Email, cc: &Email, to: &Email, subject: &NonBlankString, body: &NonBlankString) -> anyhow::Result<()> {
        assert_eq!(self.expect_from, from.as_ref());
        assert_eq!(self.expect_cc, cc.as_ref());
        assert_eq!(self.expect_to, to.as_ref());
        assert_eq!(self.expect_subject, subject.to_string());
        assert_eq!(self.expect_body, body.to_string());

        if !self.return_error {
            Ok(())

        } else {
            Err(anyhow!("mock smtp mail sender returned error as expected"))
        }

    }
}
