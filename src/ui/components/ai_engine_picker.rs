use iced::widget::pick_list;
use iced::{Element, Length};

use crate::app::Message;
use crate::models::AiEngine;

pub fn view(current_engine: AiEngine) -> Element<'static, Message> {
    pick_list(
        Some(current_engine),
        AiEngine::all(),
        |engine: &AiEngine| engine.to_string(),
    )
    .on_select(Message::AiEngineChanged)
    .placeholder("Select AI Engine")
    .padding(10)
    .width(Length::Fill)
    .into()
}
