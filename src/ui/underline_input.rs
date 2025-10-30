use iced::widget::{container, text_input, Column, Id};
use iced::{Border, Element, Length, Padding, Theme};

/// A text input component with an underline style (dashed line below the text)
pub fn underline_input<'a, Message: 'a>(
    placeholder: &str,
    value: &str,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone,
{
    let id = Id::unique();

    let mut input = text_input(placeholder, value)
        .id(id.clone())
        .on_input(on_input)
        .padding(Padding::new(4.0).left(0.0).right(0.0))
        .size(13)
        .style(|theme: &Theme, status| {
            // Remove all borders from the input
            text_input::Style {
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..text_input::default(theme, status)
            }
        });

    if let Some(submit_msg) = on_submit {
        input = input.on_submit(submit_msg);
    }

    let input_container = container(input)
        .padding(Padding::new(0.0).bottom(2.0))
        .width(Length::Fill);

    // Create the underline using a container with bottom border
    let underline = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_theme: &Theme| container::Style {
            border: Border {
                color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    Column::new()
        .spacing(0)
        .push(input_container)
        .push(underline)
        .into()
}

/// A text input component with an underline style and configurable size
pub fn underline_input_sized<'a, Message: 'a>(
    id: Id,
    placeholder: &str,
    value: &str,
    size: u16,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Option<Message>,
) -> Element<'a, Message>
where
    Message: Clone,
{
    let mut input = text_input(placeholder, value)
        .id(id)
        .on_input(on_input)
        .padding(Padding::new(4.0).left(0.0).right(0.0))
        .size(size as f32)
        .style(|theme: &Theme, status| {
            // Remove all borders from the input
            text_input::Style {
                border: Border {
                    color: iced::Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..text_input::default(theme, status)
            }
        });

    if let Some(submit_msg) = on_submit {
        input = input.on_submit(submit_msg);
    }

    let input_container = container(input)
        .padding(Padding::new(0.0).bottom(2.0))
        .width(Length::Fill);

    // Create the underline using a container with bottom border
    let underline = container("")
        .width(Length::Fill)
        .height(1)
        .style(|_theme: &Theme| container::Style {
            border: Border {
                color: iced::Color::from_rgb(0.6, 0.6, 0.6),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    Column::new()
        .spacing(0)
        .push(input_container)
        .push(underline)
        .into()
}
