use iced::widget::{button, container, text, Row};
use iced::{Element, Length};

/// A reusable option button group component
///
/// Displays a row of buttons where one can be selected at a time
/// Used for body format selector, response tab selector, etc.
pub fn view<'a, Message: Clone + 'a, T: Clone + PartialEq + 'a>(
    options: Vec<T>,
    current_selection: T,
    get_label: impl Fn(&T) -> String + 'a,
    on_select: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message> {
    let mut row = Row::new().spacing(4).padding([12, 16]);

    for option in options {
        let is_selected = option == current_selection;
        let label = get_label(&option);

        let btn = button(text(label).size(12))
            .on_press(on_select(option.clone()))
            .padding([8, 14])
            .style(if is_selected {
                button::primary
            } else {
                button::text
            });

        row = row.push(btn);
    }

    // Add flexible space to push buttons to the left
    row = row.push(container(text("")).width(Length::Fill));

    row.into()
}

/// A variant with custom styling function
pub fn view_with_style<'a, Message: Clone + 'a, T: Clone + PartialEq + 'a>(
    options: Vec<T>,
    current_selection: T,
    get_label: impl Fn(&T) -> String + 'a,
    on_select: impl Fn(T) -> Message + 'a,
    button_style: impl Fn(bool) -> fn(&iced::Theme, button::Status) -> button::Style + 'a,
) -> Element<'a, Message> {
    let mut row = Row::new().spacing(4).padding([12, 16]);

    for option in options {
        let is_selected = option == current_selection;
        let label = get_label(&option);

        let btn = button(text(label).size(12))
            .on_press(on_select(option.clone()))
            .padding([8, 14])
            .style(button_style(is_selected));

        row = row.push(btn);
    }

    // Add flexible space to push buttons to the left
    row = row.push(container(text("")).width(Length::Fill));

    row.into()
}
