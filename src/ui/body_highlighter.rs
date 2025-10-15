use iced::advanced::text::highlighter;
use iced::Color;
use std::ops::Range;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

/// Settings for the body highlighter
#[derive(Debug, Clone, PartialEq)]
pub struct HighlighterSettings {
    pub language: BodyLanguage,
}

/// Supported languages for body highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyLanguage {
    Json,
    Xml,
    Html,
    Plain,
}

impl BodyLanguage {
    pub fn syntax_name(&self) -> &'static str {
        match self {
            BodyLanguage::Json => "JSON",
            BodyLanguage::Xml => "XML",
            BodyLanguage::Html => "HTML",
            BodyLanguage::Plain => "Plain Text",
        }
    }
}

/// Body highlighter using syntect
pub struct BodyHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    language: BodyLanguage,
    current_line_index: usize,
    lines: Vec<Vec<(Range<usize>, Color)>>,
}

impl BodyHighlighter {
    pub fn new(settings: &HighlighterSettings) -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            language: settings.language,
            current_line_index: 0,
            lines: Vec::new(),
        }
    }

    fn syntect_to_iced_color(style: Style) -> Color {
        Color::from_rgb(
            style.foreground.r as f32 / 255.0,
            style.foreground.g as f32 / 255.0,
            style.foreground.b as f32 / 255.0,
        )
    }

    fn highlight_text_line(&self, line: &str) -> Vec<(Range<usize>, Color)> {
        let mut result = Vec::new();

        // Get syntax definition
        let syntax = self
            .syntax_set
            .find_syntax_by_name(self.language.syntax_name())
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        // Get theme
        let theme = &self.theme_set.themes["base16-ocean.dark"];

        // Highlight the line
        let mut highlighter_engine = HighlightLines::new(syntax, theme);
        if let Ok(ranges) = highlighter_engine.highlight_line(line, &self.syntax_set) {
            let mut offset = 0;
            for (style, text) in ranges {
                let len = text.len();
                if len > 0 {
                    let color = Self::syntect_to_iced_color(style);
                    result.push((offset..offset + len, color));
                    offset += len;
                }
            }
        }

        result
    }
}

impl highlighter::Highlighter for BodyHighlighter {
    type Settings = HighlighterSettings;
    type Highlight = Color;
    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        BodyHighlighter::new(settings)
    }

    fn update(&mut self, settings: &Self::Settings) {
        // Update language if settings changed
        self.language = settings.language;
    }

    fn change_line(&mut self, line_index: usize) {
        self.current_line_index = line_index;
    }

    fn current_line(&self) -> usize {
        self.current_line_index
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let highlights = self.highlight_text_line(line);
        Box::new(highlights.into_iter())
    }
}
