/// Response Tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTab {
    Body,
    Cookies,
    Headers,
}

impl ResponseTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseTab::Body => "Body",
            ResponseTab::Cookies => "Cookies",
            ResponseTab::Headers => "Headers",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            ResponseTab::Body,
            ResponseTab::Cookies,
            ResponseTab::Headers,
        ]
    }
}

/// Body View Mode - different ways to display response body
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyViewMode {
    Raw,
    Json,
    Xml,
    Html,
}

impl BodyViewMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BodyViewMode::Raw => "Raw",
            BodyViewMode::Json => "JSON",
            BodyViewMode::Xml => "XML",
            BodyViewMode::Html => "HTML",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            BodyViewMode::Raw,
            BodyViewMode::Json,
            BodyViewMode::Xml,
            BodyViewMode::Html,
        ]
    }
}
