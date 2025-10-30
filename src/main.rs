#![allow(dead_code)] // Allow unused code during development
#![allow(clippy::multiple_bound_locations)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::clone_on_copy)]
// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    // Set up panic hook to log panics
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };

        let location = if let Some(location) = panic_info.location() {
            format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        } else {
            "Unknown location".to_string()
        };

        eprintln!("PANIC: {} at {}", msg, location);

        // Try to write to log file as well
        if let Ok(log_dir) = get_log_dir() {
            let log_file = log_dir.join("panic.log");
            let _ = std::fs::write(
                log_file,
                format!(
                    "PANIC at {}\n{}\nat {}\n",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    msg,
                    location
                ),
            );
        }
    }));

    // Initialize logging with file output
    init_logging();

    info!("Starting Requiem v{}", env!("CARGO_PKG_VERSION"));
    info!("Platform: {}", std::env::consts::OS);
    info!("Architecture: {}", std::env::consts::ARCH);

    // Load window icon
    let icon = load_icon();

    let app = iced::application(|| (Requiem::new(), Task::none()), update, view)
        .title(|_: &Requiem| String::from("Requiem - Lightweight HTTP Client"))
        .subscription(|state: &Requiem| state.subscription())
        .theme(|_: &Requiem| Theme::Light);

    // On macOS, use system fonts that support Chinese
    #[cfg(target_os = "macos")]
    let app = app.default_font(iced::Font {
        family: iced::font::Family::Name("PingFang SC"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    });

    // On Linux, try to use Source Han Sans CN if available in the system
    // Users can install it via: sudo pacman -S adobe-source-han-sans-otf-fonts (Arch)
    // The font will be picked up automatically by fontconfig
    #[cfg(target_os = "linux")]
    let app = app.default_font(iced::Font {
        family: iced::font::Family::Name("Source Han Sans CN"),
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

/// Get log directory path
fn get_log_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let data_dir = dirs::data_dir().ok_or("Failed to get data directory")?;
    let log_dir = data_dir.join("requiem").join("logs");
    std::fs::create_dir_all(&log_dir)?;
    Ok(log_dir)
}

/// Initialize logging with both stdout and file output
fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(tracing::Level::INFO.into());

    // Try to set up file logging
    if let Ok(log_dir) = get_log_dir() {
        let log_file = log_dir.join(format!(
            "requiem-{}.log",
            chrono::Local::now().format("%Y%m%d")
        ));

        if let Ok(file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
        {
            eprintln!("Logging to: {:?}", log_file);

            use tracing_subscriber::layer::SubscriberExt;
            use tracing_subscriber::util::SubscriberInitExt;

            let file_appender = tracing_subscriber::fmt::layer()
                .with_writer(move || file.try_clone().unwrap())
                .with_target(true)
                .with_ansi(false);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(tracing_subscriber::fmt::layer())
                .with(file_appender)
                .init();
            return;
        }
    }

    // Fallback to console-only logging
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
