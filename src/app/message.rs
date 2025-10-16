use crate::app::state::ContextMenuTarget;
use crate::models::{BodyFormat, BodyViewMode, EnvironmentOption, HttpMethod, RequestTab, Response, ResponseTab};
use crate::ui::toast::Toast;
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    // Request editing
    MethodSelected(HttpMethod),
    UrlChanged(String),
    EnvironmentOptionSelected(EnvironmentOption),
    AiFill,
    SendRequest,
    RequestSent(Result<Response, String>),

    // Headers
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    AddHeader,
    RemoveHeader(usize),

    // Query Params
    ParamKeyChanged(usize, String),
    ParamValueChanged(usize, String),
    AddParam,
    RemoveParam(usize),

    // Cookies
    CookieKeyChanged(usize, String),
    CookieValueChanged(usize, String),
    AddCookie,
    RemoveCookie(usize),

    // Auth
    AuthKeyChanged(usize, String),
    AuthValueChanged(usize, String),
    AddAuthField,
    RemoveAuthField(usize),

    // Body
    BodyChanged(String),
    BodyFormatChanged(BodyFormat),
    FormDataKeyChanged(usize, String),
    FormDataValueChanged(usize, String),
    AddFormDataField,
    RemoveFormDataField(usize),
    RequestBodyAction(text_editor::Action), // Text editor action for request body

    // Tabs
    TabSelected(RequestTab),
    ResponseTabSelected(ResponseTab),
    BodyViewModeSelected(BodyViewMode),

    // Collections and Requests
    SelectRequest(Vec<usize>), // path to the request item
    AddNewRequest(Vec<usize>), // path to parent (collection or folder)
    AddNewFolder(Vec<usize>), // path to parent (collection or folder)
    DeleteItem(Vec<usize>), // path to item
    RenameItem(Vec<usize>, String), // path to item and new name
    ToggleExpanded(Vec<usize>), // path to collection/folder
    AddNewCollection,
    SearchChanged(String), // search query

    // Tabs
    OpenTab(Vec<usize>), // Open request in new tab
    CloseTab(usize), // Close tab by index
    SwitchTab(usize), // Switch to tab by index
    TabDragStart(usize, f32), // Start dragging tab (index, initial x offset)
    TabDragMove(f32), // Mouse moved while dragging (x position)
    TabDragEnd, // End tab dragging and handle tab click
    ReorderTabs(usize, usize), // Reorder tabs (from_index, to_index)
    MoveActiveTabLeft, // Move active tab 3px to the left
    MoveActiveTabRight, // Move active tab 3px to the right
    TabPressStart(usize, f32), // Tab pressed (index, x position)

    // Context menu
    ShowContextMenu(Vec<usize>, f32, f32, ContextMenuTarget), // path, x, y position, target type
    HideContextMenu,
    StartRename(Vec<usize>), // Start renaming an item
    UpdateRenamingText(String), // Update the text while renaming
    ConfirmRename, // Confirm rename (get name from state)
    CancelRename, // Cancel renaming without saving
    MouseMoved(f32, f32), // Track mouse position

    // Save and Toast
    SaveRequest,
    ShowToast(Toast),
    HideToast,

    // Environment Management
    ShowEnvironmentDialog,
    CloseEnvironmentDialog,

    // Response actions
    CopyResponseBody,
    ResponseBodyAction(text_editor::Action), // Text editor action for response body

    // Language switching
    LanguageChanged(crate::i18n::Language),

    // Settings dialog
    ShowSettingsDialog,
    CloseSettingsDialog,
    SaveDirectoryChanged(String),
    BrowseSaveDirectory,
}
