use super::App;
use super::AppMessage;
use crate::i18n::I18n;
use crate::utils::config::Config;
use iced;

impl App {
    pub fn new() -> Self {
        let i18n = I18n::new();
        let config = Config::new(&i18n.current_lang);
        Self::new_with_config(i18n, config)
    }

    pub fn new_with_config(mut i18n: I18n, config: Config) -> Self {
        // 根据配置设置语言
        i18n.set_language(config.language.clone());

        let _tray_icon = Self::init_tray(&i18n);

        Self {
            i18n,
            config,
            active_page: super::ActivePage::OnlineWallpapers,
            pending_window_size: None,
            pending_window_position: None,
            debounce_timer: std::time::Instant::now(),
            _tray_icon,
        }
    }

    pub fn title(&self) -> String {
        self.i18n.t("app-title")
    }

    pub fn view(&self) -> iced::Element<'_, AppMessage> {
        self.view_internal()
    }
}
