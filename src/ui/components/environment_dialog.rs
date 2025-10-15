use iced::widget::{button, container, text, Column, Row};
use iced::{Color, Element, Length};

use crate::app::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let title = text("环境管理").size(18);

    let content = Column::new()
        .spacing(20)
        .padding(20)
        .push(title)
        .push(text("这里可以管理自定义环境").size(14))
        .push(text("功能开发中...").size(12));

    let close_button = button(text("关闭").size(14))
        .on_press(Message::CloseEnvironmentDialog)
        .padding([10, 20])
        .style(button::primary);

    let dialog_content = Column::new()
        .spacing(20)
        .push(content)
        .push(
            Row::new()
                .spacing(10)
                .push(container(text("")).width(Length::Fill))
                .push(close_button),
        );

    container(dialog_content)
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(300.0))
        .padding(20)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
}
