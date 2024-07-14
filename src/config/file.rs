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
        mail: loaded_config.mail,
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
                            email: Email::from_str("electricity@company1.com").unwrap(),
                            template: "example.txt".to_string(),
                            signature: "Evgeny Lebedev".to_string(),
                        },
                        Counter {
                            name: NonBlankString::from_str("Water").unwrap(),
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
            mail: MailConfig {
                from: NonBlankString::from_str("eugene@mail.com").unwrap(),
                host: NonBlankString::from_str("localhost").unwrap(),
                port: 1025,
                username: "change-me".to_string(),
                password: "change-me".to_string(),
            },
        };

        assert_eq!(result_config, expected_config);
    }
}