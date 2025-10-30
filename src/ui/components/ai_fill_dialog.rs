use iced::widget::{button, scrollable, text, Column};
use iced::{Element, Length};

use crate::app::Message;

use super::{dialog, textarea};

pub fn view<'a>(
    input_content: &'a iced::widget::text_editor::Content,
    translations: &'a crate::i18n::Translations,
    is_loading: bool,
) -> Element<'a, Message> {
    let hint = text(translations.get("ai_fill_hint"))
        .size(13)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.5));

    let text_area = textarea::textarea_builder(input_content, Message::AiFillInputAction)
        .placeholder(translations.get("ai_fill_input_placeholder"))
        .height(220.0)
        .build();

    let mut inner_content = Column::new().spacing(12).push(hint);

    // Show loading indicator or text area based on loading state
    if is_loading {
        let loading_text = text(translations.get("ai_fill_loading"))
            .size(14)
            .color(iced::Color::from_rgb(0.4, 0.4, 0.4));
        inner_content = inner_content.push(loading_text);
    } else {
        inner_content = inner_content.push(text_area);
    }

    let content = scrollable(inner_content).height(Length::Fill);

    let buttons = if is_loading {
        // Only show disabled confirm button when loading
        vec![(
            translations.get("confirm").to_string(),
            Message::ConfirmAiFill, // This won't be triggered as button is disabled
            button::primary as fn(&iced::Theme, button::Status) -> button::Style,
        )]
    } else {
        vec![
            (
                translations.get("cancel").to_string(),
                Message::CloseAiFillDialog,
                button::secondary as fn(&iced::Theme, button::Status) -> button::Style,
            ),
            (
                translations.get("confirm").to_string(),
                Message::ConfirmAiFill,
                button::primary as fn(&iced::Theme, button::Status) -> button::Style,
            ),
        ]
    };

    dialog::view_with_buttons(
        translations.get("ai_fill_dialog_title"),
        content.into(),
        buttons,
        550.0,
        450.0,
    )
}
