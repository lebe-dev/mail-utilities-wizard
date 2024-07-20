use std::time::Duration;

use anyhow::anyhow;
use email_type_rs::email::Email;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use log::{error, info};
use non_blank_string_rs::NonBlankString;

use crate::config::mail::MailConfig;

pub trait MailSend {
    fn send_mail(&self, from: &Email, cc: &Email, to: &Email,
                 subject: &NonBlankString, body: &NonBlankString) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct SmtpMailSender {
    config: MailConfig,
}

impl SmtpMailSender {
    pub fn new(config: &MailConfig) -> SmtpMailSender {
        SmtpMailSender {
            config: config.clone()
        }
    }

    fn get_transport_by_host(&self, host: &str, port: u16,
                             creds: Credentials) -> anyhow::Result<SmtpTransport> {
        if self.config.host.as_ref() == "localhost" {
            let transport = SmtpTransport::builder_dangerous(host)
                .port(port).build();
            Ok(transport)

        } else {
            let builder = SmtpTransport::relay(&self.config.host)?;
            let timeout = Duration::new(15, 0);
            let smtp_transport = builder.credentials(creds)
                .port(port)
                .timeout(Some(timeout))
                .build();
            Ok(smtp_transport)
        }
    }
}

impl MailSend for SmtpMailSender {
    fn send_mail(&self, from: &Email, cc: &Email, to: &Email, subject: &NonBlankString, body: &NonBlankString) -> anyhow::Result<()> {
        info!("send mail to '{}'", to.as_ref());
        info!("- subject '{}'", subject);

        match Message::builder()
            .from(from.as_ref().parse()?)
            .cc(cc.as_ref().parse()?)
            .to(to.as_ref().parse()?)
            .subject(subject.to_string())
            .header(header::ContentType::TEXT_PLAIN)
            .body(body.to_string()) {
            Ok(msg) => {
                let creds = Credentials::new(
                    self.config.username.to_string(),
                    self.config.password.to_string(),
                );

                let mailer: SmtpTransport = self.get_transport_by_host(
                    &self.config.host, self.config.port, creds,
                )?;

                match mailer.send(&msg) {
                    Ok(_) => {
                        info!("email sent successfully to '{}'", to.as_ref());
                        Ok(())
                    }
                    Err(e) => {
                        error!("unable to send mail: {:?}", e);
                        Err(anyhow!("unable to send email"))
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(anyhow!("unable to build message for smtp"))
            }
        }
    }
}

#[cfg(test)]
mod smtp_mail_sender_tests {
    use std::str::FromStr;

    use non_blank_string_rs::NonBlankString;

    use crate::config::mail::MailConfig;
    use crate::mail::{MailSend, SmtpMailSender};
    use crate::tests::mail::{get_random_body, get_random_email, get_random_subject};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // TODO: use test containers
    #[ignore]
    #[test]
    fn return_ok_after_send() {
        init();

        let config = get_smtp_config();

        let sender = SmtpMailSender::new(&config);

        let from = get_random_email();
        let to = get_random_email();
        let cc = get_random_email();

        let subject = get_random_subject();
        let body = get_random_body();

        assert!(sender.send_mail(&from, &cc, &to, &subject, &body).is_ok())
    }

    #[test]
    fn return_error_for_unknown_smtp_host() {
        let mut config = get_smtp_config();
        config.host = NonBlankString::from_str("unknown-host").unwrap();

        let sender = SmtpMailSender::new(&config);

        let from = get_random_email();
        let cc = get_random_email();
        let to = get_random_email();

        let subject = get_random_subject();
        let body = get_random_body();

        assert!(sender.send_mail(&from, &cc, &to, &subject, &body).is_err())
    }

    #[test]
    fn return_error_if_unable_to_connect() {
        let mut config = get_smtp_config();
        config.port = 12345;

        let sender = SmtpMailSender::new(&config);

        let from = get_random_email();
        let cc = get_random_email();
        let to = get_random_email();

        let subject = get_random_subject();
        let body = get_random_body();

        assert!(sender.send_mail(&from, &cc, &to, &subject, &body).is_err())
    }

    fn get_smtp_config() -> MailConfig {
        MailConfig {
            from: get_random_email(),
            host: NonBlankString::from_str("localhost").unwrap(),
            port: 1025,
            username: "whatever".to_string(),
            password: "whatever".to_string(),
        }
    }

}

