mod app;
mod config;
mod http_client;
mod i18n;
mod models;
mod storage;
mod ui;
mod utils;

use iced::{Element, Size, Task, Theme};
use iced::keyboard::{self, Key, Modifiers};
use iced::event::{self, Event};
use iced::mouse;
use tracing::info;

use app::{Message, Requiem};

pub fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Requiem v{}", env!("CARGO_PKG_VERSION"));

    iced::application(title, update, view)
        .subscription(subscription)
        .theme(|_| Theme::Light)
        .font(include_bytes!("/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf"))
        .default_font(iced::Font::with_name("Source Han Sans CN"))
        .window_size((1280.0, 800.0))
        .resizable(true)
        .window(iced::window::Settings {
            min_size: Some(Size::new(1280.0, 800.0)),
            ..Default::default()
        })
        .run_with(|| (Requiem::new(), Task::none()))
}

fn title(_state: &Requiem) -> String {
    String::from("Requiem - Lightweight HTTP Client")
}

fn update(state: &mut Requiem, message: Message) -> Task<Message> {
    state.update(message)
}

fn view(state: &Requiem) -> Element<'_, Message> {
    ui::view(state)
}

fn subscription(_state: &Requiem) -> iced::Subscription<Message> {
    event::listen_with(handle_event)
}

fn handle_event(event: Event, status: event::Status, _window: iced::window::Id) -> Option<Message> {
    // Always handle mouse events globally, even if captured
    if let Event::Mouse(mouse::Event::CursorMoved { position }) = event {
        return Some(Message::MouseMoved(position.x, position.y));
    }

    // Only handle other events if not captured by widgets
    if matches!(status, event::Status::Captured) {
        return None;
    }

    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key,
            modifiers,
            ..
        }) => {
            match &key {
                Key::Character(c) if modifiers.contains(Modifiers::CTRL) && c == "s" => {
                    return Some(Message::SaveRequest);
                }
                Key::Named(keyboard::key::Named::Escape) => {
                    return Some(Message::CancelRename);
                }
                Key::Named(keyboard::key::Named::ArrowLeft) => {
                    return Some(Message::MoveActiveTabLeft);
                }
                Key::Named(keyboard::key::Named::ArrowRight) => {
                    return Some(Message::MoveActiveTabRight);
                }
                _ => {}
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            // End drag when left mouse button is released
            // Also trigger tab press end with current mouse position
            return Some(Message::TabDragEnd);
        }
        _ => {}
    }
    None
}
