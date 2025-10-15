use iced::widget::{button, container, text, text_editor, text_input, Column, Row};
use iced::{Alignment, Color, Element, Length};

use crate::app::Message;
use crate::i18n::Translations;
use crate::models::{BodyFormat, BodyType, Environment, Request, RequestTab};
use crate::ui::body_highlighter::BodyLanguage;
use crate::ui::components::{code_editor, environment_picker, method_picker, tabs_bar};

pub fn view<'a>(
    request: &'a Request,
    active_tab: RequestTab,
    current_env: Environment,
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

    let env_selector = environment_picker::view(current_env);

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
        .push(env_selector)
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
    // Pre-allocate all translated strings
    let key_label = translations.get("param_key_label");
    let value_label = translations.get("param_value_label");
    let key_placeholder = translations.get("param_key_placeholder");
    let value_placeholder = translations.get("param_value_placeholder");
    let add_param_text = translations.get("add_param");

    let mut params_column = Column::new()
        .spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(
            container(text(key_label).size(12))
                .width(Length::FillPortion(1))
        )
        .push(
            container(text(value_label).size(12))
                .width(Length::FillPortion(2))
        )
        .push(
            container(text("").size(12))
                .width(Length::Fixed(40.0))
        );

    params_column = params_column.push(header_labels);

    for (idx, param) in request.query_params.iter().enumerate() {
        let key_input = text_input(&key_placeholder, &param.key)
            .on_input(move |v| Message::ParamKeyChanged(idx, v))
            .padding(10)
            .size(13);

        let value_input = text_input(&value_placeholder, &param.value)
            .on_input(move |v| Message::ParamValueChanged(idx, v))
            .padding(10)
            .size(13);

        let remove_button = button(text("×").size(16))
            .on_press(Message::RemoveParam(idx))
            .padding([8, 12])
            .style(button::text);

        let param_row = Row::new()
            .spacing(16)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(1)))
            .push(container(value_input).width(Length::FillPortion(2)))
            .push(remove_button);

        params_column = params_column.push(param_row);
    }

    let add_param_button = button(text(add_param_text).size(13))
        .on_press(Message::AddParam)
        .padding([10, 16])
        .style(button::secondary);

    params_column = params_column.push(add_param_button);

    container(params_column)
        .padding(20)
        .into()
}

fn view_body_tab<'a>(request: &'a Request, body_content: &'a text_editor::Content, translations: &'a Translations) -> Element<'a, Message> {
    // Pre-allocate all translated strings for FormData
    let form_key_label = translations.get("param_key_label");
    let form_value_label = translations.get("param_value_label");
    let form_key_ph = translations.get("form_key_placeholder");
    let form_value_ph = translations.get("form_value_placeholder");
    let add_field_text = translations.get("add_form_field");

    // Body type selector
    let body_formats = BodyFormat::all();
    let current_format = request.body.format();

    let mut format_selector = Row::new().spacing(4).padding([12, 16]);

    for format in body_formats {
        let is_selected = format == current_format;
        let btn = button(text(format.as_str()).size(12))
            .on_press(Message::BodyFormatChanged(format))
            .padding([8, 14])
            .style(if is_selected {
                button::primary
            } else {
                button::text
            });
        format_selector = format_selector.push(btn);
    }

    // Add flexible space to push format buttons to the left
    format_selector = format_selector.push(container(text("")).width(Length::Fill));

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
                let form_fields = match &request.body {
                    BodyType::FormData(fields) | BodyType::FormUrlEncoded(fields) => fields.clone(),
                    _ => vec![],
                };

                let mut form_column = Column::new()
                    .spacing(10);

                // Header row with column labels
                let header_labels = Row::new()
                    .spacing(16)
                    .padding([8, 0])
                    .push(
                        container(text(form_key_label).size(12))
                            .width(Length::FillPortion(1))
                    )
                    .push(
                        container(text(form_value_label).size(12))
                            .width(Length::FillPortion(2))
                    )
                    .push(
                        container(text("").size(12))
                            .width(Length::Fixed(40.0))
                    );

                form_column = form_column.push(header_labels);

                for (idx, field) in form_fields.iter().enumerate() {
                    let key_input = text_input(&form_key_ph, &field.key)
                        .on_input(move |v| Message::FormDataKeyChanged(idx, v))
                        .padding(10)
                        .size(13);

                    let value_input = text_input(&form_value_ph, &field.value)
                        .on_input(move |v| Message::FormDataValueChanged(idx, v))
                        .padding(10)
                        .size(13);

                    let remove_button = button(text("×").size(16))
                        .on_press(Message::RemoveFormDataField(idx))
                        .padding([8, 12])
                        .style(button::text);

                    let field_row = Row::new()
                        .spacing(16)
                        .align_y(Alignment::Center)
                        .push(container(key_input).width(Length::FillPortion(1)))
                        .push(container(value_input).width(Length::FillPortion(2)))
                        .push(remove_button);

                    form_column = form_column.push(field_row);
                }

                let add_field_button = button(text(add_field_text).size(13))
                    .on_press(Message::AddFormDataField)
                    .padding([10, 16])
                    .style(button::secondary);

                form_column = form_column.push(add_field_button);

                content_column = content_column.push(
                    container(form_column)
                        .padding(20)
                );
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
    // Pre-allocate all translated strings
    let key_label = translations.get("header_key_label");
    let value_label = translations.get("header_value_label");
    let key_ph = translations.get("header_key_placeholder");
    let value_ph = translations.get("header_value_placeholder");
    let add_text = translations.get("add_header");
    let mut headers_column = Column::new()
        .spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(
            container(text(key_label).size(12))
                .width(Length::FillPortion(1))
        )
        .push(
            container(text(value_label).size(12))
                .width(Length::FillPortion(2))
        )
        .push(
            container(text("").size(12))
                .width(Length::Fixed(40.0))
        );

    headers_column = headers_column.push(header_labels);

    for (idx, header) in request.headers.iter().enumerate() {
        let key_input = text_input(&key_ph, &header.key)
            .on_input(move |v| Message::HeaderKeyChanged(idx, v))
            .padding(10)
            .size(13);

        let value_input = text_input(&value_ph, &header.value)
            .on_input(move |v| Message::HeaderValueChanged(idx, v))
            .padding(10)
            .size(13);

        let remove_button = button(text("×").size(16))
            .on_press(Message::RemoveHeader(idx))
            .padding([8, 12])
            .style(button::text);

        let header_row = Row::new()
            .spacing(16)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(1)))
            .push(container(value_input).width(Length::FillPortion(2)))
            .push(remove_button);

        headers_column = headers_column.push(header_row);
    }

    let add_header_button = button(text(add_text).size(13))
        .on_press(Message::AddHeader)
        .padding([10, 16])
        .style(button::secondary);

    headers_column = headers_column.push(add_header_button);

    container(headers_column)
        .padding(20)
        .into()
}

fn view_cookies_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
    // Pre-allocate all translated strings
    let key_label = translations.get("cookie_key_label");
    let value_label = translations.get("cookie_value_label");
    let key_ph = translations.get("cookie_key_placeholder");
    let value_ph = translations.get("cookie_value_placeholder");
    let add_text = translations.get("add_cookie");
    let mut cookies_column = Column::new()
        .spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(
            container(text(key_label).size(12))
                .width(Length::FillPortion(1))
        )
        .push(
            container(text(value_label).size(12))
                .width(Length::FillPortion(2))
        )
        .push(
            container(text("").size(12))
                .width(Length::Fixed(40.0))
        );

    cookies_column = cookies_column.push(header_labels);

    for (idx, cookie) in request.cookies.iter().enumerate() {
        let key_input = text_input(&key_ph, &cookie.key)
            .on_input(move |v| Message::CookieKeyChanged(idx, v))
            .padding(10)
            .size(13);

        let value_input = text_input(&value_ph, &cookie.value)
            .on_input(move |v| Message::CookieValueChanged(idx, v))
            .padding(10)
            .size(13);

        let remove_button = button(text("×").size(16))
            .on_press(Message::RemoveCookie(idx))
            .padding([8, 12])
            .style(button::text);

        let cookie_row = Row::new()
            .spacing(16)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(1)))
            .push(container(value_input).width(Length::FillPortion(2)))
            .push(remove_button);

        cookies_column = cookies_column.push(cookie_row);
    }

    let add_cookie_button = button(text(add_text).size(13))
        .on_press(Message::AddCookie)
        .padding([10, 16])
        .style(button::secondary);

    cookies_column = cookies_column.push(add_cookie_button);

    container(cookies_column)
        .padding(20)
        .into()
}

fn view_auth_tab<'a>(request: &'a Request, translations: &'a Translations) -> Element<'a, Message> {
    // Pre-allocate all translated strings
    let key_label = translations.get("auth_key_label");
    let value_label = translations.get("auth_value_label");
    let key_ph = translations.get("auth_key_placeholder");
    let value_ph = translations.get("auth_value_placeholder");
    let add_text = translations.get("add_auth_field");
    let mut auth_column = Column::new()
        .spacing(10);

    // Header row with column labels
    let header_labels = Row::new()
        .spacing(16)
        .padding([8, 0])
        .push(
            container(text(key_label).size(12))
                .width(Length::FillPortion(1))
        )
        .push(
            container(text(value_label).size(12))
                .width(Length::FillPortion(2))
        )
        .push(
            container(text("").size(12))
                .width(Length::Fixed(40.0))
        );

    auth_column = auth_column.push(header_labels);

    for (idx, auth_field) in request.auth.iter().enumerate() {
        let key_input = text_input(&key_ph, &auth_field.key)
            .on_input(move |v| Message::AuthKeyChanged(idx, v))
            .padding(10)
            .size(13);

        let value_input = text_input(&value_ph, &auth_field.value)
            .on_input(move |v| Message::AuthValueChanged(idx, v))
            .padding(10)
            .size(13);

        let remove_button = button(text("×").size(16))
            .on_press(Message::RemoveAuthField(idx))
            .padding([8, 12])
            .style(button::text);

        let auth_row = Row::new()
            .spacing(16)
            .align_y(Alignment::Center)
            .push(container(key_input).width(Length::FillPortion(1)))
            .push(container(value_input).width(Length::FillPortion(2)))
            .push(remove_button);

        auth_column = auth_column.push(auth_row);
    }

    let add_auth_button = button(text(add_text).size(13))
        .on_press(Message::AddAuthField)
        .padding([10, 16])
        .style(button::secondary);

    auth_column = auth_column.push(add_auth_button);

    container(auth_column)
        .padding(20)
        .into()
}
