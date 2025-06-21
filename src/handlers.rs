use crate::config::AppConfig;
use crate::function_extractor::extract_function_calls;
use crate::llm_client::call_upstream_llm;
use crate::models::{
    ChatCompletionRequest, ChatCompletionResponse, Choice, MessageResponse, Model, ModelsResponse,
};
use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use serde_json::{json, Value};

#[derive(ApiResponse)]
enum CustomResponse {
    #[oai(status = 200)]
    Success(Json<Value>),
    #[oai(status = 400)]
    BadRequest(Json<Value>),
}

#[derive(Clone)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/v1/models", method = "get")]
    async fn get_models(
        &self,
        Data(config): Data<&AppConfig>,
    ) -> Result<CustomResponse, poem::Error> {
        let models = vec![Model {
            id: config.exposed_model_name.clone(),
        }];
        Ok(CustomResponse::Success(Json(json!(ModelsResponse {
            data: models
        }))))
    }

    #[oai(path = "/v1/chat/completions", method = "post")]
    async fn chat_completions(
        &self,
        Json(req): Json<ChatCompletionRequest>,
        Data(config): Data<&AppConfig>,
    ) -> Result<CustomResponse, poem::Error> {
        let payload = serde_json::to_string(&req).unwrap_or_else(|_| "{}".to_string());
        let result = call_upstream_llm(&payload, &config.llm_config).await;

        match result {
            Ok(text) => {
                let mut response = ChatCompletionResponse {
                    id: "chatcmpl-123".to_string(),
                    object: "chat.completion".to_string(),
                    created: 0,
                    choices: vec![Choice {
                        index: 0,
                        message: MessageResponse {
                            role: "assistant".to_string(),
                            content: text.clone(),
                        },
                    }],
                };

                // Process each choice to extract function calls
                for choice in &mut response.choices {
                    if let content = &choice.message.content {
                        match extract_function_calls(content) {
                            Ok(extracted) => {
                                // If we got a JSON string different from original content
                                if extracted != *content {
                                    if let Ok(function_call) = serde_json::from_str::<MessageResponse>(&extracted) {
                                        // Convert to proper function call message
                                        choice.message = function_call;
                                    }
                                }
                            }
                            Err(e) => log::error!("Function extraction failed: {}", e),
                        }
                    }
                }

                Ok(CustomResponse::Success(Json(json!(response))))
            }
            Err(e) => Ok(CustomResponse::BadRequest(Json(
                json!({ "error": e.to_string() }),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{AppConfig, LlmConfig};
    use crate::models::{ChatCompletionRequest, Message};
    use poem::http::{Method, Uri};
    use poem::{Endpoint, Request};

    async fn get_response_body(resp: impl poem::IntoResponse) -> String {
        resp.into_response()
            .into_body()
            .into_string()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_models() {
        let mock_config = AppConfig {
            exposed_model_name: "test-model".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            llm_config: LlmConfig {
                endpoint: "http://example.com".to_string(),
                api_key: None,
                passthrough_model_name: "backend-model".to_string(),
            },
        };
        let app = crate::routes::create_routes(&mock_config);

        let req = Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/api/v1/models"))
            .finish();
        let resp = app.call(req).await.unwrap();
        let body = get_response_body(resp).await;
        assert!(body.contains("test-model"));
    }

    #[tokio::test]
    async fn test_chat_completions_stub() {
        let mock_config = AppConfig {
            exposed_model_name: "test-model".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            llm_config: LlmConfig {
                endpoint: "http://example.com".to_string(),
                api_key: None,
                passthrough_model_name: "backend-model".to_string(),
            },
        };
        let app = crate::routes::create_routes(&mock_config);

        let req_payload = ChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
        };
        let req = Request::builder()
            .method(Method::POST)
            .uri(Uri::from_static("/api/v1/chat/completions"))
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&req_payload).unwrap());

        let resp = app.call(req).await.unwrap();
        let body = get_response_body(resp).await;
        assert!(body.contains("chatcmpl"));
    }
}