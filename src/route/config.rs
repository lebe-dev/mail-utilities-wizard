use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::AppState;
use crate::route::dto::AppConfigDto;

pub async fn get_config_route(State(state): State<Arc<AppState>>) -> Json<AppConfigDto> {
    let config = AppConfigDto {
        page_config: state.config.page.clone(),
        locations: state.config.locations.clone(),
    };

    Json(config)
}