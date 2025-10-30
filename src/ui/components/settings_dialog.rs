use iced::widget::{button, scrollable, text, text_input, Column, Row};
use iced::{Element, Length};

use crate::app::Message;
use crate::i18n::{Language, Translations};
use crate::models::{AiConfig, AiEngine};

use super::{ai_engine_picker, dialog, language_picker};

pub fn view<'a>(
    current_language: Language,
    save_directory: &str,
    ai_config: &AiConfig,
    translations: &'a Translations,
) -> Element<'a, Message> {
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
                        .width(Length::Fill),
                )
                .push(
                    button(text(translations.get("browse")).size(14))
                        .on_press(Message::BrowseSaveDirectory)
                        .padding([10, 15]),
                ),
        );

    let ai_section = Column::new()
        .spacing(8)
        .push(text(translations.get("ai_config")).size(14))
        .push(ai_engine_picker::view(ai_config.engine));

    // Show OpenAI config fields when OpenAI is selected
    let ai_section = if ai_config.engine == AiEngine::OpenAI {
        ai_section
            .push(
                Column::new()
                    .spacing(8)
                    .push(text(translations.get("ai_api_url")).size(12))
                    .push(
                        text_input("", &ai_config.openai_config.api_url)
                            .padding(10)
                            .size(14)
                            .width(Length::Fill)
                            .on_input(Message::AiApiUrlChanged),
                    ),
            )
            .push(
                Column::new()
                    .spacing(8)
                    .push(text(translations.get("ai_api_key")).size(12))
                    .push(
                        text_input("", &ai_config.openai_config.api_key)
                            .padding(10)
                            .size(14)
                            .width(Length::Fill)
                            .on_input(Message::AiApiKeyChanged)
                            .secure(true),
                    ),
            )
            .push(
                Column::new()
                    .spacing(8)
                    .push(text(translations.get("ai_model")).size(12))
                    .push(
                        text_input("", &ai_config.openai_config.model)
                            .padding(10)
                            .size(14)
                            .width(Length::Fill)
                            .on_input(Message::AiModelChanged),
                    ),
            )
    } else {
        ai_section
    };

    let content = Column::new()
        .spacing(20)
        .push(language_section)
        .push(save_directory_section)
        .push(ai_section);

    let scrollable_content = scrollable(content).height(Length::Fill);

    dialog::view(
        translations.get("settings"),
        scrollable_content.into(),
        translations.get("close"),
        Message::CloseSettingsDialog,
        600.0,
        550.0,
    )
}
