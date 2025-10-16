use serde::{Deserialize, Serialize};
use std::fmt;

/// AI Engine types available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiEngine {
    OpenAI,
    ClaudeCode,
    Codex,
}

impl AiEngine {
    pub fn display_name(&self) -> &str {
        match self {
            AiEngine::OpenAI => "OpenAI",
            AiEngine::ClaudeCode => "Claude Code",
            AiEngine::Codex => "Codex",
        }
    }

    pub fn all() -> Vec<AiEngine> {
        vec![AiEngine::OpenAI, AiEngine::ClaudeCode, AiEngine::Codex]
    }
}

impl Default for AiEngine {
    fn default() -> Self {
        AiEngine::OpenAI
    }
}

impl fmt::Display for AiEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// OpenAI specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4".to_string(),
        }
    }
}

/// AI configuration containing engine selection and engine-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub engine: AiEngine,
    pub openai_config: OpenAIConfig,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            engine: AiEngine::default(),
            openai_config: OpenAIConfig::default(),
        }
    }
}
