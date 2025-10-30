use crate::ai_client::AiClient;
use crate::app::Message;
use crate::i18n::{I18n, Language, Translations};
use crate::models::{
    AiConfig, BodyFormat, BodyViewMode, Collection, CollectionItem, Environment, Request,
    RequestTab, Response, ResponseTab,
};
use crate::ui::toast::Toast;
use crate::utils::navigation;
use iced::widget::{text_editor, Id};
use iced::{event, keyboard, mouse, Element, Event, Subscription};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestTabItem {
    pub id: Uuid,
    pub name: String,
    pub request_path: Option<Vec<usize>>, // None for unsaved tabs
    pub is_modified: bool,
    pub is_new: bool,                    // true for unsaved new requests
    pub draft_request: Option<Request>,  // Draft request data for unsaved tabs
    pub parent_path: Option<Vec<usize>>, // Parent path for saving new requests
}

#[derive(Debug, Clone)]
pub struct DragState {
    pub dragging_tab_index: usize,
    pub hover_index: Option<usize>,
    pub initial_mouse_x: f32, // Initial mouse X position when drag started
    pub current_mouse_x: f32, // Current mouse X position during drag
}

#[derive(Debug, Clone)]
pub struct TabPressState {
    pub tab_index: usize,
    pub press_time: std::time::Instant,
    pub initial_x: f32, // 鼠标按下时的初始 x 位置
    pub last_x: f32,    // 上一次鼠标的 x 位置
    pub delta_x: f32,   // 本次移动的 x 轴差值 (当前x - 上一次x)
}

#[derive(Debug, Clone)]
pub enum ContextMenuTarget {
    Request,
    Folder,
    Collection,
    EmptyArea, // Empty area in request list for creating new collections
}

#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub path: Vec<usize>,
    pub x: f32,
    pub y: f32,
    pub target: ContextMenuTarget,
}

pub struct Requiem {
    pub collections: Vec<Collection>,
    pub selected_collection: Option<usize>,
    pub selected_request: Option<Vec<usize>>, // path to the selected request
    pub active_tab: RequestTab,
    pub response: Option<Response>,
    pub active_response_tab: ResponseTab, // Active response tab
    pub active_body_view_mode: BodyViewMode, // Active body view mode (Pretty, Source, Preview, Raw)
    pub raw_response_body: String,        // Original raw response body before formatting
    pub loading: bool,
    pub error_message: Option<String>, // Error message when request fails
    pub toast: Option<Toast>,
    pub context_menu: Option<ContextMenu>,
    pub renaming_item: Option<(Vec<usize>, String, String)>, // path, original name, current name being edited
    pub mouse_position: (f32, f32),                          // Track current mouse position
    pub search_query: String,                                // search query for filtering requests
    pub open_tabs: Vec<RequestTabItem>,                      // Open request tabs
    pub active_tab_index: Option<usize>,                     // Currently active tab index
    pub drag_state: Option<DragState>,                       // Tab drag state
    pub rename_input_id: Id,                                 // ID for the rename text input
    pub tab_press_state: Option<TabPressState>,              // Track tab press for timing
    pub current_environment: Environment,                    // Current selected environment
    pub show_environment_dialog: bool, // Whether to show environment management dialog
    pub response_body_content: text_editor::Content, // Text editor content for response body
    pub request_body_content: text_editor::Content, // Text editor content for request body
    pub language: Language,            // Current UI language
    pub translations: Translations,    // Translation strings
    pub show_settings_dialog: bool,    // Whether to show settings dialog
    pub save_directory: String,        // Directory to save collections and requests
    pub ai_config: AiConfig,           // AI configuration
    pub ai_client: Option<AiClient>,   // AI client instance (None until first use)
    pub show_ai_fill_dialog: bool,     // Whether to show AI Fill dialog
    pub ai_fill_input_content: text_editor::Content, // Input content for AI Fill dialog
    pub ai_fill_loading: bool,         // Whether AI Fill is loading
    pub body_format_cache: HashMap<(Uuid, BodyFormat), String>, // Cache for different body formats per request
}

impl Requiem {
    pub fn new() -> Self {
        // Load configuration from file
        let config = crate::config::Config::load();

        // Initialize language and translations from config
        let language = config.language;
        let translations = Translations::new(language);

        // Use save directory and AI config from config
        let save_directory = config.save_directory.clone();
        let ai_config = config.ai_config.clone();

        // Try to load collections from disk
        let collections = match crate::storage::load_collections(&save_directory) {
            Ok(loaded_collections) => {
                if loaded_collections.is_empty() {
                    tracing::info!("No saved collections found, starting with empty collections");
                } else {
                    tracing::info!("Loaded {} collections from disk", loaded_collections.len());
                }
                loaded_collections
            }
            Err(e) => {
                tracing::error!("Failed to load collections: {}", e);
                vec![]
            }
        };

        // Get first request for initial tab (if available)
        let (open_tabs, selected_request, selected_collection) =
            if let Some(first_coll) = collections.first() {
                if let Some(CollectionItem::Request(first_req)) = first_coll.items.first() {
                    let first_tab = RequestTabItem {
                        id: first_req.id,
                        name: first_req.name.clone(),
                        request_path: Some(vec![0, 0]),
                        is_modified: false,
                        is_new: false,
                        draft_request: None,
                        parent_path: None,
                    };
                    (vec![first_tab], Some(vec![0, 0]), Some(0))
                } else {
                    (vec![], None, Some(0))
                }
            } else {
                (vec![], None, None)
            };

        let active_tab_index = if open_tabs.is_empty() { None } else { Some(0) };

        Self {
            collections,
            selected_collection,
            selected_request,
            active_tab: RequestTab::Body,
            response: None,
            active_response_tab: ResponseTab::Body,
            active_body_view_mode: BodyViewMode::Raw,
            raw_response_body: String::new(),
            loading: false,
            error_message: None,
            toast: None,
            context_menu: None,
            renaming_item: None,
            mouse_position: (0.0, 0.0),
            search_query: String::new(),
            open_tabs,
            active_tab_index,
            drag_state: None,
            rename_input_id: Id::unique(),
            tab_press_state: None,
            current_environment: Environment::default(),
            show_environment_dialog: false,
            response_body_content: text_editor::Content::new(),
            request_body_content: text_editor::Content::new(),
            language,
            translations,
            show_settings_dialog: false,
            save_directory,
            ai_config,
            ai_client: None, // Lazy initialization
            show_ai_fill_dialog: false,
            ai_fill_input_content: text_editor::Content::new(),
            ai_fill_loading: false,
            body_format_cache: HashMap::new(),
        }
    }

    pub fn get_current_request(&self) -> Option<&Request> {
        // Check if active tab is an unsaved new request
        if let Some(active_idx) = self.active_tab_index {
            if let Some(tab) = self.open_tabs.get(active_idx) {
                if tab.is_new {
                    return tab.draft_request.as_ref();
                }
            }
        }

        // Otherwise get from collection
        let path = self.selected_request.as_ref()?;
        self.get_item_by_path(path).and_then(|item| {
            if let CollectionItem::Request(req) = item {
                Some(req)
            } else {
                None
            }
        })
    }

    pub fn get_current_request_mut(&mut self) -> Option<&mut Request> {
        // Check if active tab is an unsaved new request
        if let Some(active_idx) = self.active_tab_index {
            if self
                .open_tabs
                .get(active_idx)
                .map(|t| t.is_new)
                .unwrap_or(false)
            {
                // Return draft request from tab
                return self
                    .open_tabs
                    .get_mut(active_idx)
                    .and_then(|tab| tab.draft_request.as_mut());
            }
        }

        // Otherwise get from collection
        let path = self.selected_request.clone()?;
        self.get_item_by_path_mut(&path).and_then(|item| {
            if let CollectionItem::Request(req) = item {
                Some(req)
            } else {
                None
            }
        })
    }

    pub fn get_item_by_path(&self, path: &[usize]) -> Option<&CollectionItem> {
        navigation::get_item_by_path(&self.collections, path)
    }

    pub fn get_item_by_path_mut(&mut self, path: &[usize]) -> Option<&mut CollectionItem> {
        navigation::get_item_by_path_mut(&mut self.collections, path)
    }

    /// Save a specific collection to disk
    pub fn save_collection(&self, collection_index: usize) -> Result<(), String> {
        if let Some(collection) = self.collections.get(collection_index) {
            crate::storage::save_collection(&self.save_directory, collection)
        } else {
            Err(format!(
                "Collection at index {} not found",
                collection_index
            ))
        }
    }

    /// Save all collections to disk
    pub fn save_all_collections(&self) -> Result<(), String> {
        for collection in &self.collections {
            crate::storage::save_collection(&self.save_directory, collection)?;
        }
        Ok(())
    }

    /// Delete a collection from disk
    pub fn delete_collection_file(&self, collection_id: &uuid::Uuid) -> Result<(), String> {
        crate::storage::delete_collection(&self.save_directory, collection_id)
    }

    /// View function for iced application
    pub fn view(&self) -> Element<'_, Message> {
        crate::ui::view(self)
    }

    /// Subscription function for iced application
    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(Self::handle_event)
    }

    /// Handle events for the application
    fn handle_event(
        event: Event,
        status: event::Status,
        _window: iced::window::Id,
    ) -> Option<Message> {
        // Always handle mouse events globally, even if captured
        if let Event::Mouse(mouse::Event::CursorMoved { position }) = event {
            return Some(Message::MouseMoved(position.x, position.y));
        }

        // Only handle other events if not captured by widgets
        if matches!(status, event::Status::Captured) {
            return None;
        }

        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => match &key {
                keyboard::Key::Character(c)
                    if modifiers.contains(keyboard::Modifiers::CTRL) && c == "s" =>
                {
                    return Some(Message::SaveRequest);
                }
                keyboard::Key::Named(keyboard::key::Named::Escape) => {
                    return Some(Message::CancelRename);
                }
                keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                    return Some(Message::MoveActiveTabLeft);
                }
                keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                    return Some(Message::MoveActiveTabRight);
                }
                _ => {}
            },
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                // End drag when left mouse button is released
                // Also trigger tab press end with current mouse position
                return Some(Message::TabDragEnd);
            }
            _ => {}
        }
        None
    }
}

impl Default for Requiem {
    fn default() -> Self {
        Self::new()
    }
}

impl I18n for Requiem {
    fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.translations.get(key)
    }

    fn language(&self) -> Language {
        self.language
    }
}
