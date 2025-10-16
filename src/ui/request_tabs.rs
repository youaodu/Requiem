use iced::widget::{button, container, mouse_area, row, scrollable, text, Row, Space};
use iced::{Color, Element, Length};

use crate::app::state::{DragState, RequestTabItem};
use crate::app::Message;

pub fn view<'a>(
    tabs: &'a [RequestTabItem],
    active_index: Option<usize>,
    drag_state: &'a Option<DragState>,
) -> Element<'a, Message> {
    let mut tab_row: Row<'_, Message> = row![].spacing(0);

    // Create a rendering order based on drag state
    let render_order: Vec<usize> = if let Some(drag) = drag_state {
        // Calculate visual order during drag
        let mut order: Vec<usize> = (0..tabs.len()).collect();
        if let Some(hover_idx) = drag.hover_index {
            let dragging_idx = drag.dragging_tab_index;
            if dragging_idx != hover_idx && dragging_idx < tabs.len() && hover_idx < tabs.len() {
                // Move dragging item to hover position visually
                order.remove(dragging_idx);
                order.insert(hover_idx, dragging_idx);
            }
        }
        order
    } else {
        (0..tabs.len()).collect()
    };

    for &idx in &render_order {
        let tab = &tabs[idx];
        let is_active = Some(idx) == active_index;
        let is_dragging = drag_state
            .as_ref()
            .map(|d| d.dragging_tab_index == idx)
            .unwrap_or(false);
        let is_hover_target = drag_state
            .as_ref()
            .and_then(|d| d.hover_index)
            .map(|h| h == idx)
            .unwrap_or(false);

        // Tab content: name + close button
        let tab_name = text(&tab.name)
            .size(13)
            .color(if is_active {
                Color::from_rgb(0.95, 0.95, 0.95)
            } else {
                Color::from_rgb(0.7, 0.7, 0.7)
            });

        let close_button = button(text("×").size(15))
            .on_press(Message::CloseTab(idx))
            .padding([0, 4])
            .style(|_theme, status| button::Style {
                background: None,
                text_color: match status {
                    button::Status::Hovered => Color::from_rgb(1.0, 0.3, 0.3),
                    _ => Color::from_rgb(0.65, 0.65, 0.65),
                },
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
                snap: false,
            });

        let tab_content = row![tab_name, Space::new().width(6), close_button]
            .spacing(2)
            .padding([4, 8]);

        let tab_button = button(tab_content)
            .style(move |_theme, status| {
                let bg_color = if is_dragging {
                    Color::from_rgba(0.35, 0.35, 0.38, 0.7) // Semi-transparent when dragging
                } else if is_hover_target {
                    Color::from_rgb(0.4, 0.6, 0.8) // Highlight drop target
                } else if is_active {
                    Color::from_rgb(0.35, 0.35, 0.38)
                } else {
                    match status {
                        button::Status::Hovered => Color::from_rgb(0.28, 0.28, 0.30),
                        _ => Color::from_rgb(0.22, 0.22, 0.24),
                    }
                };

                button::Style {
                    background: Some(iced::Background::Color(bg_color)),
                    text_color: Color::WHITE,
                    border: iced::Border {
                        color: if is_hover_target {
                            Color::from_rgb(0.5, 0.7, 0.9)
                        } else if is_active {
                            Color::from_rgb(0.45, 0.45, 0.48)
                        } else {
                            Color::from_rgb(0.30, 0.30, 0.32)
                        },
                        width: if is_hover_target { 2.0 } else { 1.0 },
                        radius: iced::border::Radius::from(4.0),
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            });

        let tab_with_mouse = mouse_area(tab_button)
            .on_press(Message::TabPressStart(idx, 0.0))
            .on_release(Message::TabDragEnd); // Changed to TabDragEnd for consistency

        tab_row = tab_row.push(tab_with_mouse);
    }

    // Wrap tab_row in a scrollable for horizontal scrolling
    let scrollable_tabs = scrollable(tab_row)
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::new()
                .width(6)
                .scroller_width(6)
        ))
        .width(Length::Fill)
        .style(|_theme, status| {
            let scrollbar_color = match status {
                scrollable::Status::Hovered { .. } => Color::from_rgb(0.6, 0.6, 0.6),
                scrollable::Status::Dragged { .. } => Color::from_rgb(0.5, 0.5, 0.5),
                _ => Color::from_rgb(0.7, 0.7, 0.7),
            };

            scrollable::Style {
                container: container::Style {
                    background: None,
                    ..Default::default()
                },
                vertical_rail: scrollable::Rail {
                    background: None,
                    border: iced::Border::default(),
                    scroller: scrollable::Scroller {
                        color: scrollbar_color,
                        border: iced::Border {
                            radius: iced::border::Radius::from(3.0),
                            ..Default::default()
                        },
                    },
                },
                horizontal_rail: scrollable::Rail {
                    background: Some(iced::Background::Color(Color::from_rgba(0.85, 0.85, 0.85, 0.3))),
                    border: iced::Border {
                        radius: iced::border::Radius::from(3.0),
                        ..Default::default()
                    },
                    scroller: scrollable::Scroller {
                        color: scrollbar_color,
                        border: iced::Border {
                            radius: iced::border::Radius::from(3.0),
                            ..Default::default()
                        },
                    },
                },
                gap: None,
            }
        });

    container(scrollable_tabs)
        .padding([3, 6])
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            border: iced::Border {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                width: 1.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        })
        .into()
}
