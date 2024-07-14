use config::Config;
use log::info;

use crate::config::AppConfig;

pub fn loading_config_from_file(config_file_path: &str) -> anyhow::Result<AppConfig> {
    info!("loading config from file '{config_file_path}'");

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file_path))
        .build()?;

    let config = settings.try_deserialize::<AppConfig>()?;

    info!("config:");
    info!("{:?}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::str::FromStr;

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
                            email: NonBlankString::from_str("utilities@company.com").unwrap(),
                            template: "".to_string(),
                            signature: "".to_string(),
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