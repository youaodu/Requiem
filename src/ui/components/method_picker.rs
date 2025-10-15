use iced::widget::pick_list;
use iced::{Element, Length};

use crate::app::Message;
use crate::models::HttpMethod;

pub fn view(current_method: HttpMethod) -> Element<'static, Message> {
    let methods = vec![
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::PATCH,
        HttpMethod::DELETE,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
    ];

    pick_list(methods, Some(current_method), Message::MethodSelected)
        .width(Length::Fixed(100.0))
        .padding(8)
        .into()
}
