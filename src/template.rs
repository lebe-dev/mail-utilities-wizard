use std::collections::HashMap;

use log::{debug, info};
use tera::{Context, Tera};

use crate::config::counter::Counter;

pub const LOCATION_NAME_TEMPLATE_VAR: &str = "location";
pub const COUNTER_NAME_TEMPLATE_VAR: &str = "counter_name";
pub const COUNTER_ACCOUNT_ID_TEMPLATE_VAR: &str = "account_id";
pub const COUNTER_VALUE_TEMPLATE_VAR: &str = "counter_value";
pub const YEAR_TEMPLATE_VAR: &str = "year";
pub const MONTH_TEMPLATE_VAR: &str = "month";
pub const SIGNATURE_TEMPLATE_VAR: &str = "signature";

pub fn get_template_vars(location_name: &str, year: u16, month: &str, counter: &Counter,
                         counter_value: &str) -> HashMap<String,String> {
    HashMap::from([
        (LOCATION_NAME_TEMPLATE_VAR.to_string(), location_name.to_string()),
        (YEAR_TEMPLATE_VAR.to_string(), year.to_string()),
        (MONTH_TEMPLATE_VAR.to_string(), month.to_string()),
        (COUNTER_NAME_TEMPLATE_VAR.to_string(), counter.name.to_string()),
        (COUNTER_ACCOUNT_ID_TEMPLATE_VAR.to_string(), counter.account_id.to_string()),
        (COUNTER_VALUE_TEMPLATE_VAR.to_string(), counter_value.to_string()),
        (SIGNATURE_TEMPLATE_VAR.to_string(), counter.signature.to_string()),
    ])
}

pub fn render_mail_body_template(template_file: &str,
                                 template_vars: &HashMap<String,String>) -> anyhow::Result<String> {

    let mut tera = Tera::default();
    tera.add_template_file(&template_file, Some("mail"))?;

    let mut context = Context::new();

    for (key, value) in template_vars {
        context.insert(key, &value);
    }

    let result = tera.render("mail", &context)?;

    debug!("---[RENDERED MAIL BODY TEMPLATE]---");
    debug!("{result}");
    debug!("---[/RENDERED MAIL BODY TEMPLATE]---");

    info!("mail body template has been rendered");

    Ok(result)
}

pub fn render_mail_subject_template(template: &str,
                                    template_vars: &HashMap<String,String>) -> anyhow::Result<String> {

    let mut tera = Tera::default();
    tera.add_raw_template("subject", template)?;

    let mut context = Context::new();

    for (key, value) in template_vars {
        context.insert(key, &value);
    }

    let result = tera.render("subject", &context)?;

    debug!("---[RENDERED MAIL SUBJECT TEMPLATE]---");
    debug!("{result}");
    debug!("---[/RENDERED MAIL SUBJECT TEMPLATE]---");

    info!("mail subject template has been rendered");

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    use non_blank_string_rs::NonBlankString;

    use crate::config::counter::Counter;
    use crate::template::{get_template_vars, render_mail_body_template};

    #[test]
    fn template_render() {
        let template_file_path = Path::new("test-data").join("template.txt");
        let template_file_path = format!("{}", template_file_path.display());

        let counter = Counter {
            name: NonBlankString::from_str("Electricity").unwrap(),
            account_id: NonBlankString::from_str("92358457293485").unwrap(),
            email: "whatever@company.com".to_string(),
            email_copy: "whatever@company.com".to_string(),
            url: "".to_string(),
            manual: "".to_string(),
            mail_subject_template: "whatever".to_string(),
            mail_body_template_file: "whatever".to_string(),
            signature: "Evgeny Lebedev".to_string(),
        };

        let vars = get_template_vars("Saint Petersburg, Nevsky Street, 123", 2024, "July", &counter, "123");

        let result_template = render_mail_body_template(&template_file_path, &vars).unwrap();

        let rendered_template = Path::new("test-data").join("rendered-template.txt");
        let expected_template = fs::read_to_string(rendered_template).unwrap();

        assert_eq!(expected_template, result_template);
    }
}