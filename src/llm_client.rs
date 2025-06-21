use reqwest::Client;

use crate::config::LlmConfig;

pub async fn call_upstream_llm(
    payload: &str,
    llm_config: &LlmConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let upstream_url = &llm_config.endpoint;

    let mut req_builder = client
        .post(upstream_url)
        .header("Content-Type", "application/json");

    if let Some(ref api_key) = llm_config.api_key {
        if !api_key.is_empty() {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
        }
    }

    let resp = req_builder.body(payload.to_string()).send().await?;

    let text = resp.text().await?;
    Ok(text)
}
