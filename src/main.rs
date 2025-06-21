use poem::{listener::TcpListener, Server};

mod config;
mod function_extractor;
mod handlers;
mod llm_client;
mod models;
mod routes;

use routes::create_routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = crate::config::AppConfig::from_env().unwrap();
    let app = create_routes(&config);
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.host, config.port
    )))
    .run(app)
    .await
}
