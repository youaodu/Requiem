use iced::keyboard::{key::Named, Key, Modifiers};
use serde::{Deserialize, Serialize};

/// Keyboard shortcut action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutAction {
    // Request operations
    SendRequest,
    CancelRequest,
    SaveRequest,
    NewRequest,
    DuplicateRequest,

    // Tab operations
    CloseTab,
    NextTab,
    PreviousTab,
    NewTab,
    ReopenClosedTab,

    // Collection operations
    NewCollection,
    NewFolder,
    DeleteItem,
    RenameItem,
    SearchRequests,

    // View operations
    ToggleSidebar,
    FocusUrlBar,
    FocusSearch,

    // Environment operations
    ManageEnvironments,
    NextEnvironment,
    PreviousEnvironment,

    // Other operations
    ShowSettings,
    ShowHelp,
    CopyResponse,
    FormatJson,
}

impl ShortcutAction {
    /// Get the default key combination for this action
    pub fn default_shortcut(&self) -> KeyShortcut {
        match self {
            // Request operations
            ShortcutAction::SendRequest => KeyShortcut::new(
                Key::Character("r".into()),
                Modifiers::CTRL | Modifiers::ALT,
            ),
            ShortcutAction::CancelRequest => {
                KeyShortcut::new(Key::Named(Named::Escape), Modifiers::empty())
            }
            ShortcutAction::SaveRequest => {
                KeyShortcut::new(Key::Character("s".into()), Modifiers::CTRL)
            }
            ShortcutAction::NewRequest => {
                KeyShortcut::new(Key::Character("n".into()), Modifiers::CTRL)
            }
            ShortcutAction::DuplicateRequest => {
                KeyShortcut::new(Key::Character("d".into()), Modifiers::CTRL)
            }

            // Tab operations
            ShortcutAction::CloseTab => {
                KeyShortcut::new(Key::Character("w".into()), Modifiers::CTRL)
            }
            ShortcutAction::NextTab => {
                KeyShortcut::new(Key::Named(Named::Tab), Modifiers::CTRL)
            }
            ShortcutAction::PreviousTab => {
                KeyShortcut::new(Key::Named(Named::Tab), Modifiers::CTRL | Modifiers::SHIFT)
            }
            ShortcutAction::NewTab => {
                KeyShortcut::new(Key::Character("t".into()), Modifiers::CTRL)
            }
            ShortcutAction::ReopenClosedTab => KeyShortcut::new(
                Key::Character("t".into()),
                Modifiers::CTRL | Modifiers::SHIFT,
            ),

            // Collection operations
            ShortcutAction::NewCollection => KeyShortcut::new(
                Key::Character("n".into()),
                Modifiers::CTRL | Modifiers::SHIFT,
            ),
            ShortcutAction::NewFolder => {
                KeyShortcut::new(Key::Character("f".into()), Modifiers::CTRL | Modifiers::SHIFT)
            }
            ShortcutAction::DeleteItem => {
                KeyShortcut::new(Key::Named(Named::Delete), Modifiers::empty())
            }
            ShortcutAction::RenameItem => {
                KeyShortcut::new(Key::Named(Named::F2), Modifiers::empty())
            }
            ShortcutAction::SearchRequests => {
                KeyShortcut::new(Key::Character("f".into()), Modifiers::CTRL)
            }

            // View operations
            ShortcutAction::ToggleSidebar => {
                KeyShortcut::new(Key::Character("b".into()), Modifiers::CTRL)
            }
            ShortcutAction::FocusUrlBar => {
                KeyShortcut::new(Key::Character("l".into()), Modifiers::CTRL)
            }
            ShortcutAction::FocusSearch => {
                KeyShortcut::new(Key::Character("k".into()), Modifiers::CTRL)
            }

            // Environment operations
            ShortcutAction::ManageEnvironments => {
                KeyShortcut::new(Key::Character("e".into()), Modifiers::CTRL)
            }
            ShortcutAction::NextEnvironment => {
                KeyShortcut::new(Key::Character("e".into()), Modifiers::CTRL | Modifiers::SHIFT)
            }
            ShortcutAction::PreviousEnvironment => KeyShortcut::new(
                Key::Character("e".into()),
                Modifiers::CTRL | Modifiers::ALT,
            ),

            // Other operations
            ShortcutAction::ShowSettings => KeyShortcut::new(
                Key::Character(",".into()),
                Modifiers::CTRL,
            ),
            ShortcutAction::ShowHelp => {
                KeyShortcut::new(Key::Named(Named::F1), Modifiers::empty())
            }
            ShortcutAction::CopyResponse => {
                KeyShortcut::new(Key::Character("c".into()), Modifiers::CTRL | Modifiers::SHIFT)
            }
            ShortcutAction::FormatJson => {
                KeyShortcut::new(Key::Character("j".into()), Modifiers::CTRL | Modifiers::SHIFT)
            }
        }
    }

    /// Get the display name for this action (English)
    pub fn display_name(&self) -> &'static str {
        match self {
            ShortcutAction::SendRequest => "Send Request",
            ShortcutAction::CancelRequest => "Cancel Request",
            ShortcutAction::SaveRequest => "Save Request",
            ShortcutAction::NewRequest => "New Request",
            ShortcutAction::DuplicateRequest => "Duplicate Request",
            ShortcutAction::CloseTab => "Close Tab",
            ShortcutAction::NextTab => "Next Tab",
            ShortcutAction::PreviousTab => "Previous Tab",
            ShortcutAction::NewTab => "New Tab",
            ShortcutAction::ReopenClosedTab => "Reopen Closed Tab",
            ShortcutAction::NewCollection => "New Collection",
            ShortcutAction::NewFolder => "New Folder",
            ShortcutAction::DeleteItem => "Delete Item",
            ShortcutAction::RenameItem => "Rename Item",
            ShortcutAction::SearchRequests => "Search Requests",
            ShortcutAction::ToggleSidebar => "Toggle Sidebar",
            ShortcutAction::FocusUrlBar => "Focus URL Bar",
            ShortcutAction::FocusSearch => "Focus Search",
            ShortcutAction::ManageEnvironments => "Manage Environments",
            ShortcutAction::NextEnvironment => "Next Environment",
            ShortcutAction::PreviousEnvironment => "Previous Environment",
            ShortcutAction::ShowSettings => "Show Settings",
            ShortcutAction::ShowHelp => "Show Help",
            ShortcutAction::CopyResponse => "Copy Response",
            ShortcutAction::FormatJson => "Format JSON",
        }
    }

    /// Get the display name for this action (Chinese)
    pub fn display_name_zh(&self) -> &'static str {
        match self {
            ShortcutAction::SendRequest => "发送请求",
            ShortcutAction::CancelRequest => "取消请求",
            ShortcutAction::SaveRequest => "保存请求",
            ShortcutAction::NewRequest => "新建请求",
            ShortcutAction::DuplicateRequest => "复制请求",
            ShortcutAction::CloseTab => "关闭标签",
            ShortcutAction::NextTab => "下一个标签",
            ShortcutAction::PreviousTab => "上一个标签",
            ShortcutAction::NewTab => "新建标签",
            ShortcutAction::ReopenClosedTab => "重新打开已关闭的标签",
            ShortcutAction::NewCollection => "新建集合",
            ShortcutAction::NewFolder => "新建文件夹",
            ShortcutAction::DeleteItem => "删除项目",
            ShortcutAction::RenameItem => "重命名",
            ShortcutAction::SearchRequests => "搜索请求",
            ShortcutAction::ToggleSidebar => "切换侧边栏",
            ShortcutAction::FocusUrlBar => "聚焦URL栏",
            ShortcutAction::FocusSearch => "聚焦搜索框",
            ShortcutAction::ManageEnvironments => "管理环境",
            ShortcutAction::NextEnvironment => "下一个环境",
            ShortcutAction::PreviousEnvironment => "上一个环境",
            ShortcutAction::ShowSettings => "显示设置",
            ShortcutAction::ShowHelp => "显示帮助",
            ShortcutAction::CopyResponse => "复制响应",
            ShortcutAction::FormatJson => "格式化JSON",
        }
    }

    /// Get the category for this action (for grouping in UI)
    pub fn category(&self) -> ShortcutCategory {
        match self {
            ShortcutAction::SendRequest
            | ShortcutAction::CancelRequest
            | ShortcutAction::SaveRequest
            | ShortcutAction::NewRequest
            | ShortcutAction::DuplicateRequest => ShortcutCategory::Request,

            ShortcutAction::CloseTab
            | ShortcutAction::NextTab
            | ShortcutAction::PreviousTab
            | ShortcutAction::NewTab
            | ShortcutAction::ReopenClosedTab => ShortcutCategory::Tabs,

            ShortcutAction::NewCollection
            | ShortcutAction::NewFolder
            | ShortcutAction::DeleteItem
            | ShortcutAction::RenameItem
            | ShortcutAction::SearchRequests => ShortcutCategory::Collection,

            ShortcutAction::ToggleSidebar
            | ShortcutAction::FocusUrlBar
            | ShortcutAction::FocusSearch => ShortcutCategory::View,

            ShortcutAction::ManageEnvironments
            | ShortcutAction::NextEnvironment
            | ShortcutAction::PreviousEnvironment => ShortcutCategory::Environment,

            ShortcutAction::ShowSettings
            | ShortcutAction::ShowHelp
            | ShortcutAction::CopyResponse
            | ShortcutAction::FormatJson => ShortcutCategory::Other,
        }
    }

    /// Get all available actions
    pub fn all() -> Vec<ShortcutAction> {
        vec![
            // Request operations
            ShortcutAction::SendRequest,
            ShortcutAction::CancelRequest,
            ShortcutAction::SaveRequest,
            ShortcutAction::NewRequest,
            ShortcutAction::DuplicateRequest,
            // Tab operations
            ShortcutAction::CloseTab,
            ShortcutAction::NextTab,
            ShortcutAction::PreviousTab,
            ShortcutAction::NewTab,
            ShortcutAction::ReopenClosedTab,
            // Collection operations
            ShortcutAction::NewCollection,
            ShortcutAction::NewFolder,
            ShortcutAction::DeleteItem,
            ShortcutAction::RenameItem,
            ShortcutAction::SearchRequests,
            // View operations
            ShortcutAction::ToggleSidebar,
            ShortcutAction::FocusUrlBar,
            ShortcutAction::FocusSearch,
            // Environment operations
            ShortcutAction::ManageEnvironments,
            ShortcutAction::NextEnvironment,
            ShortcutAction::PreviousEnvironment,
            // Other operations
            ShortcutAction::ShowSettings,
            ShortcutAction::ShowHelp,
            ShortcutAction::CopyResponse,
            ShortcutAction::FormatJson,
        ]
    }
}

/// Shortcut category for grouping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutCategory {
    Request,
    Tabs,
    Collection,
    View,
    Environment,
    Other,
}

impl ShortcutCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            ShortcutCategory::Request => "Request Operations",
            ShortcutCategory::Tabs => "Tab Management",
            ShortcutCategory::Collection => "Collection Management",
            ShortcutCategory::View => "View Operations",
            ShortcutCategory::Environment => "Environment Management",
            ShortcutCategory::Other => "Other",
        }
    }

    pub fn display_name_zh(&self) -> &'static str {
        match self {
            ShortcutCategory::Request => "请求操作",
            ShortcutCategory::Tabs => "标签管理",
            ShortcutCategory::Collection => "集合管理",
            ShortcutCategory::View => "视图操作",
            ShortcutCategory::Environment => "环境管理",
            ShortcutCategory::Other => "其他",
        }
    }
}

/// Keyboard shortcut definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyShortcut {
    pub key: KeyWrapper,
    pub modifiers: ModifiersWrapper,
}

impl KeyShortcut {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self {
            key: KeyWrapper(key),
            modifiers: ModifiersWrapper(modifiers),
        }
    }

    /// Check if this shortcut matches the given key and modifiers
    pub fn matches(&self, key: &Key, modifiers: &Modifiers) -> bool {
        self.key.0 == *key && self.modifiers.0 == *modifiers
    }

    /// Get a display string for this shortcut
    pub fn display_string(&self) -> String {
        let mut parts = Vec::new();

        // Add modifiers
        if self.modifiers.0.contains(Modifiers::CTRL) {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.0.contains(Modifiers::SHIFT) {
            parts.push("Shift".to_string());
        }
        if self.modifiers.0.contains(Modifiers::ALT) {
            parts.push("Alt".to_string());
        }
        // Note: SUPER/Command modifier is not commonly used in iced cross-platform apps

        // Add key
        parts.push(self.key.display_string());

        parts.join("+")
    }
}

/// Wrapper around iced::keyboard::Key for serialization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyWrapper(pub Key);

impl KeyWrapper {
    fn display_string(&self) -> String {
        match &self.0 {
            Key::Character(c) => c.to_uppercase(),
            Key::Named(named) => format!("{:?}", named),
            _ => "Unknown".to_string(),
        }
    }
}

impl Serialize for KeyWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            Key::Character(c) => serializer.serialize_str(&format!("char:{}", c)),
            Key::Named(named) => serializer.serialize_str(&format!("named:{:?}", named)),
            _ => serializer.serialize_str("unknown"),
        }
    }
}

impl<'de> Deserialize<'de> for KeyWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(char_part) = s.strip_prefix("char:") {
            Ok(KeyWrapper(Key::Character(char_part.into())))
        } else if let Some(named_part) = s.strip_prefix("named:") {
            // Parse named keys
            let key = match named_part {
                "Escape" => Key::Named(Named::Escape),
                "Tab" => Key::Named(Named::Tab),
                "Enter" => Key::Named(Named::Enter),
                "Delete" => Key::Named(Named::Delete),
                "F1" => Key::Named(Named::F1),
                "F2" => Key::Named(Named::F2),
                "ArrowLeft" => Key::Named(Named::ArrowLeft),
                "ArrowRight" => Key::Named(Named::ArrowRight),
                "ArrowUp" => Key::Named(Named::ArrowUp),
                "ArrowDown" => Key::Named(Named::ArrowDown),
                _ => return Err(serde::de::Error::custom("Unknown named key")),
            };
            Ok(KeyWrapper(key))
        } else {
            Err(serde::de::Error::custom("Invalid key format"))
        }
    }
}

/// Wrapper around iced::keyboard::Modifiers for serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModifiersWrapper(pub Modifiers);

impl Serialize for ModifiersWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut parts = Vec::new();
        if self.0.contains(Modifiers::CTRL) {
            parts.push("ctrl");
        }
        if self.0.contains(Modifiers::SHIFT) {
            parts.push("shift");
        }
        if self.0.contains(Modifiers::ALT) {
            parts.push("alt");
        }
        // Note: SUPER modifier not used for cross-platform compatibility
        serializer.serialize_str(&parts.join("|"))
    }
}

impl<'de> Deserialize<'de> for ModifiersWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut modifiers = Modifiers::empty();
        for part in s.split('|') {
            match part {
                "ctrl" => modifiers |= Modifiers::CTRL,
                "shift" => modifiers |= Modifiers::SHIFT,
                "alt" => modifiers |= Modifiers::ALT,
                "" => {} // Empty string for no modifiers
                _ => return Err(serde::de::Error::custom("Unknown modifier")),
            }
        }
        Ok(ModifiersWrapper(modifiers))
    }
}

/// Shortcut registry that manages all shortcuts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutRegistry {
    shortcuts: std::collections::HashMap<ShortcutAction, KeyShortcut>,
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutRegistry {
    /// Create a new registry with default shortcuts
    pub fn new() -> Self {
        let mut shortcuts = std::collections::HashMap::new();
        for action in ShortcutAction::all() {
            shortcuts.insert(action, action.default_shortcut());
        }
        Self { shortcuts }
    }

    /// Get the shortcut for an action
    pub fn get(&self, action: &ShortcutAction) -> Option<&KeyShortcut> {
        self.shortcuts.get(action)
    }

    /// Set a custom shortcut for an action
    pub fn set(&mut self, action: ShortcutAction, shortcut: KeyShortcut) {
        self.shortcuts.insert(action, shortcut);
    }

    /// Reset an action to its default shortcut
    pub fn reset(&mut self, action: ShortcutAction) {
        self.shortcuts.insert(action, action.default_shortcut());
    }

    /// Reset all shortcuts to defaults
    pub fn reset_all(&mut self) {
        for action in ShortcutAction::all() {
            self.shortcuts.insert(action, action.default_shortcut());
        }
    }

    /// Find the action for a given key combination
    pub fn find_action(&self, key: &Key, modifiers: &Modifiers) -> Option<ShortcutAction> {
        self.shortcuts
            .iter()
            .find(|(_, shortcut)| shortcut.matches(key, modifiers))
            .map(|(action, _)| *action)
    }

    /// Get all shortcuts grouped by category
    pub fn by_category(&self) -> Vec<(ShortcutCategory, Vec<(ShortcutAction, &KeyShortcut)>)> {
        let mut result = Vec::new();
        let categories = vec![
            ShortcutCategory::Request,
            ShortcutCategory::Tabs,
            ShortcutCategory::Collection,
            ShortcutCategory::View,
            ShortcutCategory::Environment,
            ShortcutCategory::Other,
        ];

        for category in categories {
            let mut shortcuts_in_category = Vec::new();
            for (action, shortcut) in &self.shortcuts {
                if action.category() == category {
                    shortcuts_in_category.push((*action, shortcut));
                }
            }
            // Sort by action display name for consistent ordering
            shortcuts_in_category.sort_by_key(|(action, _)| action.display_name());
            result.push((category, shortcuts_in_category));
        }

        result
    }
}
