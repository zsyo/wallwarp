pub mod message;
pub mod settings;
pub mod update;
pub mod view;

use crate::i18n::I18n;
use crate::utils::config::Config;
use iced;
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
    config: Config,
    active_page: ActivePage,
    pending_window_size: Option<(u32, u32)>,
    pending_window_position: Option<(i32, i32)>,
    debounce_timer: std::time::Instant,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let i18n = I18n::new();
        let config = Config::new(&i18n.current_lang);
        Self::new_with_config(i18n, config)
    }

    pub fn new_with_config(mut i18n: I18n, config: Config) -> Self {
        // 根据配置设置语言
        i18n.set_language(config.language.clone());

        Self {
            i18n,
            config,
            active_page: ActivePage::OnlineWallpapers,
            pending_window_size: None,
            pending_window_position: None,
            debounce_timer: std::time::Instant::now(),
        }
    }

    pub fn title(&self) -> String {
        self.i18n.t("app-title")
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        self.view_internal()
    }
}
