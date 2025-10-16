use iced::widget::{button, column, container, row, scrollable, text, text_editor, Space};
use iced::{Border, Color, Element, Length};
use std::collections::HashMap;

use crate::app::Message;
use crate::models::{BodyViewMode, Response, ResponseTab};
use crate::ui::body_highlighter::BodyLanguage;
use crate::ui::components::code_editor;
use crate::ui::icons;

/// Detect the language for syntax highlighting based on Content-Type header
fn detect_language_from_headers(headers: &HashMap<String, String>) -> BodyLanguage {
    if let Some(content_type) = headers.get("content-type").or_else(|| headers.get("Content-Type")) {
        let content_type = content_type.to_lowercase();
        if content_type.contains("json") {
            return BodyLanguage::Json;
        } else if content_type.contains("xml") {
            return BodyLanguage::Xml;
        } else if content_type.contains("html") {
            return BodyLanguage::Html;
        }
    }
    BodyLanguage::Plain
}

pub fn view<'a>(
    response: &'a Option<Response>,
    active_tab: ResponseTab,
    active_body_view_mode: BodyViewMode,
    response_body_content: &'a text_editor::Content,
    loading: bool,
    error_message: &'a Option<String>,
) -> Element<'a, Message> {
    if loading {
        // Show loading state
        container(
            column![
                Space::with_height(40),
                container(
                    text("Loading...")
                        .size(16)
                        .color(Color::from_rgb(0.4, 0.4, 0.4))
                )
                .width(Length::Fill)
                .center_x(Length::Fill),
                Space::with_height(12),
                container(icons::loading_icon(32))
                    .width(Length::Fill)
                    .center_x(Length::Fill),
            ]
            .align_x(iced::Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else if let Some(ref resp) = response {
        // Status bar with status, time, and size
        let status_color = if resp.status >= 200 && resp.status < 300 {
            Color::from_rgb(0.0, 0.7, 0.0) // Green for success
        } else if resp.status >= 400 {
            Color::from_rgb(0.9, 0.0, 0.0) // Red for errors
        } else {
            Color::from_rgb(0.5, 0.5, 0.5) // Gray for others
        };

        // Header bar with status info and tab buttons combined
        let tabs = ResponseTab::all();
        let tab_buttons = tabs.iter().fold(row![].spacing(0), |row, tab| {
            let is_active = *tab == active_tab;
            let tab_style = move |_theme: &iced::Theme, status: button::Status| {
                let base_bg = if is_active {
                    Color::from_rgb(0.95, 0.95, 0.95)
                } else {
                    Color::from_rgb(0.98, 0.98, 0.98)
                };

                let bg = match status {
                    button::Status::Hovered if !is_active => Color::from_rgb(0.96, 0.96, 0.96),
                    _ => base_bg,
                };

                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    text_color: if is_active {
                        Color::from_rgb(0.0, 0.0, 0.0)
                    } else {
                        Color::from_rgb(0.4, 0.4, 0.4)
                    },
                    border: Border {
                        width: 0.0,
                        color: Color::TRANSPARENT,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            };

            let tab_button = button(
                container(text(tab.as_str()).size(14))
                    .padding([8, 16])
                    .center_x(Length::Shrink),
            )
            .on_press(Message::ResponseTabSelected(*tab))
            .style(tab_style)
            .padding(0);

            row.push(tab_button)
        });

        let header_bar = row![
            tab_buttons,
            Space::with_width(Length::Fill),
            text("Status:").size(14),
            Space::with_width(8),
            text(format!("{} {}", resp.status, resp.status_text))
                .size(14)
                .color(status_color),
            Space::with_width(20),
            text("Time:").size(14),
            Space::with_width(8),
            text(format!("{}ms", resp.time_ms)).size(14),
            Space::with_width(20),
            text("Size:").size(14),
            Space::with_width(8),
            text(format!("{} bytes", resp.size_bytes)).size(14),
        ]
        .spacing(0)
        .padding([8, 16]);

        // Tab content based on active tab
        let tab_content: Element<'a, Message> = match active_tab {
            ResponseTab::Body => {
                // Body view mode sub-tabs
                let body_modes = BodyViewMode::all();
                let body_mode_buttons = body_modes.iter().fold(row![].spacing(0), |row, mode| {
                    let is_active = *mode == active_body_view_mode;
                    let mode_style = move |_theme: &iced::Theme, status: button::Status| {
                        let base_bg = if is_active {
                            Color::from_rgb(0.93, 0.93, 0.93)
                        } else {
                            Color::from_rgb(0.98, 0.98, 0.98)
                        };

                        let bg = match status {
                            button::Status::Hovered if !is_active => Color::from_rgb(0.95, 0.95, 0.95),
                            _ => base_bg,
                        };

                        button::Style {
                            background: Some(iced::Background::Color(bg)),
                            text_color: if is_active {
                                Color::from_rgb(0.0, 0.0, 0.0)
                            } else {
                                Color::from_rgb(0.5, 0.5, 0.5)
                            },
                            border: Border {
                                width: 0.0,
                                color: Color::TRANSPARENT,
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }
                    };

                    let mode_button = button(
                        container(text(mode.as_str()).size(12))
                            .padding([6, 12])
                            .center_x(Length::Shrink),
                    )
                    .on_press(Message::BodyViewModeSelected(*mode))
                    .style(mode_style)
                    .padding(0);

                    row.push(mode_button)
                });

                let body_mode_bar: Element<'a, Message> = container(body_mode_buttons)
                    .padding([4, 16])
                    .width(Length::Fill)
                    .style(|_theme: &iced::Theme| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                        border: Border {
                            width: 1.0,
                            color: Color::from_rgb(0.9, 0.9, 0.9),
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })
                    .into();

                // Body content based on view mode
                let body_content: Element<'a, Message> = match active_body_view_mode {
                    BodyViewMode::Raw => {
                        // Raw view without syntax highlighting
                        code_editor::view(response_body_content, BodyLanguage::Plain, Message::ResponseBodyAction)
                    }
                    BodyViewMode::Json => {
                        // JSON syntax highlighting
                        code_editor::view(response_body_content, BodyLanguage::Json, Message::ResponseBodyAction)
                    }
                    BodyViewMode::Xml => {
                        // XML syntax highlighting
                        code_editor::view(response_body_content, BodyLanguage::Xml, Message::ResponseBodyAction)
                    }
                    BodyViewMode::Html => {
                        // HTML syntax highlighting
                        code_editor::view(response_body_content, BodyLanguage::Html, Message::ResponseBodyAction)
                    }
                };

                // Combine mode bar and body content
                column![body_mode_bar, body_content]
                    .spacing(0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            ResponseTab::Headers => {
                let headers_list = resp.headers.iter().fold(
                    column![].spacing(8).padding(16),
                    |col, (key, value)| {
                        col.push(
                            row![
                                container(text(key).size(12))
                                    .width(Length::Fixed(200.0))
                                    .padding([4, 8]),
                                text(value).size(12).color(Color::from_rgb(0.4, 0.4, 0.4)),
                            ]
                            .spacing(8),
                        )
                    },
                );
                container(scrollable(headers_list))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            ResponseTab::Cookies => {
                if resp.cookies.is_empty() {
                    container(
                        text("No cookies in response")
                            .size(14)
                            .color(Color::from_rgb(0.5, 0.5, 0.5)),
                    )
                    .padding(16)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                } else {
                    let cookies_list = resp.cookies.iter().fold(
                        column![].spacing(8).padding(16),
                        |col, cookie| {
                            col.push(
                                row![
                                    container(text(&cookie.key).size(12))
                                        .width(Length::Fixed(200.0))
                                        .padding([4, 8]),
                                    text(&cookie.value).size(12).color(Color::from_rgb(0.4, 0.4, 0.4)),
                                ]
                                .spacing(8),
                            )
                        },
                    );
                    container(scrollable(cookies_list))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                }
            }
        };

        container(
            column![header_bar, tab_content]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else if let Some(error) = error_message {
        // Show error message
        container(
            column![
                container(
                    text("Request Failed")
                        .size(16)
                        .color(Color::from_rgb(0.9, 0.0, 0.0))
                )
                .width(Length::Fill)
                .center_x(Length::Fill),
                Space::with_height(8),
                container(
                    text(error)
                        .size(14)
                        .color(Color::from_rgb(0.5, 0.5, 0.5))
                )
                .padding(16)
                .width(Length::Fill)
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.98, 0.95, 0.95))),
                    border: Border {
                        width: 1.0,
                        color: Color::from_rgb(0.9, 0.7, 0.7),
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
            ]
            .align_x(iced::Alignment::Center)
            .padding(40)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
    } else {
        container(
            container(text("No response yet").size(14).color(Color::from_rgb(0.5, 0.5, 0.5)))
                .padding(40)
                .width(Length::Fill)
                .center_x(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}
