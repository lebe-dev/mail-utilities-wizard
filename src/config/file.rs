use std::str::FromStr;

use anyhow::Context;
use config::Config;
use email_type_rs::email::Email;
use log::info;

use crate::config::AppConfig;
use crate::config::counter::Counter;
use crate::config::location::Location;

pub fn loading_config_from_file(config_file_path: &str) -> anyhow::Result<AppConfig> {
    info!("loading config from file '{config_file_path}'");

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file_path))
        .build()?;

    let loaded_config = settings.try_deserialize::<AppConfig>()?;

    let mut locations: Vec<Location> = vec![];

    for location in loaded_config.locations {
        let mut counters: Vec<Counter> = vec![];

        for counter in location.counters {
            let mut email_copy = counter.email_copy.clone();
            let mut mail_subject_template = counter.mail_subject_template.clone();
            let mut mail_body_template_file = counter.mail_body_template_file.clone();
            let mut signature = counter.signature.clone();

            if counter.email_copy.is_empty() {
                email_copy = loaded_config.defaults.email_copy.to_string();

            } else {
                Email::from_str(&counter.email_copy).context("invalid email-copy value in counter configuration")?;
            }

            if counter.mail_subject_template.is_empty() {
                mail_subject_template = loaded_config.defaults.mail_subject_template.to_string();
            }

            if counter.mail_body_template_file.is_empty() {
                mail_body_template_file = loaded_config.defaults.mail_body_template_file.to_string();
            }

            if counter.signature.is_empty() {
                signature = loaded_config.defaults.signature.to_string();
            }

            counters.push(
                Counter {
                    name: counter.name,
                    account_id: counter.account_id,
                    email: counter.email,
                    email_copy,
                    url: counter.url,
                    manual: counter.manual,
                    mail_subject_template,
                    mail_body_template_file,
                    signature,
                }
            )
        }

        locations.push(
            Location {
                name: location.name,
                counters,
            }
        )
    }

    let config = AppConfig {
        bind: loaded_config.bind,
        log_level: loaded_config.log_level,
        locations,
        defaults: loaded_config.defaults,
        page: loaded_config.page,
        mail: loaded_config.mail
    };

    info!("config:");
    info!("{:?}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;

    use email_type_rs::email::Email;
    use non_blank_string_rs::NonBlankString;

    use crate::config::AppConfig;
    use crate::config::counter::Counter;
    use crate::config::defaults::DefaultsConfig;
    use crate::config::file::loading_config_from_file;
    use crate::config::location::Location;
    use crate::config::mail::MailConfig;
    use crate::config::page::PageConfig;

    #[test]
    fn config_load() {
        let config_file = Path::new("test-data").join("test-config.yml");
        let config_file = format!("{}", config_file.display());
        let result_config = loading_config_from_file(&config_file).unwrap();

        let expected_config = AppConfig {
            bind: NonBlankString::from_str("127.0.0.1:8080").unwrap(),
            log_level: NonBlankString::from_str("debug").unwrap(),
            locations: vec![
                Location {
                    name: NonBlankString::from_str("Saint Petersburg, Nevsky Street, 123").unwrap(),
                    counters: vec![
                        Counter {
                            name: NonBlankString::from_str("Water").unwrap(),
                            account_id: NonBlankString::from_str("568346545734").unwrap(),
                            email: "water@company2.com".to_string(),
                            email_copy: "relative@mail.com".to_string(),
                            url: "".to_string(),
                            manual: "".to_string(),
                            mail_subject_template: "Counter {{ counter_name }} data for {{ month }} 2".to_string(),
                            mail_body_template_file: "custom-template.txt".to_string(),
                            signature: "Boris Britva".to_string(),
                        },
                        Counter {
                            name: NonBlankString::from_str("Electricity").unwrap(),
                            account_id: NonBlankString::from_str("85678463456").unwrap(),
                            email: "".to_string(),
                            email_copy: "default@mail.com".to_string(),
                            url: "https://metrics.company.com/population/send-and-pay/?from=main-menu".to_string(),
                            manual: "Electricity provider requires to send counter data via web site. Use account-id to log-in.".to_string(),
                            mail_subject_template: "Counter {{ counter_name }} data for {{ month }}".to_string(),
                            mail_body_template_file: "example.txt".to_string(),
                            signature: "Evgeny Lebedev".to_string(),
                        }
                    ]
                }
            ],
            defaults: DefaultsConfig {
                email_copy: Email::from_str("default@mail.com").unwrap(),
                mail_subject_template: NonBlankString::from_str("Counter {{ counter_name }} data for {{ month }}").unwrap(),
                mail_body_template_file: NonBlankString::from_str("example.txt").unwrap(),
                signature: NonBlankString::from_str("Evgeny Lebedev").unwrap(),
            },
            page: PageConfig {
                locale: NonBlankString::from_str("en").unwrap(),

                title: NonBlankString::from_str("Mail Utilities Wizard").unwrap(),
                header: NonBlankString::from_str("Send utilities data").unwrap(),

                select_location_label: NonBlankString::from_str("Select location:").unwrap(),
                select_location_dropdown: NonBlankString::from_str("- Select -").unwrap(),

                select_counter_label: NonBlankString::from_str("Select counter:").unwrap(),
                select_counter_dropdown: NonBlankString::from_str("- Select -").unwrap(),

                account_id_label: NonBlankString::from_str("Your account id:").unwrap(),
                account_id_hint: NonBlankString::from_str("Unique for each counter").unwrap(),

                email_label: NonBlankString::from_str("Data will be sent to e-mail:").unwrap(),

                period_label: NonBlankString::from_str("Period:").unwrap(),

                counter_value_label: NonBlankString::from_str("Counter value:").unwrap(),

                mail_template_title: NonBlankString::from_str("Letter").unwrap(),
                mail_template_to_label: NonBlankString::from_str("To:").unwrap(),
                mail_template_subject_label: NonBlankString::from_str("Subject:").unwrap(),
                mail_template_body_label: NonBlankString::from_str("Body:").unwrap(),
                mail_template_close_button: NonBlankString::from_str("Close").unwrap(),

                send_confirm_msg: NonBlankString::from_str("Do you want to continue?").unwrap(),

                send_button: NonBlankString::from_str("Send").unwrap(),
                show_letter_button: NonBlankString::from_str("Show the letter").unwrap(),
                send_more_button: NonBlankString::from_str("Send more").unwrap(),

                app_error_msg: NonBlankString::from_str("Application error").unwrap(),

                sending_msg: NonBlankString::from_str("Sending..").unwrap(),
                send_success_msg: NonBlankString::from_str("Counter value has been sent").unwrap(),
                send_error_msg: NonBlankString::from_str("Unable to send counter data, contact john@company.com").unwrap(),
            },
            mail: MailConfig {
                from: Email::from_str("eugene@mail.com").unwrap(),
                host: NonBlankString::from_str("localhost").unwrap(),
                port: 1025,
                username: "change-me".to_string(),
                password: "change-me".to_string(),
            },
        };

        assert_eq!(result_config, expected_config);
    }
}