use iced::widget::{button, container, text, text_editor, text_input, Column, Row};
use iced::{Alignment, Color, Element, Length};

use crate::app::Message;
use crate::i18n::Translations;
use crate::models::{BodyFormat, BodyType, Environment, Request, RequestTab};
use crate::ui::body_highlighter::BodyLanguage;
use crate::ui::components::{code_editor, /* environment_picker, */ key_value_editor, method_picker, option_buttons, tabs_bar};

pub fn view<'a>(
    request: &'a Request,
    active_tab: RequestTab,
    _current_env: Environment,
    body_content: &'a text_editor::Content,
    translations: &'a Translations,
) -> Element<'a, Message> {
    // Top bar: method, URL, environment dropdown, send button
    let method_selector = method_picker::view(request.method);

    let url_placeholder = translations.get("url_placeholder");
    let url_input = text_input(&url_placeholder, &request.url)
        .on_input(Message::UrlChanged)
        .padding(10)
        .size(13);

    // let env_selector = environment_picker::view(current_env);

    let send_text = translations.get("send");
    let send_button = button(text(send_text).size(14))
        .on_press(Message::SendRequest)
        .padding([10, 24])
        .style(button::primary);

    let top_bar = Row::new()
        .spacing(12)
        .padding([12, 16])
        .align_y(Alignment::Center)
        .push(method_selector)
        .push(url_input)
        // .push(env_selector)
        .push(send_button);

    // Tabs bar
    let tabs = tabs_bar::view(active_tab);

    // Tab content based on active tab
    let tab_content = match active_tab {
        RequestTab::Params => view_params_tab(request, translations),
        RequestTab::Body => view_body_tab(request, body_content, translations),
        RequestTab::Headers => view_headers_tab(request, translations),
        RequestTab::Cookies => view_cookies_tab(request, translations),
        RequestTab::Auth => view_auth_tab(request, translations),
    };

    let content = Column::new()
        .spacing(0)
        .height(Length::Fill)
        .push(top_bar)
        .push(tabs)
        .push(tab_content);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            ..Default::default()
        })
        .into()
}

fn view_params_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
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

fn view_body_tab<'a>(request: &'a Request, body_content: &'a text_editor::Content, translations: &'a Translations) -> Element<'a, Message> {
    // Body type selector using option_buttons component
    let body_formats = BodyFormat::all();
    let current_format = request.body.format();

    let format_selector = option_buttons::view(
        body_formats,
        current_format,
        |format| format.as_str().to_string(),
        Message::BodyFormatChanged,
    );

    // Body editor - only show if not None
    let mut content_column = Column::new()
        .spacing(0)
        .height(Length::Fill)
        .push(format_selector);

    // Only show editor when body format is not None
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

                content_column = content_column.push(form_editor);
            }
            _ => {
                // Determine language for syntax highlighting
                let language = match current_format {
                    BodyFormat::Json => BodyLanguage::Json,
                    BodyFormat::Xml => BodyLanguage::Xml,
                    BodyFormat::Text | BodyFormat::Binary => BodyLanguage::Plain,
                    _ => BodyLanguage::Plain,
                };

                // Use the reusable code editor component
                let editor = code_editor::view(body_content, language, Message::RequestBodyAction);

                content_column = content_column.push(editor);
            }
        }
    }

    container(content_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn view_headers_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
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

fn view_cookies_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
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
