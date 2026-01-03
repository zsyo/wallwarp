pub mod message;
pub mod settings;
pub mod view;

use crate::i18n::I18n;
use message::AppMessage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivePage {
    OnlineWallpapers,
    LocalList,
    DownloadProgress,
    Settings,
}

pub struct App {
    i18n: I18n,
    active_page: ActivePage,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            i18n: I18n::new(),
            active_page: ActivePage::OnlineWallpapers,
        }
    }

    pub fn title(&self) -> String {
        self.i18n.t("app-title")
    }

    pub fn update(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::LanguageSelected(lang) => self.i18n.set_language(lang),
            AppMessage::PageSelected(page) => self.active_page = page,
        }
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        self.view_internal()
    }
}
