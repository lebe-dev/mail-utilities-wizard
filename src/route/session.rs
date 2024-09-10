use crate::auth::create_session_token;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::{error, info};
use non_blank_string_rs::NonBlankString;
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};

pub const SESSION_COOKIE_NAME: &str = "MUW_SESSION";

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: NonBlankString
}

pub async fn get_user_session_route(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
) -> StatusCode {

    if state.config.auth.enabled {
        if state.config.auth.password == request.password {
            match create_session_token(&state.config.auth.secret, 3600) {
                Ok(token) => {
                    let cookie = get_session_cookie_with_token(token.to_string());
                    cookies.add(cookie);
                    StatusCode::OK
                }
                Err(e) => {
                    error!("unable to create session token: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }

        } else {
            info!("invalid password");
            StatusCode::UNAUTHORIZED
        }

    } else {
        info!("authentication is not enabled, check config");
        StatusCode::BAD_REQUEST
    }
}

fn get_session_cookie_with_token<'a>(token: String) -> Cookie<'a> {
    let mut cookie = Cookie::new(SESSION_COOKIE_NAME, token.to_string());
    cookie.set_path("/");
    cookie.set_secure(false);
    cookie.set_http_only(false);
    cookie
}