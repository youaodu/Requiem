use iced::widget::pick_list;
use iced::{Element, Length};

use crate::app::Message;
use crate::models::AiEngine;

pub fn view(current_engine: AiEngine) -> Element<'static, Message> {
    pick_list(
        AiEngine::all(),
        Some(current_engine),
        Message::AiEngineChanged,
    )
    .placeholder("Select AI Engine")
    .padding(10)
    .width(Length::Fill)
    .into()
}
