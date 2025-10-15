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
