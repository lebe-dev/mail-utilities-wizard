use std::path::Path;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::error;
use non_blank_string_rs::NonBlankString;
use serde::Deserialize;

use crate::AppState;
use crate::route::dto::MailTemplateDto;
use crate::template::{get_template_vars, render_mail_body_template};

pub async fn get_mail_template_route(State(state): State<Arc<AppState>>,
                                     Json(request): Json<CounterData>) -> Result<Json<MailTemplateDto>, StatusCode> {
    let location = state.config.locations.iter()
                                            .find(|l| l.name == request.location_name);

    match location {
        Some(location) => {
            let counter = location.counters.iter()
                                                .find(|c| c.name == request.counter_name);

            match counter {
                Some(counter) => {
                    let template_vars = get_template_vars(
                        &location.name, &request.month, &counter, &request.counter_value
                    );

                    let template_file = Path::new("templates").join(&counter.mail_body_template_file);
                    let template_file = format!("{}", template_file.display());

                    match render_mail_body_template(&template_file, &template_vars) {
                        Ok(template) => {
                            let template = MailTemplateDto { template };
                            Ok(Json(template))

                        }
                        Err(e) => {
                            error!("mail template render error: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                None => {
                    error!("counter wasn't found by name '{}'", request.counter_name);
                    Err(StatusCode::BAD_REQUEST)
                }
            }

        }
        None => {
            error!("location wasn't found by name '{}'", request.location_name);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CounterData {
    pub location_name: NonBlankString,
    pub counter_name: NonBlankString,
    pub month: NonBlankString,
    pub counter_value: NonBlankString
}
