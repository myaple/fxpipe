mod function_extractor;
mod handlers;
mod llm_client;
mod routes;
mod config;

use crate::config::AppConfig;
use log::{info, error};
use std::process;

use poem::web::Json;
use poem::{Route, Server, get, handler, listener::TcpListener, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ModelsResponse {
    data: Vec<Model>,
}

#[derive(Serialize, Deserialize)]
struct Model {
    id: String,
}

#[handler]
pub async fn get_models() -> Json<ModelsResponse> {
    let models = vec![
        Model {
            id: "fx-small".to_string(),
        },
    ];
    Json(ModelsResponse { data: models })
}

#[derive(Serialize, Deserialize)]
struct ChatCompletionRequest {
    // This should match the OpenAI chat completion request structure.
    // For now, accept a minimal structure for demonstration
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    choices: Vec<Choice>,
}

#[derive(Serialize)]
struct Choice {
    index: usize,
    message: MessageResponse,
}

#[derive(Serialize)]
struct MessageResponse {
    role: String,
    content: String,
}

#[handler]
async fn chat_completions(Json(_req): Json<ChatCompletionRequest>) -> Json<ChatCompletionResponse> {
    // Stub implementation: respond with a fixed response for now
    let response = ChatCompletionResponse {
        id: "chatcmpl-123".to_string(),
        object: "chat.completion".to_string(),
        created: 1234567890,
        choices: vec![Choice {
            index: 0,
            message: MessageResponse {
                role: "assistant".to_string(),
                content: "This is a stub response.".to_string(),
            },
        }],
    };
    Json(response)
}


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

    let app = Route::new()
        .at("/v1/models", get(get_models))
        .at("/v1/chat/completions", post(chat_completions));

    let bind_addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(bind_addr.clone());

    info!("Starting server at http://{}", bind_addr);

    Server::new(listener).run(app).await
}

