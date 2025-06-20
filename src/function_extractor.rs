use regex::Regex;
use serde_json::Value;

pub fn extract_function_calls(llm_response: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Regex to match JSON objects within the response
    // This is a basic matching of curly braces blocks, could be improved
    let re = Regex::new(r"\{.*?\}")?;

    for mat in re.find_iter(llm_response) {
        let candidate = mat.as_str();
        // Try to parse candidate as JSON
        if let Ok(json_val) = serde_json::from_str::<Value>(candidate) {
            // Check for typical function call keys (e.g. "function_call" or some marker)
            if json_val.get("function_call").is_some() || json_val.get("name").is_some() {
                // Return cleaned json string
                let cleaned = serde_json::to_string(&json_val)?;
                return Ok(cleaned);
            }
        }
    }

    // If nothing extracted, return original
    Ok(llm_response.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_valid_function_call() {
        let input = r#"Some text {\"function_call\": {\"name\": \"my_func\", \"arguments\": \"{\\\"arg1\\\": \\\"value1\\\"}\"}} some more text"#;
        let res = extract_function_calls(input).unwrap();
        assert!(res.contains("my_func"));
        assert!(res.contains("arg1"));
    }

    #[test]
    fn test_no_function_call_returns_original() {
        let input = "No json here at all";
        let res = extract_function_calls(input).unwrap();
        assert_eq!(res, input);
    }

    #[test]
    fn test_malformed_json_ignored() {
        let input = "Some text {\"function_call\": {\"name\": \"my_func\", some broken json }";
        let res = extract_function_calls(input).unwrap();
        // Should return original because JSON is malformed
        assert_eq!(res, input);
    }
}
