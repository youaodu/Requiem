use iced::advanced::text::highlighter;
use iced::widget::{container, text_editor};
use iced::{Color, Element, Length};

use crate::ui::body_highlighter::{BodyHighlighter, BodyLanguage, HighlighterSettings};

/// A reusable code editor component with syntax highlighting support
pub fn view<'a, Message: Clone + 'a>(
    content: &'a text_editor::Content,
    language: BodyLanguage,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message> {
    let settings = HighlighterSettings { language };

    let editor = text_editor(content)
        .on_action(on_action)
        .padding(10)
        .height(Length::Fill)
        .highlight_with::<BodyHighlighter>(settings, |highlight, _theme| highlighter::Format {
            color: Some(*highlight),
            font: None,
        })
        .style(|_theme, _status| {
            use iced::widget::text_editor::Style;
            Style {
                background: iced::Background::Color(Color::WHITE),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(0.0),
                },
                placeholder: Color::from_rgb(0.7, 0.7, 0.7),
                value: Color::from_rgb(0.2, 0.2, 0.2),
                selection: Color::from_rgb(0.6, 0.7, 0.9),
            }
        });

    container(editor)
        .padding([8, 16])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
