use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Object)]
pub struct ModelsResponse {
    pub data: Vec<Model>,
}

#[derive(Serialize, Deserialize, Object)]
pub struct Model {
    pub id: String,
}

#[derive(Serialize, Deserialize, Object)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Object)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Object)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Object)]
pub struct Choice {
    pub index: usize,
    pub message: MessageResponse,
}

#[derive(Serialize, Deserialize, Object)]
pub struct MessageResponse {
    pub role: String,
    pub content: String,
}

// New structures for conversation logging
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")] // Add this attribute
pub enum ContentPart {
    #[serde(rename = "text")]
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<HashMap<String, serde_json::Value>>,
    },
    #[serde(rename = "toolRequest")]
    ToolRequest {
        id: String,
        #[serde(rename = "toolCall")]
        tool_call: ToolCall,
    },
    #[serde(rename = "toolResponse")]
    ToolResponse {
        id: String,
        #[serde(rename = "toolResult")]
        tool_result: ToolResult,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToolCall {
    status: String,
    value: ToolCallValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToolCallValue {
    name: String,
    arguments: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToolResult {
    status: String,
    value: Vec<ToolResultItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ToolResultItem {
    #[serde(rename = "type")]
    item_type: String,
    text: String,
    annotations: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LogMessage {
    role: String,
    created: u64,
    content: Vec<ContentPart>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_deserialize_conversation_log() {
        // Read test data as a JSON array
        let data = fs::read_to_string("data/test.json").expect("Failed to read test.json");

        // Parse as vector of LogMessage objects
        let messages: Vec<LogMessage> =
            serde_json::from_str(&format!("[{}]", data.replace("}\n{", "},\n{")))
                .expect("Failed to parse JSON");

        // Verify structure
        assert_eq!(messages.len(), 3);

        // Verify first message (user)
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content.len(), 1);

        if let ContentPart::Text { text, annotations } = &messages[0].content[0] {
            assert!(text.starts_with("Based on the extensive conversation"));
            assert!(annotations.is_none());
        } else {
            panic!("First message should be text content");
        }

        // Verify second message (assistant)
        assert_eq!(messages[1].role, "assistant");
        assert_eq!(messages[1].content.len(), 2);

        if let ContentPart::Text { text, annotations } = &messages[1].content[0] {
            assert_eq!(
                text,
                "### Final Test Run\nLet's run the tests to ensure everything is fixed."
            );
            assert!(annotations.is_none());
        } else {
            panic!("First part of second message should be text");
        }

        if let ContentPart::ToolRequest { id: _, tool_call } = &messages[1].content[1] {
            assert_eq!(tool_call.value.name, "developer__shell");
            assert_eq!(
                tool_call.value.arguments["command"],
                "cd /home/matt/projects/fxpipe && cargo test"
            );
        } else {
            panic!("Second part of second message should be tool request");
        }

        // Verify third message (user with tool response)
        assert_eq!(messages[2].role, "user");
        assert_eq!(messages[2].content.len(), 1);

        if let ContentPart::ToolResponse { id: _, tool_result } = &messages[2].content[0] {
            assert_eq!(tool_result.value.len(), 2);
            assert_eq!(tool_result.value[0].item_type, "text");
            assert!(tool_result.value[0].text.contains("Compiling fxpipe"));
        } else {
            panic!("Third message should be tool response");
        }
    }
}
