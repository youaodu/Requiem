use iced::widget::{button, column, container, row, scrollable, text, text_editor, Space};
use iced::{Border, Color, Element, Length};
use std::collections::HashMap;

use crate::app::Message;
use crate::models::{Response, ResponseTab};
use crate::ui::body_highlighter::BodyLanguage;
use crate::ui::components::code_editor;

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
    response_body_content: &'a text_editor::Content,
) -> Element<'a, Message> {
    if let Some(ref resp) = response {
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
        let tab_content = match active_tab {
            ResponseTab::Body => {
                // Determine language based on Content-Type header
                let language = detect_language_from_headers(&resp.headers);

                // Use the code editor component with syntax highlighting
                code_editor::view(response_body_content, language, Message::ResponseBodyAction)
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
                // For now, show a placeholder for cookies
                container(
                    text("Cookies parsing not yet implemented")
                        .size(14)
                        .color(Color::from_rgb(0.5, 0.5, 0.5)),
                )
                .padding(16)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
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
