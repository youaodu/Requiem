use iced::widget::{button, container, mouse_area, scrollable, text, text_input, Column, Row};
use iced::{Alignment, Element, Length, Padding};

use crate::app::Message;
use crate::app::state::ContextMenuTarget;
use crate::i18n::Translations;
use crate::models::{Collection, CollectionItem};
use crate::ui::{icons, underline_input};

pub fn view<'a>(
    collections: &'a [Collection],
    selected_request: Option<&Vec<usize>>,
    _context_menu: Option<&crate::app::state::ContextMenu>,
    renaming_item: Option<&(Vec<usize>, String, String)>,
    search_query: &'a str,
    rename_input_id: &text_input::Id,
    translations: &'a Translations,
) -> Element<'a, Message> {
    let mut sidebar = Column::new()
        .spacing(0)
        .width(Length::Fill);

    // Search input with add button
    let search_input = text_input(translations.get("search_placeholder"), search_query)
        .padding([8, 12])
        .size(13)
        .on_input(Message::SearchChanged);

    let search_row = Row::new()
        .spacing(8)
        .padding(Padding::new(16.0).top(16.0).bottom(12.0))
        .push(
            container(search_input)
                .width(Length::Fill)
        )
        .push(
            button(text("+").size(18))
                .padding([4, 10])
                .style(button::text)
                .on_press(Message::AddNewCollection)
        );

    sidebar = sidebar.push(search_row);

    // Interface management section
    let interface_header = Row::new()
        .spacing(8)
        .padding([8, 16])
        .push(text(translations.get("api_information")).size(16));

    sidebar = sidebar.push(interface_header);

    // Collections list
    let mut collections_column = Column::new().spacing(4).padding([0, 8]);

    for (coll_idx, collection) in collections.iter().enumerate() {
        let coll_path = vec![coll_idx];

        // Check if this collection is being renamed
        let is_renaming = renaming_item.as_ref().map(|(path, _, _)| path) == Some(&coll_path);

        // Collection header
        let expand_icon = if collection.expanded { "v" } else { ">" };

        let collection_content: Element<'_, Message> = if is_renaming {
            let current_name = renaming_item.as_ref().map(|(_, _, name)| name.as_str()).unwrap_or(&collection.name);
            Row::new()
                .spacing(8)
                .padding([8, 8])
                .push(text(expand_icon).size(12))
                .push(
                    underline_input::underline_input_sized(
                        rename_input_id.clone(),
                        translations.get("name_placeholder"),
                        current_name,
                        13,
                        |new_text| Message::UpdateRenamingText(new_text),
                        Some(Message::ConfirmRename),
                    )
                )
                .push(text(format!("({})", collection.items.len())).size(11))
                .into()
        } else {
            let collection_row = Row::new()
                .spacing(8)
                .width(Length::Fill)
                .push(
                    button(
                        Row::new()
                            .spacing(8)
                            .padding([8, 8])
                            .push(text(expand_icon).size(12))
                            .push(text(&collection.name).size(13))
                            .push(text(format!("({})", collection.items.len())).size(11))
                    )
                    .on_press(Message::ToggleExpanded(coll_path.clone()))
                    .width(Length::Fill)
                    .style(button::text)
                )
                .push(
                    button(text("+").size(16))
                        .padding([4, 8])
                        .style(button::text)
                        .on_press(Message::AddNewRequest(coll_path.clone()))
                );

            mouse_area(collection_row)
                .on_right_press(Message::ShowContextMenu(coll_path.clone(), 0.0, 0.0, ContextMenuTarget::Collection))
                .into()
        };

        collections_column = collections_column.push(collection_content);

        // Show items if expanded
        if collection.expanded {
            collections_column = render_items(
                &collection.items,
                &coll_path,
                selected_request,
                0,
                collections_column,
                renaming_item,
                rename_input_id,
                translations,
            );
        }
    }

    // Wrap collections in scrollable
    let scrollable_content = scrollable(collections_column)
        .height(Length::Fill);

    // Settings button at the bottom
    let settings_button = button(
        Row::new()
            .spacing(8)
            .padding([8, 16])
            .align_y(Alignment::Center)
            .push(icons::settings_icon(16))
            .push(text(translations.get("settings")).size(13))
    )
    .on_press(Message::ShowSettingsDialog)
    .width(Length::Fill)
    .style(button::text);

    // Build final layout: header + scrollable content + settings button
    let final_column = Column::new()
        .push(sidebar)
        .push(scrollable_content)
        .push(settings_button);

    container(final_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn render_items<'a>(
    items: &'a [CollectionItem],
    parent_path: &[usize],
    selected_request: Option<&Vec<usize>>,
    depth: usize,
    mut column: Column<'a, Message>,
    renaming_item: Option<&(Vec<usize>, String, String)>,
    rename_input_id: &text_input::Id,
    translations: &'a Translations,
) -> Column<'a, Message> {
    // Separate folders and requests
    let mut folders: Vec<(usize, &CollectionItem)> = Vec::new();
    let mut requests: Vec<(usize, &CollectionItem)> = Vec::new();

    for (item_idx, item) in items.iter().enumerate() {
        match item {
            CollectionItem::Folder(_) => folders.push((item_idx, item)),
            CollectionItem::Request(_) => requests.push((item_idx, item)),
        }
    }

    // Render folders first, then requests
    let all_items = folders.into_iter().chain(requests.into_iter());

    for (item_idx, item) in all_items {
        let mut item_path = parent_path.to_vec();
        item_path.push(item_idx);

        let indent = 28.0 + (depth as f32 * 16.0);

        match item {
            CollectionItem::Request(req) => {
                let is_selected = selected_request == Some(&item_path);
                let req_name = if req.name.is_empty() {
                    translations.get("unnamed_request").to_string()
                } else {
                    req.name.clone()
                };

                let is_renaming = renaming_item.as_ref().map(|(path, _, _)| path) == Some(&item_path);

                let request_content: Element<'a, Message> = if is_renaming {
                    let current_name = renaming_item.as_ref().map(|(_, _, name)| name.as_str()).unwrap_or(&req_name);
                    Row::new()
                        .spacing(8)
                        .padding(Padding::new(8.0).top(6.0).bottom(6.0).left(indent))
                        .push(text(req.method.as_str()).size(11))
                        .push(
                            underline_input::underline_input_sized(
                                rename_input_id.clone(),
                                translations.get("name_placeholder"),
                                current_name,
                                12,
                                |new_text| Message::UpdateRenamingText(new_text),
                                Some(Message::ConfirmRename),
                            )
                        )
                        .into()
                } else {
                    let request_button = button(
                        Row::new()
                            .spacing(8)
                            .padding(Padding::new(8.0).top(6.0).bottom(6.0).left(indent))
                            .push(text(req.method.as_str()).size(11))
                            .push(text(req_name).size(12))
                    )
                    .on_press(Message::SelectRequest(item_path.clone()))
                    .width(Length::Fill)
                    .style(if is_selected {
                        button::primary
                    } else {
                        button::text
                    });

                    mouse_area(request_button)
                        .on_right_press(Message::ShowContextMenu(item_path.clone(), 0.0, 0.0, ContextMenuTarget::Request))
                        .into()
                };

                column = column.push(request_content);
            }
            CollectionItem::Folder(folder) => {
                let expand_icon = if folder.expanded { "v" } else { ">" };
                let is_renaming = renaming_item.as_ref().map(|(path, _, _)| path) == Some(&item_path);

                let folder_content: Element<'a, Message> = if is_renaming {
                    let current_name = renaming_item.as_ref().map(|(_, _, name)| name.as_str()).unwrap_or(&folder.name);
                    Row::new()
                        .spacing(8)
                        .padding(Padding::new(8.0).top(6.0).bottom(6.0).left(indent))
                        .push(text(expand_icon).size(12))
                        .push(
                            underline_input::underline_input_sized(
                                rename_input_id.clone(),
                                translations.get("name_placeholder"),
                                current_name,
                                12,
                                |new_text| Message::UpdateRenamingText(new_text),
                                Some(Message::ConfirmRename),
                            )
                        )
                        .push(text(format!("({})", folder.items.len())).size(10))
                        .into()
                } else {
                    let folder_row = Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .push(
                            button(
                                Row::new()
                                    .spacing(8)
                                    .padding(Padding::new(8.0).top(6.0).bottom(6.0).left(indent))
                                    .push(text(expand_icon).size(12))
                                    .push(text(&folder.name).size(12))
                                    .push(text(format!("({})", folder.items.len())).size(10))
                            )
                            .on_press(Message::ToggleExpanded(item_path.clone()))
                            .width(Length::Fill)
                            .style(button::text)
                        )
                        .push(
                            button(text("+").size(14))
                                .padding([4, 8])
                                .style(button::text)
                                .on_press(Message::AddNewRequest(item_path.clone()))
                        );

                    mouse_area(folder_row)
                        .on_right_press(Message::ShowContextMenu(item_path.clone(), 0.0, 0.0, ContextMenuTarget::Folder))
                        .into()
                };

                column = column.push(folder_content);

                // Render folder contents if expanded
                if folder.expanded {
                    column = render_items(
                        &folder.items,
                        &item_path,
                        selected_request,
                        depth + 1,
                        column,
                        renaming_item,
                        rename_input_id,
                        translations,
                    );
                }
            }
        }
    }

    column
}

