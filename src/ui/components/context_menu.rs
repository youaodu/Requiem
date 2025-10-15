use iced::widget::{button, column, container, mouse_area, row, text, Space};
use iced::{Element, Length};

use crate::app::Message;
use crate::app::state::ContextMenuTarget;
use crate::i18n::Translations;

pub fn view<'a>(path: &[usize], x: f32, y: f32, target: &ContextMenuTarget, translations: &'a Translations) -> Element<'a, Message> {
    let menu_item_style = |_theme: &iced::Theme, status: button::Status| {
        let base = button::Style {
            background: None,
            text_color: iced::Color::from_rgb(0.2, 0.2, 0.2),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        };

        match status {
            button::Status::Hovered => button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.95, 1.0))),
                ..base
            },
            _ => base,
        }
    };

    let path = path.to_vec();

    let mut menu_items = column![].spacing(2).padding(4);

    // Add menu items based on target type
    match target {
        ContextMenuTarget::Request => {
            // For requests, only show Rename and Delete
            menu_items = menu_items.push(
                button(text(translations.get("ctx_rename")).size(12))
                    .on_press(Message::StartRename(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
            menu_items = menu_items.push(
                button(text(translations.get("ctx_delete")).size(12))
                    .on_press(Message::DeleteItem(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
        }
        ContextMenuTarget::Folder | ContextMenuTarget::Collection => {
            // For folders and collections, show all options
            menu_items = menu_items.push(
                button(text(translations.get("ctx_new_request")).size(12))
                    .on_press(Message::AddNewRequest(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
            menu_items = menu_items.push(
                button(text(translations.get("ctx_new_folder")).size(12))
                    .on_press(Message::AddNewFolder(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
            menu_items = menu_items.push(
                button(text(translations.get("ctx_rename")).size(12))
                    .on_press(Message::StartRename(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
            menu_items = menu_items.push(
                button(text(translations.get("ctx_delete")).size(12))
                    .on_press(Message::DeleteItem(path.clone()))
                    .width(Length::Fixed(150.0))
                    .padding([6, 12])
                    .style(menu_item_style)
            );
        }
    }

    let menu_container = container(menu_items)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::WHITE)),
            border: iced::Border {
                color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 4.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        });

    // Position the menu at the cursor position using spacers
    let vertical_spacer = Space::new(Length::Fixed(0.0), Length::Fixed(y));
    let horizontal_spacer = Space::new(Length::Fixed(x), Length::Fixed(0.0));

    let positioned_menu = column![
        vertical_spacer,
        row![horizontal_spacer, menu_container]
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    // Wrap in mouse_area to detect clicks outside the menu
    mouse_area(positioned_menu)
        .on_press(Message::HideContextMenu)
        .into()
}
