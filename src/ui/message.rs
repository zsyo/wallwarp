use super::ActivePage;

#[derive(Debug, Clone)]
pub enum AppMessage {
    LanguageSelected(String),
    PageSelected(ActivePage),
}