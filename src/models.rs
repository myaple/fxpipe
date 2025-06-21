use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Object)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

#[derive(Serialize, Deserialize, Object)]
pub struct Model {
    pub id: String,
}

#[derive(Serialize, Deserialize, Object)]
pub struct ChatCompletionRequest {
    // This should match the OpenAI chat completion request structure.
    // For now, accept a minimal structure for demonstration
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Object)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Object)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Object)]
pub struct Choice {
    pub index: usize,
    pub message: MessageResponse,
}

#[derive(Serialize, Deserialize, Object)]
pub struct MessageResponse {
    pub role: String,
    pub content: String,
}
