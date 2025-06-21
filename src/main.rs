mod config;
mod function_extractor;
mod handlers;
mod llm_client;
mod routes;

use crate::config::AppConfig;
use log::{error, info};
use std::process;

use poem::web::Json;
use poem::{get, handler, listener::TcpListener, post, Route, Server};
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
    let models = vec![Model {
        id: "fx-small".to_string(),
    }];
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

use crate::llm_client::call_upstream_llm;

#[handler]
async fn chat_completions(Json(req): Json<ChatCompletionRequest>) -> Json<ChatCompletionResponse> {
    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(_) => {
            return Json(ChatCompletionResponse {
                id: "error".to_string(),
                object: "error".to_string(),
                created: 0,
                choices: vec![],
            });
        }
    };

    let payload = serde_json::to_string(&req).unwrap_or_else(|_| "{}".to_string());

    // Call the upstream LLM with the configured LLM config
    let result = call_upstream_llm(&payload, &config.llm_config).await;

    match result {
        Ok(text) => {
            // Simplified response: just echo the raw response text in content field
            Json(ChatCompletionResponse {
                id: "chatcmpl-123".to_string(),
                object: "chat.completion".to_string(),
                created: 0,
                choices: vec![Choice {
                    index: 0,
                    message: MessageResponse {
                        role: "assistant".to_string(),
                        content: text,
                    },
                }],
            })
        }
        Err(_) => Json(ChatCompletionResponse {
            id: "error".to_string(),
            object: "error".to_string(),
            created: 0,
            choices: vec![],
        }),
    }
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
