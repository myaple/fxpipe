use std::error::Error;
use regex::Regex;

pub fn extract_function_calls(llm_response: &str) -> Result<String, Box<dyn Error>> {
    // First, try strict extraction of function call patterns
    if let Some(extracted) = extract_strict_function_call(llm_response) {
        return Ok(extracted);
    }

    // If strict extraction fails, try lenient JSON extraction
    if let Some((start, end)) = find_json_boundaries(llm_response) {
        return Ok(llm_response[start..end].to_string());
    }

    // Fallback to original content
    Ok(llm_response.to_string())
}

fn extract_strict_function_call(s: &str) -> Option<String> {
    let re = Regex::new(r#"\{\s*"function_call"\s*:\s*\{\s*"name"\s*:\s*"([^"]+)"\s*,\s*"arguments"\s*:\s*"([^"]+)"\s*\}\s*\}"#).unwrap();
    if let Some(caps) = re.captures(s) {
        let name = caps.get(1).unwrap().as_str();
        let args = caps.get(2).unwrap().as_str();
        Some(format!("{{\"name\":\"{}\",\"arguments\":\"{}\"}}", name, args))
    } else {
        None
    }
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
    fn test_extract_strict_function_call() {
        let input = r#"{\"function_call\":{\"name\":\"get_weather\",\"arguments\":\"{\\\"location\\\":\\\"London\\\"}\"}}"#;
        let extracted = extract_strict_function_call(input).unwrap();
        assert!(extracted.contains("get_weather"));
        assert!(extracted.contains("London"));
    }

    #[test]
    fn test_find_json_boundaries() {
        let input = r#"Some text {"key": "value"} more text"#;
        let (start, end) = find_json_boundaries(input).unwrap();
        assert_eq!(&input[start..end], r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_function_calls() {
        let input = r#"{\"function_call\":{\"name\":\"get_weather\",\"arguments\":\"{\\\"location\\\":\\\"London\\\"}\"}}"#;
        let extracted = extract_function_calls(input).unwrap();
        assert!(extracted.contains("get_weather"));
        assert!(extracted.contains("London"));
    }
}
