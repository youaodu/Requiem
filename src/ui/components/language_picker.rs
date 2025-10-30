use iced::widget::{container, pick_list};
use iced::Element;

use crate::app::Message;
use crate::i18n::Language;

pub fn view(current_language: Language) -> Element<'static, Message> {
    let languages = Language::all();

    let language_selector = pick_list(languages, Some(current_language), Message::LanguageChanged)
        .placeholder("Select Language")
        .padding([8, 12]);

    container(language_selector).padding([4, 8]).into()
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
