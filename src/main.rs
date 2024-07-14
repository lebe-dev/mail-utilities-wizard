use axum::{
    Router, routing::get,
};

use crate::config::file::loading_config_from_file;
use crate::logging::get_logging_config;
use crate::route::version::get_version_route;

pub mod config;

pub mod logging;
pub mod route;

pub const VERSION: &str = "0.1.0 #UNKNOWN";

#[tokio::main]
async fn main() {
    let config = loading_config_from_file("config.yml")
                                        .expect("unable to load config");

    let logging_config = get_logging_config(&config.log_level);

    match log4rs::init_config(logging_config) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e)
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/api/version", get(get_version_route));

    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{}", config.port)
    ).await.unwrap();

    println!("MAIL UTILITIES WIZARD v{VERSION}");
    println!("URL: http://127.0.0.1:{}", config.port);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "UNDER DEVELOPMENT"
}