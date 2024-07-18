use std::sync::Arc;

use axum::{Router, routing::get, routing::post};
use axum::http::{header, StatusCode, Uri};
use axum::response::{Html, IntoResponse, Response};
use rust_embed::Embed;

use crate::config::AppConfig;
use crate::config::file::loading_config_from_file;
use crate::logging::get_logging_config;
use crate::route::config::get_config_route;
use crate::route::mail::get_mail_template_route;
use crate::route::version::get_version_route;

pub mod config;

pub mod logging;
pub mod route;
pub mod template;
pub mod mail;

#[cfg(test)]
pub mod tests;

pub const VERSION: &str = "0.1.0 #UNKNOWN";

static INDEX_HTML: &str = "index.html";

#[derive(Clone)]
pub struct AppState {
    config: AppConfig
}

#[tokio::main]
async fn main() {
    let config = loading_config_from_file("config.yml")
                                        .expect("unable to load config");

    let logging_config = get_logging_config(&config.clone().log_level);

    match log4rs::init_config(logging_config) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }

    let app_state = AppState {
        config: config.clone(),
    };

    let app = Router::new()
                        .route("/api/config", get(get_config_route))
                        .route("/api/mail/template", post(get_mail_template_route))
                        .route("/api/version", get(get_version_route))
                        .fallback(static_handler)
                        .with_state(Arc::new(app_state));

    let bind = format!("127.0.0.1:{}", config.port);

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();

    println!("MAIL UTILITIES WIZARD v{VERSION}");
    println!("URL: http://{bind}");

    axum::serve(listener, app).await.unwrap();
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

#[derive(Embed)]
#[folder = "static/"]
struct Assets;