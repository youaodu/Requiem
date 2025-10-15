use iced::widget::{container, text_editor};
use iced::{Border, Color, Element, Length, Padding, Theme};

/// A reusable multiline text area component with customizable styling
pub fn textarea<'a, Message: 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone,
{
    textarea_builder(content, on_action)
        .height(Length::Fill)
        .placeholder("Enter text...")
        .build()
}

/// A builder for creating customizable text area components
pub struct TextAreaBuilder<'a, Message>
where
    Message: Clone + 'a,
{
    content: &'a text_editor::Content,
    on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
    placeholder: String,
    height: Length,
    font_size: u16,
    padding: Padding,
    border_width: f32,
    border_color: Color,
    border_radius: f32,
    background_color: Option<Color>,
    read_only: bool,
}

impl<'a, Message> TextAreaBuilder<'a, Message>
where
    Message: Clone + 'a,
{
    fn new(
        content: &'a text_editor::Content,
        on_action: impl Fn(text_editor::Action) -> Message + 'a,
    ) -> Self {
        Self {
            content,
            on_action: Some(Box::new(on_action)),
            placeholder: String::new(),
            height: Length::Fixed(200.0),
            font_size: 13,
            padding: Padding::new(8.0),
            border_width: 1.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 4.0,
            background_color: None,
            read_only: false,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the height of the text area
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the font size
    pub fn font_size(mut self, size: u16) -> Self {
        self.font_size = size;
        self
    }

    /// Set the padding
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the border width
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    /// Set the border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Set the border radius
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    /// Set the read-only state (disables editing)
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Build the text area element
    pub fn build(self) -> Element<'a, Message> {
        let placeholder = self.placeholder.clone();
        let read_only = self.read_only;

        let mut editor = text_editor(self.content)
            .height(self.height)
            .padding(self.padding)
            .size(self.font_size);

        // Add placeholder if not empty
        if !placeholder.is_empty() {
            editor = editor.placeholder(placeholder);
        }

        // Always attach on_action to support text selection even in read-only mode
        if let Some(on_action) = self.on_action {
            editor = editor.on_action(on_action);
        }

        // Apply custom style for read-only mode to remove focus border
        // Note: iced 0.13 doesn't support hiding cursor, but we can make it transparent via icon color
        if read_only {
            editor = editor.style(|_theme, _status| {
                use iced::widget::text_editor::Style;
                Style {
                    background: iced::Background::Color(Color::WHITE),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    icon: Color::WHITE, // Make cursor same color as background to hide it
                    placeholder: Color::from_rgb(0.7, 0.7, 0.7),
                    value: Color::BLACK,
                    selection: Color::from_rgb(0.7, 0.85, 1.0),
                }
            });
        }

        let border_width = self.border_width;
        let border_color = self.border_color;
        let border_radius = self.border_radius;
        let background_color = self.background_color;

        container(editor)
            .width(Length::Fill)
            .style(move |_theme: &Theme| container::Style {
                border: Border {
                    color: border_color,
                    width: border_width,
                    radius: border_radius.into(),
                },
                background: background_color.or_else(|| {
                    // Light gray background for read-only
                    if read_only {
                        Some(Color::from_rgb(0.95, 0.95, 0.95))
                    } else {
                        None
                    }
                }).map(|c| c.into()),
                ..Default::default()
            })
            .into()
    }
}

/// Create a text area builder for advanced customization
pub fn textarea_builder<'a, Message: 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> TextAreaBuilder<'a, Message>
where
    Message: Clone,
{
    TextAreaBuilder::new(content, on_action)
}

/// A compact text area with minimal styling
pub fn compact_textarea<'a, Message: 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone,
{
    textarea_builder(content, on_action)
        .height(Length::Fixed(100.0))
        .font_size(12)
        .padding(Padding::new(4.0))
        .border_width(1.0)
        .border_color(Color::from_rgb(0.8, 0.8, 0.8))
        .build()
}

/// A text area with no border (borderless style)
pub fn borderless_textarea<'a, Message: 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
) -> Element<'a, Message>
where
    Message: Clone,
{
    textarea_builder(content, on_action)
        .height(Length::Fill)
        .border_width(0.0)
        .border_color(Color::TRANSPARENT)
        .padding(Padding::new(8.0))
        .build()
}
