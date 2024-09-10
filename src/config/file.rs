use std::str::FromStr;

use anyhow::Context;
use config::Config;
use email_type_rs::email::Email;
use log::info;

use crate::config::auth::AuthConfig;
use crate::config::counter::Counter;
use crate::config::location::Location;
use crate::config::AppConfig;

pub fn loading_config_from_file(config_file_path: &str, locale_file_path: &str) -> anyhow::Result<AppConfig> {
    info!("loading config from files: '{config_file_path}', {locale_file_path}");

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file_path))
        .add_source(config::File::with_name(locale_file_path))
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
        db_cnn: loaded_config.db_cnn,
        log_level: loaded_config.log_level,
        auth: AuthConfig {
            enabled: loaded_config.auth.enabled,
            password: loaded_config.auth.password,
            secret: loaded_config.auth.secret,
        },
        locations,
        defaults: loaded_config.defaults,
        locale: loaded_config.locale,
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

    use crate::config::auth::AuthConfig;
    use crate::config::counter::Counter;
    use crate::config::defaults::DefaultsConfig;
    use crate::config::file::loading_config_from_file;
    use crate::config::locale::LocaleConfig;
    use crate::config::location::Location;
    use crate::config::mail::MailConfig;
    use crate::config::AppConfig;

    #[test]
    fn config_load() {
        let config_file = Path::new("test-data").join("test-config.yml");
        let config_file = format!("{}", config_file.display());
        let locale_file = Path::new("test-data").join("test-locale.yml");
        let locale_file = format!("{}", locale_file.display());
        let result_config = loading_config_from_file(&config_file, &locale_file).unwrap();

        let expected_config = AppConfig {
            bind: "127.0.0.1:8080".parse().unwrap(),
            db_cnn: "sqlite://app.db?mode=rwc".parse().unwrap(),
            log_level: "debug".parse().unwrap(),
            auth: AuthConfig {
                enabled: true,
                password: "h246346rGw3g545".parse().unwrap(),
                secret: "23FS0932fdfSD".parse().unwrap(),
            },
            locations: vec![
                Location {
                    name: "Saint Petersburg, Nevsky Street, 123".parse().unwrap(),
                    counters: vec![
                        Counter {
                            name: "Water".parse().unwrap(),
                            account_id: "568346545734".parse().unwrap(),
                            email: "water@company2.com".to_string(),
                            email_copy: "relative@mail.com".to_string(),
                            url: "".to_string(),
                            manual: "".to_string(),
                            mail_subject_template: "Counter {{ counter_name }} data for {{ month }} 2".to_string(),
                            mail_body_template_file: "custom-template.txt".to_string(),
                            signature: "Boris Britva".to_string(),
                        },
                        Counter {
                            name: "Electricity".parse().unwrap(),
                            account_id: "85678463456".parse().unwrap(),
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
                mail_subject_template: "Counter {{ counter_name }} data for {{ month }}".parse().unwrap(),
                mail_body_template_file: "example.txt".parse().unwrap(),
                signature: "Evgeny Lebedev".parse().unwrap(),
            },
            locale: LocaleConfig {
                language: "en".parse().unwrap(),

                title: "Mail Utilities Wizard".parse().unwrap(),
                header: "Send utilities data".parse().unwrap(),

                login_text: "Your password:".parse().unwrap(),
                login_button: "Enter".parse().unwrap(),
                login_error_msg: "Invalid password".parse().unwrap(),

                select_location_label: "Select location:".parse().unwrap(),
                select_location_dropdown: "- Select -".parse().unwrap(),

                select_counter_label: "Select counter:".parse().unwrap(),
                select_counter_dropdown: "- Select -".parse().unwrap(),

                account_id_label: "Your account id:".parse().unwrap(),
                account_id_hint: "Unique for each counter".parse().unwrap(),

                email_label: "Data will be sent to e-mail:".parse().unwrap(),

                period_label: "Period:".parse().unwrap(),

                counter_value_label: "Counter value:".parse().unwrap(),

                mail_template_title: "Letter".parse().unwrap(),
                mail_template_to_label: "To:".parse().unwrap(),
                mail_template_subject_label: "Subject:".parse().unwrap(),
                mail_template_body_label: "Body:".parse().unwrap(),
                mail_template_close_button: "Close".parse().unwrap(),

                already_sent_warn_msg: "Data for this meter has already been sent for the selected period.".parse().unwrap(),

                send_confirm_msg: "Do you want to continue?".parse().unwrap(),

                send_button: "Send".parse().unwrap(),
                show_letter_button: "Show the letter".parse().unwrap(),
                send_more_button: "Send more".parse().unwrap(),
                back_button: "Back".parse().unwrap(),

                app_error_msg: "Application error".parse().unwrap(),

                history_table_text: "History for previous periods:".parse().unwrap(),
                history_record_date: "Date:".parse().unwrap(),
                history_record_values: "Values:".parse().unwrap(),
                history_record_counter: "Counter:".parse().unwrap(),
                history_record_location: "Location:".parse().unwrap(),
                history_record_period: "Period:".parse().unwrap(),
                history_record_value: "Value:".parse().unwrap(),

                sending_msg: "Sending..".parse().unwrap(),
                send_success_msg: "Counter value has been sent".parse().unwrap(),
                send_error_msg: "Unable to send counter data, contact john@company.com".parse().unwrap(),
            },
            mail: MailConfig {
                from: "eugene@mail.com".parse().unwrap(),
                host: "localhost".parse().unwrap(),
                port: 1025,
                username: "change-me".to_string(),
                password: "change-me".to_string(),
            },
        };

        assert_eq!(result_config, expected_config);
    }
}