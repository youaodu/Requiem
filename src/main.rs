mod app;
mod config;
mod http_client;
mod i18n;
mod models;
mod storage;
mod ui;
mod utils;

use iced::{Size, Task, Theme};
use tracing::info;

use app::Requiem;

pub fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Requiem v{}", env!("CARGO_PKG_VERSION"));

    iced::application(
        || (Requiem::new(), Task::none()),
        update,
        view
    )
        .title(|_: &Requiem| String::from("Requiem - Lightweight HTTP Client"))
        .subscription(|state: &Requiem| state.subscription())
        .theme(|_: &Requiem| Theme::Light)
        .font(include_bytes!("/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf"))
        .default_font(iced::Font::with_name("Source Han Sans CN"))
        .window_size((1280.0, 800.0))
        .resizable(true)
        .window(iced::window::Settings {
            min_size: Some(Size::new(1280.0, 800.0)),
            platform_specific: iced::window::settings::PlatformSpecific {
                #[cfg(target_os = "linux")]
                application_id: String::from("com.requiem.app"),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}

fn update(state: &mut Requiem, msg: app::Message) -> Task<app::Message> {
    state.update(msg)
}

fn view(state: &Requiem) -> iced::Element<'_, app::Message> {
    state.view()
}
