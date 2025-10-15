use iced::widget::{button, text, text_input, Column, Row};
use iced::{Element, Length};

use crate::app::Message;
use crate::i18n::{Language, Translations};

use super::{dialog, language_picker};

pub fn view<'a>(current_language: Language, save_directory: &str, translations: &'a Translations) -> Element<'a, Message> {
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
        .push(language_section)
        .push(save_directory_section);

    dialog::view(
        translations.get("settings"),
        content.into(),
        translations.get("close"),
        Message::CloseSettingsDialog,
        500.0,
        380.0,
    )
}
