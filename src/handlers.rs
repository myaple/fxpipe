#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelsResponse;
    use crate::{ChatCompletionRequest, Message};
    use poem::http::Method;
    use poem::http::Uri;
    use poem::Endpoint;
    use poem::{Route, Server};
    use serde_json::json;

    #[tokio::test]
    async fn test_get_models() {
        let app = crate::routes::create_routes();
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
        let app = crate::routes::create_routes();
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
