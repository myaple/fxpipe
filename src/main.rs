mod function_extractor;
mod handlers;
mod llm_client;
mod routes;

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
            id: "gpt-4".to_string(),
        },
        Model {
            id: "gpt-3.5-turbo".to_string(),
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
    let app = Route::new()
        .at("/v1/models", get(get_models))
        .at("/v1/chat/completions", post(chat_completions));

    let listener = TcpListener::bind("127.0.0.1:3000");
    println!("Starting server at http://127.0.0.1:3000");
    Server::new(listener).run(app).await
}
