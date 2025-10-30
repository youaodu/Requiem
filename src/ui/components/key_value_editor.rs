use iced::widget::{button, container, text, text_input, Column, Row};
use iced::{Alignment, Element, Length};

/// Configuration for the key-value editor component
pub struct KeyValueEditorConfig<'a> {
    pub key_label: &'a str,
    pub value_label: &'a str,
    pub key_placeholder: &'a str,
    pub value_placeholder: &'a str,
    pub add_button_text: &'a str,
}

/// A reusable key-value pair editor component with add/remove functionality
///
/// Used for editing params, headers, cookies, auth fields, and form data
pub fn view<'a, Message: Clone + 'a, T, F1, F2, F3, F4, F5>(
    items: &'a [T],
    config: KeyValueEditorConfig<'a>,
    get_key: F1,
    get_value: F2,
    on_key_changed: F3,
    on_value_changed: F4,
    on_remove: F5,
    on_add: Message,
) -> Element<'a, Message>
where
    F1: Fn(&T) -> &str + 'a,
    F2: Fn(&T) -> &str + 'a,
    F3: Fn(usize, String) -> Message + 'a + Copy,
    F4: Fn(usize, String) -> Message + 'a + Copy,
    F5: Fn(usize) -> Message + 'a + Copy,
{
    let mut column = Column::new().spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(container(text(config.key_label).size(12)).width(Length::FillPortion(1)))
        .push(container(text(config.value_label).size(12)).width(Length::FillPortion(2)))
        .push(container(text("").size(12)).width(Length::Fixed(40.0)));

    column = column.push(header_labels);

    // Data rows
    for (idx, item) in items.iter().enumerate() {
        let key_input = text_input(config.key_placeholder, get_key(item))
            .on_input(move |v| on_key_changed(idx, v))
            .padding(10)
            .size(13);

        let value_input = text_input(config.value_placeholder, get_value(item))
            .on_input(move |v| on_value_changed(idx, v))
            .padding(10)
            .size(13);

        let remove_button = button(text("×").size(16))
            .on_press(on_remove(idx))
            .padding([8, 12])
            .style(button::text);

        let row = Row::new()
            .spacing(16)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(1)))
            .push(container(value_input).width(Length::FillPortion(2)))
            .push(remove_button);

        column = column.push(row);
    }

    // Add button
    let add_button = button(text(config.add_button_text).size(13))
        .on_press(on_add)
        .padding([10, 16])
        .style(button::secondary);

    column = column.push(add_button);

    container(column).padding(20).into()
}
