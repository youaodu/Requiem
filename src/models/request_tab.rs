/// Request Tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestTab {
    Params,
    Body,
    Headers,
    Cookies,
    Auth,
}

impl RequestTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestTab::Params => "Params",
            RequestTab::Body => "Body",
            RequestTab::Headers => "Headers",
            RequestTab::Cookies => "Cookies",
            RequestTab::Auth => "Auth",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            RequestTab::Params,
            RequestTab::Body,
            RequestTab::Headers,
            RequestTab::Cookies,
            RequestTab::Auth,
        ]
    }
}
