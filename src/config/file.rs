use config::Config;
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
            let mut template = counter.template.clone();
            let mut signature = counter.signature.clone();

            if counter.template.is_empty() {
                template = loaded_config.defaults.template.to_string();
            }

            if counter.signature.is_empty() {
                signature = loaded_config.defaults.signature.to_string();
            }

            counters.push(
                Counter {
                    name: counter.name,
                    account_id: counter.account_id,
                    email: counter.email,
                    template,
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
        port: loaded_config.port,
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
            port: 8080,
            log_level: NonBlankString::from_str("debug").unwrap(),
            locations: vec![
                Location {
                    name: NonBlankString::from_str("Saint Petersburg, Nevsky Street, 123").unwrap(),
                    counters: vec![
                        Counter {
                            name: NonBlankString::from_str("Electricity").unwrap(),
                            account_id: NonBlankString::from_str("85678463456").unwrap(),
                            email: Email::from_str("electricity@company1.com").unwrap(),
                            template: "example.txt".to_string(),
                            signature: "Evgeny Lebedev".to_string(),
                        },
                        Counter {
                            name: NonBlankString::from_str("Water").unwrap(),
                            account_id: NonBlankString::from_str("568346545734").unwrap(),
                            email: Email::from_str("water@company2.com").unwrap(),
                            template: "custom-template.txt".to_string(),
                            signature: "Boris Britva".to_string(),
                        }
                    ]
                }
            ],
            defaults: DefaultsConfig {
                template: NonBlankString::from_str("example.txt").unwrap(),
                signature: NonBlankString::from_str("Evgeny Lebedev").unwrap(),
            },
            page: PageConfig {
                title: NonBlankString::from_str("Mail Utilities Wizard").unwrap(),
                header: NonBlankString::from_str("Send utilities data").unwrap(),

                select_location_label: NonBlankString::from_str("Select location:").unwrap(),
                select_location_dropdown: NonBlankString::from_str("- Select -").unwrap(),

                select_counter_label: NonBlankString::from_str("Select counter:").unwrap(),
                select_counter_dropdown: NonBlankString::from_str("- Select -").unwrap(),

                account_id_label: NonBlankString::from_str("Your account id:").unwrap(),
                account_id_placeholder: NonBlankString::from_str("1234567").unwrap(),

                email_label: NonBlankString::from_str("Data will be sent to e-mail:").unwrap(),

                counter_value_label: NonBlankString::from_str("Counter value:").unwrap(),
                counter_default_value: NonBlankString::from_str("0").unwrap(),

                send_confirm_msg: NonBlankString::from_str("Do you want to continue?").unwrap(),
                send_confirm_yes: NonBlankString::from_str("Yes").unwrap(),
                send_confirm_no: NonBlankString::from_str("No").unwrap(),

                send_button: NonBlankString::from_str("Send").unwrap(),
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