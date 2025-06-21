mod config;
mod function_extractor;
mod handlers;
mod llm_client;
mod models;
mod routes;

use crate::config::AppConfig;
use crate::routes::create_routes;
use log::{error, info};
use poem::listener::TcpListener;
use poem::Server;
use std::process;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            process::exit(1);
        }
    };

    let bind_addr = format!("{}:{}", config.host, config.port);
    let app = create_routes(config);
    let listener = TcpListener::bind(bind_addr.clone());

    info!("Starting server at http://{}", bind_addr);

    Server::new(listener).run(app).await
}
