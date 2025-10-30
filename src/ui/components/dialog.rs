use iced::widget::{button, container, text, Column, Row};
use iced::{Color, Element, Length};

/// A reusable dialog container component with consistent styling
///
/// Provides a centered modal dialog with:
/// - White background
/// - Rounded corners
/// - Border
/// - Title, content, and button sections
pub fn view<'a, Message: Clone + 'a>(
    title: &'a str,
    content: Element<'a, Message>,
    close_button_text: &'a str,
    on_close: Message,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    let title_text = text(title).size(18);

    let close_button = button(text(close_button_text).size(14))
        .on_press(on_close)
        .padding([10, 20])
        .style(button::primary);

    let dialog_content = Column::new()
        .spacing(20)
        .push(title_text)
        .push(content)
        .push(
            Row::new()
                .spacing(10)
                .push(container(text("")).width(Length::Fill))
                .push(close_button),
        );

    container(dialog_content)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
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

/// A reusable dialog container with multiple action buttons
#[allow(clippy::type_complexity)]
pub fn view_with_buttons<'a, Message: Clone + 'a>(
    title: &'a str,
    content: Element<'a, Message>,
    buttons: Vec<(
        String,
        Message,
        fn(&iced::Theme, button::Status) -> button::Style,
    )>,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    let title_text = text(title).size(18);

    let mut button_row = Row::new()
        .spacing(10)
        .push(container(text("")).width(Length::Fill));

    for (label, message, style) in buttons {
        let btn = button(text(label).size(14))
            .on_press(message)
            .padding([10, 20])
            .style(style);
        button_row = button_row.push(btn);
    }

    let dialog_content = Column::new()
        .spacing(20)
        .push(title_text)
        .push(content)
        .push(button_row);

    container(dialog_content)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
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
