use reqwest::Client;
use serde_json::{json, Value};

use crate::config::LlmConfig;

pub async fn call_upstream_llm(
    payload: &str,
    llm_config: &LlmConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let upstream_url = &llm_config.endpoint;

    // Parse the payload as JSON
    let mut payload_json: Value = serde_json::from_str(payload)?;

    // Override the model field with passthrough_model_name
    if let Some(model) = payload_json.get_mut("model") {
        *model = json!(llm_config.passthrough_model_name);
    }

    let mut req_builder = client
        .post(upstream_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload_json)?);

    if let Some(ref api_key) = llm_config.api_key {
        if !api_key.is_empty() {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }
    }

    let resp = req_builder.send().await?;
    let text = resp.text().await?;
    Ok(text)
}
