use iced::widget::{text, Column};
use iced::Element;

use crate::app::Message;

use super::dialog;

pub fn view<'a>() -> Element<'a, Message> {
    let content = Column::new()
        .spacing(20)
        .push(text("这里可以管理自定义环境").size(14))
        .push(text("功能开发中...").size(12));

    dialog::view(
        "环境管理",
        content.into(),
        "关闭",
        Message::CloseEnvironmentDialog,
        500.0,
        300.0,
    )
}
