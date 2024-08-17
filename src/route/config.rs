use std::sync::Arc;

use crate::route::dto::AppConfigDto;
use crate::state::find::find_history_records;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use log::error;

pub async fn get_config_route(State(state): State<Arc<AppState>>) -> impl IntoResponse {

    match find_history_records(&state.db_pool).await {
        Ok(history_records) => {
            let config = AppConfigDto {
                page: state.config.page.clone(),
                locations: state.config.locations.clone(),
                history_records
            };

            (StatusCode::OK, Json(config)).into_response()
        }
        Err(e) => {
            error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}