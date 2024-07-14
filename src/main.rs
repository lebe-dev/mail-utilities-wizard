use axum::{
    Router, routing::get,
};
use serde::{Deserialize, Serialize};

use crate::config::file::loading_config_from_file;
use crate::logging::get_logging_config;

pub mod config;

pub mod logging;

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
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind(
        format!("127.0.0.1:{}", config.port)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "MAIL UTILITIES WIZARD v0.1.0"
}