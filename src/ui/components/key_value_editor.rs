use iced::widget::{button, container, pick_list, scrollable, text, text_input, Column, Row};
use iced::{Alignment, Element, Length};

use crate::models::{FormDataParamType, KeyValue};

/// Configuration for the key-value editor component
pub struct KeyValueEditorConfig<'a> {
    pub key_label: &'a str,
    pub value_label: &'a str,
    pub key_placeholder: &'a str,
    pub value_placeholder: &'a str,
    pub add_button_text: &'a str,
}

pub struct FormBodyEditorConfig<'a> {
    pub key_label: &'a str,
    pub value_label: &'a str,
    pub type_label: &'a str,
    pub key_placeholder: &'a str,
    pub value_placeholder: &'a str,
    pub add_button_text: &'a str,
    pub text_option_label: &'a str,
    pub file_option_label: &'a str,
    pub browse_button_text: &'a str,
    pub file_value_placeholder: &'a str,
}

/// A reusable key-value pair editor component with add/remove functionality
///
/// Used for editing params, headers, cookies, auth fields, and form data
#[allow(clippy::too_many_arguments)]
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
    let mut rows = Column::new().spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(container(text(config.key_label).size(12)).width(Length::FillPortion(1)))
        .push(container(text(config.value_label).size(12)).width(Length::FillPortion(2)))
        .push(container(text("").size(12)).width(Length::Fixed(40.0)));

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

        rows = rows.push(row);
    }

    let rows_scrollable = scrollable(rows)
        .height(Length::Fill)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().width(8).scroller_width(8),
        ));

    // Add button
    let add_button = button(text(config.add_button_text).size(13))
        .on_press(on_add)
        .padding([10, 16])
        .style(button::secondary);

    let content = Column::new()
        .spacing(10)
        .height(Length::Fill)
        .push(header_labels)
        .push(container(rows_scrollable).height(Length::Fill))
        .push(add_button);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: 12.0,
            right: 16.0,
            bottom: 12.0,
            left: 16.0,
        })
        .into()
}

#[allow(clippy::too_many_arguments)]
pub fn view_form_body<'a, Message: Clone + 'a, F1, F2, F3, F4, F5>(
    items: &'a [KeyValue],
    config: FormBodyEditorConfig<'a>,
    allow_file_type: bool,
    on_key_changed: F1,
    on_value_changed: F2,
    on_type_changed: F3,
    on_remove: F4,
    on_browse_file: F5,
    on_add: Message,
) -> Element<'a, Message>
where
    F1: Fn(usize, String) -> Message + 'a + Copy,
    F2: Fn(usize, String) -> Message + 'a + Copy,
    F3: Fn(usize, FormDataParamType) -> Message + 'a + Copy,
    F4: Fn(usize) -> Message + 'a + Copy,
    F5: Fn(usize) -> Message + 'a + Copy,
{
    let mut rows = Column::new().spacing(8);

    let header_labels = Row::new()
        .spacing(10)
        .padding([6, 0])
        .push(container(text(config.key_label).size(12)).width(Length::FillPortion(2)))
        .push(container(text(config.value_label).size(12)).width(Length::FillPortion(3)))
        .push(container(text(config.type_label).size(12)).width(Length::Fixed(100.0)))
        .push(container(text("").size(12)).width(Length::Fixed(32.0)));

    let type_options: Vec<FormDataParamType> = if allow_file_type {
        vec![FormDataParamType::Text, FormDataParamType::File]
    } else {
        vec![FormDataParamType::Text]
    };

    for (idx, item) in items.iter().enumerate() {
        let key_input = text_input(config.key_placeholder, &item.key)
            .on_input(move |v| on_key_changed(idx, v))
            .padding([8, 10])
            .size(12);

        let value_control: Element<'a, Message> = if allow_file_type
            && item.param_type == FormDataParamType::File
        {
            let file_path = text_input(config.file_value_placeholder, &item.value)
                .padding([8, 10])
                .size(12)
                .width(Length::Fill);

            let browse_button = button(text(config.browse_button_text).size(11))
                .on_press(on_browse_file(idx))
                .padding([6, 10])
                .style(button::secondary);

            Row::new()
                .spacing(8)
                .align_y(Alignment::Center)
                .push(file_path)
                .push(browse_button)
                .width(Length::Fill)
                .into()
        } else {
            text_input(config.value_placeholder, &item.value)
                .on_input(move |v| on_value_changed(idx, v))
                .padding([8, 10])
                .size(12)
                .width(Length::Fill)
                .into()
        };

        let type_picker = pick_list(Some(item.param_type), type_options.clone(), move |t| {
            match t {
                FormDataParamType::Text => config.text_option_label.to_string(),
                FormDataParamType::File => config.file_option_label.to_string(),
            }
        })
        .on_select(move |t| on_type_changed(idx, t))
        .padding([6, 8])
        .width(Length::Fixed(100.0));

        let remove_button = button(text("×").size(14))
            .on_press(on_remove(idx))
            .padding([6, 8])
            .style(button::text);

        let row = Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(2)))
            .push(container(value_control).width(Length::FillPortion(3)))
            .push(type_picker)
            .push(remove_button);

        rows = rows.push(row);
    }

    let rows_scrollable = scrollable(rows)
        .height(Length::Fill)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().width(8).scroller_width(8),
        ));

    let add_button = button(text(config.add_button_text).size(12))
        .on_press(on_add)
        .padding([8, 12])
        .style(button::secondary);

    let content = Column::new()
        .spacing(8)
        .height(Length::Fill)
        .push(header_labels)
        .push(container(rows_scrollable).height(Length::Fill))
        .push(add_button);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: 10.0,
            right: 16.0,
            bottom: 10.0,
            left: 16.0,
        })
        .into()
}
