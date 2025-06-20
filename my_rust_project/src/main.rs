use poem::{Route, Server, listener::TcpListener};
use poem_openapi::payload::Json;
// Removed: use poem_openapi::ui::SwaggerUi;
// The specific type might not be needed if api_service.swagger_ui() provides the service directly.
use poem_openapi::{Object, OpenApi, OpenApiService};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Model {
    #[oai(skip_serializing_if_is_none)]
    pub id: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub object: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub created: Option<i64>,
    #[oai(skip_serializing_if_is_none)]
    pub owned_by: Option<String>,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct ChatCompletionRequest {
    #[oai(skip_serializing_if_is_none)]
    pub model: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub messages: Option<Vec<ChatMessage>>,
    #[oai(skip_serializing_if_is_none)]
    pub temperature: Option<f32>,
    #[oai(skip_serializing_if_is_none)]
    pub top_p: Option<f32>,
    #[oai(skip_serializing_if_is_none)]
    pub n: Option<i32>,
    #[oai(skip_serializing_if_is_none)]
    pub stream: Option<bool>,
    #[oai(skip_serializing_if_is_none)]
    pub stop: Option<Vec<String>>,
    #[oai(skip_serializing_if_is_none)]
    pub max_tokens: Option<i32>,
    #[oai(skip_serializing_if_is_none)]
    pub presence_penalty: Option<f32>,
    #[oai(skip_serializing_if_is_none)]
    pub frequency_penalty: Option<f32>,
    #[oai(skip_serializing_if_is_none)]
    pub logit_bias: Option<HashMap<String, i32>>,
    #[oai(skip_serializing_if_is_none)]
    pub user: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub functions: Option<Vec<Function>>,
    #[oai(skip_serializing_if_is_none)]
    pub function_call: Option<FunctionCall>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct ChatCompletionResponse {
    #[oai(skip_serializing_if_is_none)]
    pub id: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub object: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub created: Option<i64>,
    #[oai(skip_serializing_if_is_none)]
    pub model: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub choices: Option<Vec<ChatCompletionChoice>>,
    #[oai(skip_serializing_if_is_none)]
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct ChatMessage {
    #[oai(skip_serializing_if_is_none)]
    pub role: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub content: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub name: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub function_call: Option<FunctionCall>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct ChatCompletionChoice {
    #[oai(skip_serializing_if_is_none)]
    pub index: Option<i32>,
    #[oai(skip_serializing_if_is_none)]
    pub message: Option<ChatMessage>,
    #[oai(skip_serializing_if_is_none)]
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Object, Debug)]
pub struct Usage {
    #[oai(skip_serializing_if_is_none)]
    pub prompt_tokens: Option<i32>,
    #[oai(skip_serializing_if_is_none)]
    pub completion_tokens: Option<i32>,
    #[oai(skip_serializing_if_is_none)]
    pub total_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct Function {
    #[oai(skip_serializing_if_is_none)]
    pub name: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub description: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub parameters: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Object, Debug, Clone)]
pub struct FunctionCall {
    #[oai(skip_serializing_if_is_none)]
    pub name: Option<String>,
    #[oai(skip_serializing_if_is_none)]
    pub arguments: Option<String>,
}

// Placeholder functions
#[allow(unused_variables)]
fn extract_function_calls(request: &ChatCompletionRequest) -> Option<Vec<FunctionCall>> {
    None
}

#[allow(unused_variables)]
fn correct_request(request: &mut ChatCompletionRequest) {
    // No-op for now
}

#[allow(unused_variables)]
fn modify_user_prompt(request: &mut ChatCompletionRequest) {
    // No-op for now
}

#[allow(unused_variables)]
async fn call_downstream_llm(
    request: ChatCompletionRequest,
) -> Result<ChatCompletionResponse, reqwest::Error> {
    Ok(ChatCompletionResponse {
        id: Some("chatcmpl-123".to_string()),
        object: Some("chat.completion".to_string()),
        created: Some(chrono::Utc::now().timestamp()),
        model: request
            .model
            .clone()
            .or_else(|| Some("gpt-dummy".to_string())),
        choices: Some(vec![ChatCompletionChoice {
            index: Some(0),
            message: Some(ChatMessage {
                role: Some("assistant".to_string()),
                content: Some("This is a dummy response.".to_string()),
                name: None,
                function_call: None,
            }),
            finish_reason: Some("stop".to_string()),
        }]),
        usage: Some(Usage {
            prompt_tokens: Some(10),
            completion_tokens: Some(20),
            total_tokens: Some(30),
        }),
    })
}

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/models", method = "get")]
    async fn get_models(&self) -> Json<Vec<Model>> {
        let models = vec![
            Model {
                id: Some("gpt-4".to_string()),
                object: Some("model".to_string()),
                created: Some(chrono::Utc::now().timestamp() - 3600),
                owned_by: Some("openai".to_string()),
            },
            Model {
                id: Some("gpt-3.5-turbo".to_string()),
                object: Some("model".to_string()),
                created: Some(chrono::Utc::now().timestamp() - 7200),
                owned_by: Some("openai".to_string()),
            },
        ];
        Json(models)
    }

    #[oai(path = "/chat/completions", method = "post")]
    async fn chat_completions(
        &self,
        mut request: Json<ChatCompletionRequest>,
    ) -> Json<ChatCompletionResponse> {
        let _function_calls = extract_function_calls(&request.0);
        correct_request(&mut request.0);
        modify_user_prompt(&mut request.0);

        match call_downstream_llm(request.0.clone()).await {
            Ok(response) => Json(response),
            Err(_) => Json(ChatCompletionResponse {
                id: Some("error-id".to_string()),
                object: Some("error".to_string()),
                created: Some(chrono::Utc::now().timestamp()),
                model: Some("error-model".to_string()),
                choices: Some(vec![ChatCompletionChoice {
                    index: Some(0),
                    message: Some(ChatMessage {
                        role: Some("assistant".to_string()),
                        content: Some("An error occurred calling the downstream LLM.".to_string()),
                        name: None,
                        function_call: None,
                    }),
                    finish_reason: Some("error".to_string()),
                }]),
                usage: None,
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        // SAFETY: Setting RUST_LOG environment variable if not already set.
        // This is done early in main and is a common practice for initializing logging.
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    tracing_subscriber::fmt::init();

    let api_service = OpenApiService::new(Api, "OpenAI Compatible API", "1.0")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    let route = Route::new()
        .nest("/api", api_service) // Serve OpenApiService under /api
        .nest("/", ui); // Serve Swagger UI at the root

    let server_addr = "127.0.0.1:3000";
    println!("Server running on http://{}", server_addr);

    Server::new(TcpListener::bind(server_addr)).run(route).await
}
