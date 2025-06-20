use reqwest::Client;
use std::env;

pub async fn call_upstream_llm(payload: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    // The upstream URL should be configured via environment variable for flexibility
    let upstream_url = env::var("UPSTREAM_LLM_ENDPOINT")
        .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string());
    // The upstream API key (if any) should be provided via environment variable
    let api_key = env::var("UPSTREAM_API_KEY").unwrap_or_default();

    let mut req_builder = client
        .post(&upstream_url)
        .header("Content-Type", "application/json");

    if !api_key.is_empty() {
        req_builder = req_builder.header("Authorization", format!("Bearer {}", api_key));
    }

    let resp = req_builder.body(payload.to_string()).send().await?;

    let text = resp.text().await?;
    Ok(text)
}
