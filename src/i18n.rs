use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn code(&self) -> &str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![Language::English, Language::Chinese]
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// Translations structure containing all UI strings
#[derive(Debug, Clone)]
pub struct Translations {
    strings: HashMap<String, String>,
}

impl Translations {
    /// Create translations for the given language
    pub fn new(language: Language) -> Self {
        let strings = Self::load_from_file(language).unwrap_or_else(|e| {
            tracing::warn!(
                "Failed to load translations from file: {}. Using fallback.",
                e
            );
            Self::fallback_strings(language)
        });
        Self { strings }
    }

    /// Get a translation string by key
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.strings.get(key).map(|s| s.as_str()).unwrap_or(key)
    }

    /// Load translations from JSON file
    fn load_from_file(
        language: Language,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let locale_file = format!("locales/{}.json", language.code());

        // Try multiple paths for loading locale files
        let paths = vec![
            PathBuf::from(&locale_file),
            PathBuf::from(format!("./{}", locale_file)),
            // For development: relative to cargo project root
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.join(&locale_file)))
                .unwrap_or_else(|| PathBuf::from(&locale_file)),
        ];

        for path in paths {
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                let translations: HashMap<String, String> = serde_json::from_str(&content)?;
                tracing::debug!("Loaded translations from: {:?}", path);
                return Ok(translations);
            }
        }

        Err(format!("Translation file not found: {}", locale_file).into())
    }

    /// Fallback translations if JSON file loading fails
    fn fallback_strings(language: Language) -> HashMap<String, String> {
        match language {
            Language::English => Self::english_strings(),
            Language::Chinese => Self::chinese_strings(),
        }
    }

    /// English translations (fallback)
    fn english_strings() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("send".to_string(), "Send".to_string());
        map.insert("ai_fill".to_string(), "AI Fill".to_string());
        map.insert(
            "ai_fill_placeholder".to_string(),
            "AI Fill feature coming soon!".to_string(),
        );
        map.insert("cancel".to_string(), "Cancel".to_string());
        map.insert("save".to_string(), "Save".to_string());
        map.insert(
            "empty_state".to_string(),
            "Select a request from the sidebar or create a new one".to_string(),
        );

        // Context menu
        map.insert("ctx_new_request".to_string(), "New Request".to_string());
        map.insert("ctx_new_folder".to_string(), "New Folder".to_string());
        map.insert("ctx_rename".to_string(), "Rename".to_string());
        map.insert("ctx_delete".to_string(), "Delete".to_string());
        map.insert("ctx_open_folder".to_string(), "Open Folder".to_string());
        map.insert("ctx_new_collection".to_string(), "New Collection".to_string());

        // Settings dialog
        map.insert("settings".to_string(), "Settings".to_string());
        map.insert("language".to_string(), "Language".to_string());
        map.insert("save_directory".to_string(), "Save Directory".to_string());
        map.insert("browse".to_string(), "Browse".to_string());
        map.insert("close".to_string(), "Close".to_string());

        // AI Configuration
        map.insert("ai_config".to_string(), "AI Configuration".to_string());
        map.insert("ai_api_url".to_string(), "API URL".to_string());
        map.insert("ai_api_key".to_string(), "API Key".to_string());
        map.insert("ai_model".to_string(), "Model".to_string());

        // AI Fill Dialog
        map.insert("ai_fill_dialog_title".to_string(), "AI Fill".to_string());
        map.insert("ai_fill_hint".to_string(), "Paste any request-related content: backend API code, frontend request code, struct definitions, curl commands, etc.".to_string());
        map.insert(
            "ai_fill_input_placeholder".to_string(),
            "Paste code or curl command here...".to_string(),
        );
        map.insert(
            "ai_fill_loading".to_string(),
            "AI is processing your request, please wait...".to_string(),
        );
        map.insert("confirm".to_string(), "Confirm".to_string());
        map.insert("word_wrap".to_string(), "Word Wrap".to_string());
        map.insert("format_json".to_string(), "Format JSON".to_string());

        map
    }

    /// Chinese translations (fallback)
    fn chinese_strings() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("send".to_string(), "发送".to_string());
        map.insert("ai_fill".to_string(), "AI 填充".to_string());
        map.insert(
            "ai_fill_placeholder".to_string(),
            "AI 填充功能即将推出！".to_string(),
        );
        map.insert("cancel".to_string(), "取消".to_string());
        map.insert("save".to_string(), "保存".to_string());
        map.insert(
            "empty_state".to_string(),
            "从侧边栏选择一个请求或创建新请求".to_string(),
        );

        // Context menu
        map.insert("ctx_new_request".to_string(), "新建请求".to_string());
        map.insert("ctx_new_folder".to_string(), "新建文件夹".to_string());
        map.insert("ctx_rename".to_string(), "重命名".to_string());
        map.insert("ctx_delete".to_string(), "删除".to_string());
        map.insert("ctx_open_folder".to_string(), "打开所在文件夹".to_string());
        map.insert("ctx_new_collection".to_string(), "新建集合".to_string());

        // Settings dialog
        map.insert("settings".to_string(), "设置".to_string());
        map.insert("language".to_string(), "语言".to_string());
        map.insert("save_directory".to_string(), "保存目录".to_string());
        map.insert("browse".to_string(), "选择".to_string());
        map.insert("close".to_string(), "关闭".to_string());

        // AI Configuration
        map.insert("ai_config".to_string(), "AI 配置".to_string());
        map.insert("ai_api_url".to_string(), "API 地址".to_string());
        map.insert("ai_api_key".to_string(), "API 密钥".to_string());
        map.insert("ai_model".to_string(), "模型".to_string());

        // AI Fill Dialog
        map.insert("ai_fill_dialog_title".to_string(), "AI 填充".to_string());
        map.insert(
            "ai_fill_hint".to_string(),
            "粘贴任何与请求相关的内容：后端接口代码、前端请求代码、结构体定义、curl 命令等"
                .to_string(),
        );
        map.insert(
            "ai_fill_input_placeholder".to_string(),
            "在此粘贴代码或 curl 命令...".to_string(),
        );
        map.insert(
            "ai_fill_loading".to_string(),
            "AI 正在处理您的请求，请稍候...".to_string(),
        );
        map.insert("confirm".to_string(), "确定".to_string());
        map.insert("word_wrap".to_string(), "自动换行".to_string());
        map.insert("format_json".to_string(), "美化 JSON".to_string());

        map
    }
}

/// Global accessor for translations (stored in app state)
pub trait I18n {
    fn t<'a>(&'a self, key: &'a str) -> &'a str;
    fn language(&self) -> Language;
}
