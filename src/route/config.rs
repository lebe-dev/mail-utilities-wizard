use std::sync::Arc;

use crate::route::auth::is_request_permitted;
use crate::route::dto::AppConfigDto;
use crate::state::find::find_history_records;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;
use tower_cookies::Cookies;

pub async fn get_config_route(State(state): State<Arc<AppState>>,
                              cookies: Cookies) -> impl IntoResponse {

    if is_request_permitted(state.config.auth.enabled, &state.config.auth.secret.as_ref(), &cookies) {
        match find_history_records(&state.db_pool).await {
            Ok(history_records) => {
                let config = AppConfigDto {
                    locale: state.config.locale.clone(),
                    auth_enabled: state.config.auth.enabled,
                    locations: state.config.locations.clone(),
                    history_records
                };

                (StatusCode::OK, Json(config)).into_response()
            }
            Err(e) => {
                error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }

    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn get_locale_config_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.config.locale.clone())).into_response()
}