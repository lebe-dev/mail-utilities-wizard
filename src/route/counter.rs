use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use email_type_rs::email::Email;
use log::{error, info};
use non_blank_string_rs::NonBlankString;

use crate::AppState;
use crate::mail::{MailSend, SmtpMailSender};
use crate::route::mail::CounterData;
use crate::template::{get_template_vars, render_mail_body_template, render_mail_subject_template};

pub async fn send_counter_data_route(State(state): State<Arc<AppState>>,
                                     Json(request): Json<CounterData>) -> StatusCode {
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

                    match render_mail_subject_template(&counter.mail_subject_template, &template_vars) {
                        Ok(mail_subject) => {

                            match render_mail_body_template(&template_file, &template_vars) {
                                Ok(mail_body) => {

                                    let sender = SmtpMailSender::new(&state.config.mail);

                                    let to = Email::from_str(&counter.email).unwrap();
                                    let cc = Email::from_str(&counter.email_copy).unwrap();

                                    match sender.send_mail(
                                        &state.config.mail.from,
                                        &cc, &to,
                                        &NonBlankString::from_str(&mail_subject).unwrap(),
                                        &NonBlankString::from_str(&mail_body).unwrap()
                                    ) {
                                        Ok(_) => {
                                            info!("counter data has been sent");
                                            StatusCode::OK
                                        }
                                        Err(e) => {
                                            error!("counter data send error: {}", e);
                                            StatusCode::INTERNAL_SERVER_ERROR
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("mail body template render error: {}", e);
                                    StatusCode::INTERNAL_SERVER_ERROR
                                }
                            }

                        },
                        Err(e) => {
                            error!("mail subject template render error: {}", e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        }
                    }
                }
                None => {
                    error!("counter wasn't found by name '{}'", request.counter_name);
                    StatusCode::BAD_REQUEST
                }
            }

        }
        None => {
            error!("location wasn't found by name '{}'", request.location_name);
            StatusCode::BAD_REQUEST
        }
    }
}