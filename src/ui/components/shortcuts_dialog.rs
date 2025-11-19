use iced::widget::{button, column, container, row, scrollable, text, Column, Row};
use iced::{Alignment, Element, Length};

use crate::app::{Message, Requiem};
use crate::i18n::I18n;

/// Render the shortcuts help dialog
pub fn shortcuts_dialog<'a>(app: &'a Requiem) -> Element<'a, Message> {
    let registry = &crate::models::ShortcutRegistry::new();

    // Title
    let title = text(app.t("shortcuts_dialog_title"))
        .size(24)
        .style(text::primary);

    // Create sections for each category
    let mut content_column = column![title].spacing(20).padding(20);

    for (category, shortcuts) in registry.by_category() {
        if shortcuts.is_empty() {
            continue;
        }

        // Category header
        let category_name = match app.language {
            crate::i18n::Language::English => category.display_name(),
            crate::i18n::Language::Chinese => category.display_name_zh(),
        };

        let category_header = text(category_name)
            .size(18)
            .style(text::secondary);

        let mut category_rows = Column::new().spacing(10);

        // Add shortcuts for this category
        for (action, shortcut) in shortcuts {
            let action_name = match app.language {
                crate::i18n::Language::English => action.display_name(),
                crate::i18n::Language::Chinese => action.display_name_zh(),
            };

            let shortcut_text = shortcut.display_string();

            let row_widget: Row<'a, Message> = row![
                text(action_name).width(Length::Fill),
                container(
                    text(shortcut_text)
                        .size(14)
                        .style(|_theme| {
                            text::Style {
                                color: Some(iced::Color::from_rgb(0.4, 0.4, 0.4)),
                            }
                        })
                )
                .padding([4, 8])
                .style(|_theme: &iced::Theme| {
                    container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.95, 0.95, 0.95
                        ))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            ]
            .spacing(10)
            .align_y(Alignment::Center);

            category_rows = category_rows.push(row_widget);
        }

        content_column = content_column.push(category_header).push(category_rows);
    }

    // Close button
    let close_button = button(text(app.t("close")).size(16))
        .padding([8, 16])
        .on_press(Message::CloseShortcutsDialog);

    let button_row = row![close_button]
        .spacing(10)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    content_column = content_column.push(button_row);

    // Scrollable content
    let scrollable_content = scrollable(content_column).height(Length::Fill);

    // Main container
    container(scrollable_content)
        .width(600)
        .height(500)
        .padding(20)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .into()
}
