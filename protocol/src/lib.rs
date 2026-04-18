use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SOCKET_PATH: &str = "/tmp/afox.sock";
pub const LOG_PATH: &str = "/tmp/afox.log";
pub const CONFIG_PATH: &str = ".config/agentfox/config.json";

pub fn get_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    std::path::Path::new(&home).join(CONFIG_PATH)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Request {
    Search {
        query: String,
        #[serde(default)]
        summarize: bool,
    },
    Open {
        url: String,
        #[serde(default)]
        summarize: bool,
    },
    View {
        #[serde(default)]
        summarize: bool,
    },
    Snap {
        #[serde(default)]
        summarize: bool,
    },
    Click {
        element_id: String,
    },
    Fill {
        element_id: String,
        text: String,
    },
    Text {
        element_id: String,
    },
    Eval {
        code: String,
    },
    Quit,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Response {
    Ok {
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        markdown: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        elements: Option<Vec<SemanticNode>>,
    },
    Error {
        error: String,
    },
}

impl Response {
    pub fn ok_message(message: impl Into<String>) -> Self {
        Self::Ok {
            message: Some(message.into()),
            url: None,
            title: None,
            text: None,
            markdown: None,
            summary: None,
            result: None,
            elements: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            error: message.into(),
        }
    }
}
