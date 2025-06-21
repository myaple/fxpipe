use crate::config::AppConfig;
use crate::llm_client::call_upstream_llm;
use crate::models::{
    ChatCompletionRequest, ChatCompletionResponse, Choice, MessageResponse, Model, ModelsResponse,
};
use poem::{
    handler,
    web::{Data, Json},
};

#[handler]
pub async fn get_models() -> Json<ModelsResponse> {
    let models = vec![Model {
        id: "fx-small".to_string(),
    }];
    Json(ModelsResponse { data: models })
}

#[handler]
pub async fn chat_completions(
    Data(config): Data<&AppConfig>,
    Json(req): Json<ChatCompletionRequest>,
) -> Json<ChatCompletionResponse> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, LlmConfig};
    use crate::models::ModelsResponse;
    use crate::models::{ChatCompletionRequest, Message};
    use poem::http::Method;
    use poem::http::Uri;
    use poem::Endpoint;
    use poem::{Route, Server};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_models() {
        let mock_config = AppConfig {
            model_name: "test-model".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            llm_config: LlmConfig {
                endpoint: "http://example.com".to_string(),
                api_key: None,
            },
        };
        let app = crate::routes::create_routes(mock_config);

        let req = poem::Request::builder()
            .method(Method::GET)
            .uri(Uri::from_static("/v1/models"))
            .finish();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        let body = resp.into_body().into_bytes().await.unwrap();
        let models_resp: ModelsResponse = serde_json::from_slice(&body).unwrap();
        assert!(models_resp.data.iter().any(|m| m.id == "fx-small"));
    }

    #[tokio::test]
    async fn test_chat_completions_stub() {
        let mock_config = AppConfig {
            model_name: "test-model".to_string(),
            host: "0.0.0.0".to_string(),
            port: 0,
            llm_config: LlmConfig {
                endpoint: "http://example.com".to_string(),
                api_key: None,
            },
        };
        let app = crate::routes::create_routes(mock_config);

        let req_payload = ChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
        };
        let req = poem::Request::builder()
            .method(Method::POST)
            .uri(Uri::from_static("/v1/chat/completions"))
            .header("content-type", "application/json")
            .body(poem::Body::from(serde_json::to_vec(&req_payload).unwrap()));

        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), 200);

        let body = resp.into_body().into_bytes().await.unwrap();
        let resp_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(resp_json["id"]
            .as_str()
            .unwrap_or("")
            .starts_with("chatcmpl"));
        assert!(resp_json["choices"].is_array());
    }

    #[test]
    fn test_chat_completion_request_serde() {
        let json_data = json!({
            "model": "gpt-4",
            "messages": [{"role": "user", "content": "Hello"}]
        });
        let req: ChatCompletionRequest = serde_json::from_value(json_data).unwrap();
        assert_eq!(req.model, "gpt-4");
        assert_eq!(req.messages.len(), 1);
    }
}
