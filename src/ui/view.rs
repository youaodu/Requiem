use iced::widget::{column, container, horizontal_rule, mouse_area, row, stack, text, vertical_rule};
use iced::{Alignment, Element, Length};

use crate::app::{Message, Requiem};
use crate::i18n::I18n;

use super::components::{context_menu, environment_dialog, settings_dialog};
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
        );
        let response_panel = response_viewer::view(
            &state.response,
            state.active_response_tab,
            state.active_body_view_mode,
            &state.response_body_content,
            state.loading,
        );

        // Column layout: tab bar on top, editor in middle, response at bottom
        column![
            tab_bar_row,
            container(request_editor_panel)
                .width(Length::Fill)
                .height(Length::FillPortion(1)),
            horizontal_rule(1),
            container(response_panel)
                .width(Length::Fill)
                .height(Length::FillPortion(1))
        ]
        .spacing(0)
    } else {
        // Show empty state if no request is selected
        column![
            tab_bar_row,
            container(
                text(state.t("empty_state"))
                    .size(16)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
        ]
        .spacing(0)
    };

    // Three-column layout: sidebar | editor | response
    let content_row = row![
        container(request_list_panel)
            .width(Length::Fixed(280.0))
            .height(Length::Fill),
        vertical_rule(1),
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
        let menu_overlay = context_menu::view(&ctx_menu.path, ctx_menu.x, ctx_menu.y, &ctx_menu.target, &state.translations);
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
                    background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
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
        let backdrop = mouse_area(
            container(text(""))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                    ..Default::default()
                }),
        )
        .on_press(Message::CloseSettingsDialog);

        layers.push(backdrop.into());

        // Dialog centered on screen
        let dialog = container(settings_dialog::view(state.language(), &state.save_directory, &state.translations))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        layers.push(dialog.into());
    }

    // Use stack to layer all overlays
    stack(layers).into()
}

