use iced::widget::{button, container, text, Row};
use iced::Element;

use crate::app::Message;
use crate::models::RequestTab;

pub fn view(active_tab: RequestTab) -> Element<'static, Message> {
    let tabs = RequestTab::all();
    let mut tabs_row = Row::new().spacing(0);

    for tab in tabs {
        let tab_label = tab.as_str();
        let is_active = tab == active_tab;

        let display_text = tab_label.to_string();

        let tab_button = button(text(display_text).size(13))
            .on_press(Message::TabSelected(tab))
            .padding([12, 20])
            .style(if is_active {
                button::primary
            } else {
                button::text
            });

        tabs_row = tabs_row.push(tab_button);
    }

    container(tabs_row).padding([0, 16]).into()
}
