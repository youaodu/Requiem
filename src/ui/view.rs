use iced::widget::{column, container, mouse_area, row, stack, text};
use iced::{mouse, Alignment, Element, Length};

use crate::app::{Message, Requiem};
use crate::i18n::I18n;

use super::components::{ai_fill_dialog, context_menu, environment_dialog, settings_dialog};
use super::{request_editor, request_list, request_tabs, response_viewer, toast};

pub fn view(state: &Requiem) -> Element<'_, Message> {
    let request_list_panel = request_list::view(
        &state.collections,
        state.selected_request.as_ref(),
        None, // Don't pass context menu to request_list anymore
        state.renaming_item.as_ref(),
        &state.search_query,
        &state.rename_input_id,
        &state.translations,
    );

    // Build tab bar
    let tab_bar_row = container(request_tabs::view(
        &state.open_tabs,
        state.active_tab_index,
        &state.drag_state,
    ))
    .align_y(Alignment::Center);

    let main_content = if let Some(request) = state.get_current_request() {
        let request_editor_panel = request_editor::view(
            request,
            state.active_tab,
            state.current_environment,
            &state.request_body_content,
            &state.translations,
            state.request_body_word_wrap,
        );
        let response_panel = response_viewer::view(
            &state.response,
            state.active_response_tab,
            state.active_body_view_mode,
            &state.response_body_content,
            state.loading,
            &state.error_message,
            &state.translations,
        );

        // Calculate portions for vertical split based on ratio
        let top_portion = (state.vertical_split_ratio * 100.0) as u16;
        let bottom_portion = ((1.0 - state.vertical_split_ratio) * 100.0) as u16;

        // Create vertical splitter (draggable divider)
        // Visual line is 2px but interaction area is 6px for easier clicking
        let vertical_splitter = mouse_area(
            container("")
                .width(Length::Fill)
                .height(2)
                .padding([2, 0]) // 2px padding on top/bottom, makes visual line 2px
                .style(move |_theme: &iced::Theme| {
                    let color = if state.dragging_vertical_splitter {
                        iced::Color::from_rgb(0.5, 0.5, 0.5)
                    } else {
                        iced::Color::from_rgb(0.8, 0.8, 0.8)
                    };
                    iced::widget::container::Style {
                        background: Some(iced::Background::Color(color)),
                        ..Default::default()
                    }
                })
        )
        .on_press(Message::VerticalSplitterPressed)
        .interaction(mouse::Interaction::ResizingVertically);

        // Column layout: tab bar on top, editor in middle, splitter, response at bottom
        column![
            tab_bar_row,
            container(request_editor_panel)
                .width(Length::Fill)
                .height(Length::FillPortion(top_portion)),
            vertical_splitter,
            container(response_panel)
                .width(Length::Fill)
                .height(Length::FillPortion(bottom_portion))
        ]
        .spacing(0)
    } else {
        // Show empty state if no request is selected
        column![
            tab_bar_row,
            container(text(state.t("empty_state")).size(16))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        ]
        .spacing(0)
    };

    // Create sidebar splitter (draggable divider)
    // Visual line is 2px but interaction area is 6px for easier clicking
    let sidebar_splitter = mouse_area(
        container("")
            .width(2)
            .height(Length::Fill)
            .padding([0, 2]) // 2px padding on left/right, makes visual line 2px
            .style(move |_theme: &iced::Theme| {
                let color = if state.dragging_sidebar_splitter {
                    iced::Color::from_rgb(0.5, 0.5, 0.5)
                } else {
                    iced::Color::from_rgb(0.8, 0.8, 0.8)
                };
                iced::widget::container::Style {
                    background: Some(iced::Background::Color(color)),
                    ..Default::default()
                }
            })
    )
    .on_press(Message::SidebarSplitterPressed)
    .interaction(mouse::Interaction::ResizingHorizontally);

    // Three-column layout: sidebar | splitter | main content
    let content_row = row![
        container(request_list_panel)
            .width(Length::Fixed(state.sidebar_width))
            .height(Length::Fill),
        sidebar_splitter,
        main_content
    ]
    .spacing(0);

    let base_layout = container(content_row)
        .width(Length::Fill)
        .height(Length::Fill);

    // Wrap base layout in mouse_area to detect clicks and mouse movement
    let base_with_mouse_detection = if state.renaming_item.is_some() {
        // When renaming, wrap content to detect clicks outside
        mouse_area(base_layout)
            .on_press(Message::HideContextMenu) // This will clear renaming_item
            .on_move(|point| Message::MouseMoved(point.x, point.y))
            .into()
    } else {
        // Always track mouse movement for tab dragging
        mouse_area(base_layout)
            .on_move(|point| Message::MouseMoved(point.x, point.y))
            .on_release(Message::TabDragEnd) // Global release handler
            .into()
    };

    let mut layers: Vec<Element<'_, Message>> = vec![base_with_mouse_detection];

    // Display toast as a floating overlay if present
    if let Some(toast_data) = &state.toast {
        let toast_view = toast::view(toast_data);

        // Create a floating toast using stack
        let toast_container = container(toast_view)
            .padding([20, 0])
            .width(Length::Fill)
            .center_x(Length::Fill);

        layers.push(toast_container.into());
    }

    // Display context menu as a floating overlay if present
    if let Some(ctx_menu) = &state.context_menu {
        let menu_overlay = context_menu::view(
            &ctx_menu.path,
            ctx_menu.x,
            ctx_menu.y,
            &ctx_menu.target,
            &state.translations,
        );
        layers.push(menu_overlay);
    }

    // Display environment dialog as a modal overlay if present
    if state.show_environment_dialog {
        // Semi-transparent backdrop
        let backdrop = mouse_area(
            container(text(""))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(
                        0.0, 0.0, 0.0, 0.5,
                    ))),
                    ..Default::default()
                }),
        )
        .on_press(Message::CloseEnvironmentDialog);

        layers.push(backdrop.into());

        // Dialog centered on screen
        let dialog = container(environment_dialog::view())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        layers.push(dialog.into());
    }

    // Display settings dialog as a modal overlay if present
    if state.show_settings_dialog {
        // Semi-transparent backdrop
        let backdrop = container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.5,
                ))),
                ..Default::default()
            });

        layers.push(backdrop.into());

        // Dialog centered on screen
        let dialog = container(settings_dialog::view(
            state.language(),
            &state.save_directory,
            &state.ai_config,
            &state.translations,
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        layers.push(dialog.into());
    }

    // Display AI Fill dialog as a modal overlay if present
    if state.show_ai_fill_dialog {
        // Semi-transparent backdrop
        let backdrop = container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.5,
                ))),
                ..Default::default()
            });

        layers.push(backdrop.into());

        // Dialog centered on screen
        let dialog = container(ai_fill_dialog::view(
            &state.ai_fill_input_content,
            &state.translations,
            state.ai_fill_loading,
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

        layers.push(dialog.into());
    }

    // Use stack to layer all overlays
    stack(layers).into()
}
