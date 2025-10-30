mod ai_client;
mod app;
mod config;
mod http_client;
mod i18n;
mod models;
mod storage;
mod ui;
mod utils;

use iced::window;
use iced::{Size, Task, Theme};
use tracing::info;

use app::Requiem;

// Embed logo at compile time
const LOGO_BYTES: &[u8] = include_bytes!("resources/logo.png");

pub fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("Starting Requiem v{}", env!("CARGO_PKG_VERSION"));

    // Load window icon
    let icon = load_icon();

    let app = iced::application(|| (Requiem::new(), Task::none()), update, view)
        .title(|_: &Requiem| String::from("Requiem - Lightweight HTTP Client"))
        .subscription(|state: &Requiem| state.subscription())
        .theme(|_: &Requiem| Theme::Light);

    // Load custom font on Linux where the path exists
    #[cfg(target_os = "linux")]
    let app = app
        .font(include_bytes!(
            "/usr/share/fonts/adobe-source-han-sans/SourceHanSansCN-Regular.otf"
        ))
        .default_font(iced::Font::with_name("Source Han Sans CN"));

    // On macOS, use system fonts that support Chinese
    #[cfg(target_os = "macos")]
    let app = app.default_font(iced::Font {
        family: iced::font::Family::Name("PingFang SC"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    });

    app.window_size((1280.0, 800.0))
        .resizable(true)
        .window(iced::window::Settings {
            min_size: Some(Size::new(1280.0, 800.0)),
            icon: icon.ok(),
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

/// Load application icon from embedded PNG
fn load_icon() -> Result<window::Icon, Box<dyn std::error::Error>> {
    let image = image::load_from_memory(LOGO_BYTES)?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let raw_pixels = rgba.into_raw();

    // Try to create icon from RGBA data
    let icon = window::icon::from_rgba(raw_pixels, width, height)
        .map_err(|e| format!("Failed to create icon: {:?}", e))?;
    Ok(icon)
}
