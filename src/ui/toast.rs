use iced::widget::{container, row, text};
use iced::{Background, Border, Color, Element, Shadow, Theme};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Info,
    Warning,
}

impl ToastType {
    pub fn background_color(&self) -> Color {
        match self {
            ToastType::Success => Color::from_rgb(0.22, 0.71, 0.29), // Green
            ToastType::Error => Color::from_rgb(0.86, 0.24, 0.24),   // Red
            ToastType::Info => Color::from_rgb(0.13, 0.59, 0.95),    // Blue
            ToastType::Warning => Color::from_rgb(0.95, 0.61, 0.07), // Orange
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            ToastType::Success => "✓",
            ToastType::Error => "✗",
            ToastType::Info => "ℹ",
            ToastType::Warning => "⚠",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Duration,
}

impl Toast {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            toast_type: ToastType::Success,
            duration: Duration::from_secs(2),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            toast_type: ToastType::Error,
            duration: Duration::from_secs(3),
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            toast_type: ToastType::Info,
            duration: Duration::from_secs(2),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            toast_type: ToastType::Warning,
            duration: Duration::from_secs(3),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

pub fn view<'a, Message: 'a>(toast: &'a Toast) -> Element<'a, Message> {
    let icon = text(toast.toast_type.icon())
        .size(16)
        .color(Color::WHITE);

    let message_text = text(&toast.message)
        .size(14)
        .color(Color::WHITE);

    let content = row![icon, message_text]
        .spacing(8)
        .align_y(iced::Alignment::Center);

    let bg_color = toast.toast_type.background_color();

    container(content)
        .padding([12, 20])
        .max_width(400.0)
        .style(move |_theme: &Theme| container::Style {
            background: Some(Background::Color(bg_color)),
            text_color: Some(Color::WHITE),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
        })
        .into()
}
