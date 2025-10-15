use iced::widget::{button, container, text, text_input, Column, Row};
use iced::{Color, Element, Length};

use crate::app::Message;
use crate::i18n::{Language, Translations};

use super::language_picker;

pub fn view<'a>(current_language: Language, save_directory: &str, translations: &'a Translations) -> Element<'a, Message> {
    let title = text(translations.get("settings")).size(18);

    let language_section = Column::new()
        .spacing(8)
        .push(text(translations.get("language")).size(14))
        .push(language_picker::view(current_language));

    let save_directory_section = Column::new()
        .spacing(8)
        .push(text(translations.get("save_directory")).size(14))
        .push(
            Row::new()
                .spacing(10)
                .push(
                    text_input("", save_directory)
                        .padding(10)
                        .size(14)
                        .width(Length::Fill)
                )
                .push(
                    button(text(translations.get("browse")).size(14))
                        .on_press(Message::BrowseSaveDirectory)
                        .padding([10, 15])
                )
        );

    let content = Column::new()
        .spacing(20)
        .padding(20)
        .push(title)
        .push(language_section)
        .push(save_directory_section);

    let close_button = button(text(translations.get("close")).size(14))
        .on_press(Message::CloseSettingsDialog)
        .padding([10, 20])
        .style(button::primary);

    let dialog_content = Column::new()
        .spacing(20)
        .push(content)
        .push(
            Row::new()
                .spacing(10)
                .push(container(text("")).width(Length::Fill))
                .push(close_button),
        );

    container(dialog_content)
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(380.0))
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}
