use iced::widget::{button, container, text, text_editor, text_input, Column, Row};
use iced::{Alignment, Color, Element, Length};

use crate::app::Message;
use crate::i18n::Translations;
use crate::models::{BodyFormat, BodyType, Environment, Request, RequestTab};
use crate::ui::body_highlighter::BodyLanguage;
use crate::ui::components::{
    code_editor, /* environment_picker, */ key_value_editor, method_picker, option_buttons,
    tabs_bar,
};

pub fn view<'a>(
    request: &'a Request,
    active_tab: RequestTab,
    _current_env: Environment,
    body_content: &'a text_editor::Content,
    translations: &'a Translations,
    request_body_word_wrap: bool,
) -> Element<'a, Message> {
    // Top bar: method, URL, environment dropdown, send button
    let method_selector = method_picker::view(request.method);

    let url_placeholder = translations.get("url_placeholder");
    let url_input = text_input(url_placeholder, &request.url)
        .on_input(Message::UrlChanged)
        .padding(10)
        .size(13);

    // let env_selector = environment_picker::view(current_env);

    let ai_fill_text = translations.get("ai_fill");
    let ai_fill_button = button(text(ai_fill_text).size(14))
        .on_press(Message::AiFill)
        .padding([10, 20])
        .style(button::secondary);

    let send_text = translations.get("send");
    let send_button = button(text(send_text).size(14))
        .on_press(Message::SendRequest)
        .padding([10, 24])
        .style(button::primary);

    let top_bar = container(
        Row::new()
            .spacing(12)
            .padding([12, 16])
            .align_y(Alignment::Center)
            .push(method_selector)
            .push(url_input)
            // .push(env_selector)
            .push(ai_fill_button)
            .push(send_button),
    )
    .height(Length::Shrink);

    // Tabs bar
    let tabs = container(tabs_bar::view(active_tab))
        .height(Length::Shrink);

    // Tab content based on active tab
    let tab_content = match active_tab {
        RequestTab::Params => view_params_tab(request, translations),
        RequestTab::Body => view_body_tab(request, body_content, translations, request_body_word_wrap),
        RequestTab::Headers => view_headers_tab(request, translations),
        RequestTab::Cookies => view_cookies_tab(request, translations),
        RequestTab::Auth => view_auth_tab(request, translations),
    };

    // Wrap tab content in container with Fill height to ensure it takes all remaining space
    let tab_content_container = container(tab_content)
        .width(Length::Fill)
        .height(Length::Fill);

    let content = Column::new()
        .spacing(0)
        .height(Length::Fill)
        .push(top_bar)
        .push(tabs)
        .push(tab_content_container);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            ..Default::default()
        })
        .into()
}

fn view_params_tab<'a>(
    request: &'a Request,
    translations: &'a Translations,
) -> Element<'a, Message> {
    let config = key_value_editor::KeyValueEditorConfig {
        key_label: translations.get("param_key_label"),
        value_label: translations.get("param_value_label"),
        key_placeholder: translations.get("param_key_placeholder"),
        value_placeholder: translations.get("param_value_placeholder"),
        add_button_text: translations.get("add_param"),
    };

    key_value_editor::view(
        &request.query_params,
        config,
        |param| &param.key,
        |param| &param.value,
        Message::ParamKeyChanged,
        Message::ParamValueChanged,
        Message::RemoveParam,
        Message::AddParam,
    )
}

fn view_body_tab<'a>(
    request: &'a Request,
    body_content: &'a text_editor::Content,
    translations: &'a Translations,
    word_wrap_enabled: bool,
) -> Element<'a, Message> {
    // Body type selector using option_buttons component
    let body_formats = BodyFormat::all();
    let current_format = request.body.format();

    let format_selector = container(option_buttons::view(
        body_formats,
        current_format,
        |format| format.as_str().to_string(),
        Message::BodyFormatChanged,
    ))
    .height(Length::Shrink);

    // Create toolbar with word wrap and format JSON buttons
    let toolbar = container(create_body_toolbar(current_format, word_wrap_enabled, translations))
        .height(Length::Shrink);

    // Body editor - only show if not None
    let mut content_column = Column::new()
        .spacing(0)
        .height(Length::Fill)
        .push(format_selector);

    // Only show toolbar and editor when body format is not None
    if current_format != BodyFormat::None {
        match current_format {
            BodyFormat::FormData | BodyFormat::FormUrlEncoded => {
                // Show key-value table for form data
                let config = key_value_editor::KeyValueEditorConfig {
                    key_label: translations.get("param_key_label"),
                    value_label: translations.get("param_value_label"),
                    key_placeholder: translations.get("form_key_placeholder"),
                    value_placeholder: translations.get("form_value_placeholder"),
                    add_button_text: translations.get("add_form_field"),
                };

                let form_editor = match &request.body {
                    BodyType::FormData(fields) | BodyType::FormUrlEncoded(fields) => {
                        key_value_editor::view(
                            fields,
                            config,
                            |field| &field.key,
                            |field| &field.value,
                            Message::FormDataKeyChanged,
                            Message::FormDataValueChanged,
                            Message::RemoveFormDataField,
                            Message::AddFormDataField,
                        )
                    }
                    _ => container(text("")).into(),
                };

                // Wrap form editor in container with Fill height
                let form_container = container(form_editor)
                    .width(Length::Fill)
                    .height(Length::Fill);

                content_column = content_column.push(form_container);
            }
            _ => {
                // Add toolbar for text-based formats
                content_column = content_column.push(toolbar);

                // Determine language for syntax highlighting
                let language = match current_format {
                    BodyFormat::Json => BodyLanguage::Json,
                    BodyFormat::Xml => BodyLanguage::Xml,
                    BodyFormat::Text | BodyFormat::Binary => BodyLanguage::Plain,
                    _ => BodyLanguage::Plain,
                };

                // Use the reusable code editor component with word wrap support
                // Note: code_editor::view already returns an Element with Fill height
                let editor = code_editor::view(
                    body_content,
                    language,
                    Message::RequestBodyAction,
                    word_wrap_enabled,
                );

                content_column = content_column.push(editor);
            }
        }
    }

    container(content_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Create toolbar with word wrap and format JSON buttons for body editor
fn create_body_toolbar<'a>(
    format: BodyFormat,
    word_wrap_enabled: bool,
    translations: &'a Translations,
) -> Element<'a, Message> {
    let word_wrap_text = translations.get("word_wrap");
    let word_wrap_button = button(text(word_wrap_text).size(11))
        .on_press(Message::ToggleRequestBodyWordWrap)
        .padding([4, 10])
        .style(move |theme, status| {
            let base = if word_wrap_enabled {
                button::primary(theme, status)
            } else {
                button::secondary(theme, status)
            };

            // Make button more subtle
            iced::widget::button::Style {
                background: base.background.map(|bg| {
                    if let iced::Background::Color(_) = bg {
                        if word_wrap_enabled {
                            iced::Background::Color(Color::from_rgb(0.3, 0.5, 0.9))
                        } else {
                            iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95))
                        }
                    } else {
                        bg
                    }
                }),
                text_color: if word_wrap_enabled {
                    Color::WHITE
                } else {
                    Color::from_rgb(0.4, 0.4, 0.4)
                },
                ..base
            }
        });

    let mut toolbar_row = Row::new()
        .spacing(6)
        .padding(iced::Padding {
            top: 6.0,
            right: 0.0,
            bottom: 6.0,
            left: 10.0, // Match editor's internal padding to align with editor content
        })
        .align_y(Alignment::Center)
        .push(word_wrap_button);

    // Only show format JSON button for JSON body type
    if format == BodyFormat::Json {
        let format_json_text = translations.get("format_json");
        let format_json_button = button(text(format_json_text).size(11))
            .on_press(Message::FormatRequestBodyJson)
            .padding([4, 10])
            .style(|theme, status| {
                let base = button::secondary(theme, status);
                iced::widget::button::Style {
                    background: base.background.map(|bg| {
                        if let iced::Background::Color(_) = bg {
                            iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95))
                        } else {
                            bg
                        }
                    }),
                    text_color: Color::from_rgb(0.4, 0.4, 0.4),
                    ..base
                }
            });

        toolbar_row = toolbar_row.push(format_json_button);
    }

    container(toolbar_row)
        .width(Length::Fill)
        .padding(iced::Padding {
            top: 0.0,
            right: 16.0,
            bottom: 0.0,
            left: 16.0,
        })
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border {
                color: Color::from_rgb(0.92, 0.92, 0.92),
                width: 0.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        })
        .into()
}

fn view_headers_tab<'a>(
    request: &'a Request,
    translations: &'a Translations,
) -> Element<'a, Message> {
    let config = key_value_editor::KeyValueEditorConfig {
        key_label: translations.get("header_key_label"),
        value_label: translations.get("header_value_label"),
        key_placeholder: translations.get("header_key_placeholder"),
        value_placeholder: translations.get("header_value_placeholder"),
        add_button_text: translations.get("add_header"),
    };

    key_value_editor::view(
        &request.headers,
        config,
        |header| &header.key,
        |header| &header.value,
        Message::HeaderKeyChanged,
        Message::HeaderValueChanged,
        Message::RemoveHeader,
        Message::AddHeader,
    )
}

fn view_cookies_tab<'a>(
    request: &'a Request,
    translations: &'a Translations,
) -> Element<'a, Message> {
    let config = key_value_editor::KeyValueEditorConfig {
        key_label: translations.get("cookie_key_label"),
        value_label: translations.get("cookie_value_label"),
        key_placeholder: translations.get("cookie_key_placeholder"),
        value_placeholder: translations.get("cookie_value_placeholder"),
        add_button_text: translations.get("add_cookie"),
    };

    key_value_editor::view(
        &request.cookies,
        config,
        |cookie| &cookie.key,
        |cookie| &cookie.value,
        Message::CookieKeyChanged,
        Message::CookieValueChanged,
        Message::RemoveCookie,
        Message::AddCookie,
    )
}

fn view_auth_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
    let config = key_value_editor::KeyValueEditorConfig {
        key_label: translations.get("auth_key_label"),
        value_label: translations.get("auth_value_label"),
        key_placeholder: translations.get("auth_key_placeholder"),
        value_placeholder: translations.get("auth_value_placeholder"),
        add_button_text: translations.get("add_auth_field"),
    };

    key_value_editor::view(
        &request.auth,
        config,
        |auth| &auth.key,
        |auth| &auth.value,
        Message::AuthKeyChanged,
        Message::AuthValueChanged,
        Message::RemoveAuthField,
        Message::AddAuthField,
    )
}
