use crate::route::auth::is_request_permitted;
use crate::state::find::find_history_record;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use email_type_rs::email::Email;
use log::{error, info};
use non_blank_string_rs::NonBlankString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_cookies::Cookies;

#[derive(Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub struct CheckRequest {
    pub email: Email,
    pub account_id: NonBlankString,
    pub year: u16,
    pub month: NonBlankString,
}

#[derive(Serialize,Debug)]
pub struct CheckResponse {
    pub exist: bool
}

pub async fn check_if_already_sent_route(State(state): State<Arc<AppState>>,
                                         cookies: Cookies,
                                         Json(request): Json<CheckRequest>) -> impl IntoResponse {

    if is_request_permitted(state.config.auth.enabled, &state.config.auth.secret.as_ref(), &cookies) {

        info!("check if counter data already sent");

        info!("request: {:?}", request);

        let mut exist = false;

        for location in &state.config.locations {
            if let Some(counter) = location.counters.iter()
                .find(|c|
                    c.account_id == request.account_id && c.email == request.email.as_ref()) {
                info!("counter has been found: {:?}", counter);

                match find_history_record(&state.db_pool, &counter.name,
                                          &request.account_id, request.year, &request.month).await {
                    Ok(history_record) => {
                        match history_record {
                            Some(_) => {
                                info!("history record exists");
                                exist = true;
                            }
                            None => info!("history record wasn't found")
                        }

                        break
                    }
                    Err(e) => {
                        error!("{}", e);
                        return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                    }
                }
            }
        }

        (StatusCode::OK, Json(CheckResponse { exist })).into_response()

    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}