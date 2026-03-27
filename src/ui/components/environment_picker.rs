use iced::widget::pick_list;
use iced::{Element, Length};

use crate::app::Message;
use crate::models::{Environment, EnvironmentOption};

pub fn view<'a>(current_env: Environment) -> Element<'a, Message> {
    pick_list(
        Some(EnvironmentOption::Environment(current_env)),
        EnvironmentOption::all(),
        |option: &EnvironmentOption| option.to_string(),
    )
    .on_select(Message::EnvironmentOptionSelected)
    .width(Length::Fixed(140.0))
    .padding(8)
    .into()
}
