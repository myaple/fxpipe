use serde_json::Value;
use std::error::Error;

pub fn extract_function_calls(llm_response: &str) -> Result<String, Box<dyn Error>> {
    // First, try to find JSON-like structure
    if let Some((start, end)) = find_json_boundaries(llm_response) {
        let json_str = &llm_response[start..end];

        // Try to parse as JSON to validate
        if serde_json::from_str::<Value>(json_str).is_ok() {
            return Ok(json_str.to_string());
        }
    }

    // Fallback to original content
    Ok(llm_response.to_string())
}

fn find_json_boundaries(s: &str) -> Option<(usize, usize)> {
    let mut stack = Vec::new();
    let mut start = None;

    for (i, c) in s.chars().enumerate() {
        match c {
            '{' | '[' => {
                if stack.is_empty() {
                    start = Some(i);
                }
                stack.push(c);
            }
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                    if stack.is_empty() && start.is_some() {
                        return Some((start.unwrap(), i + 1));
                    }
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                    if stack.is_empty() && start.is_some() {
                        return Some((start.unwrap(), i + 1));
                    }
                }
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_json_boundaries() {
        let input = r#"Some text {"key": "value"} more text"#;
        let (start, end) = find_json_boundaries(input).unwrap();
        assert_eq!(&input[start..end], r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_function_calls() {
        let input = r#"Here's your data: {"function_call":{"name":"get_weather","arguments":"{\"location\":\"London\"}"}}"#;
        let extracted = extract_function_calls(input).unwrap();
        assert!(extracted.contains("get_weather"));
        assert!(extracted.contains("London"));
    }
}
