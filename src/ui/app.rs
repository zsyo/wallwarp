// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::{App, NotificationType};
use crate::i18n::I18n;
use crate::ui::main::TrayManager;
use crate::ui::style;
use crate::utils::config::{Config, Theme};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

impl App {
    pub fn new() -> Self {
        let i18n = I18n::new();
        let config = Config::new(&i18n.current_lang);
        Self::new_with_config(i18n, config)
    }

    pub fn new_with_config(mut i18n: I18n, config: Config) -> Self {
        // 根据配置设置语言
        i18n.set_language(config.global.language.clone());

        // 初始化窗口最大化状态（默认为 false）
        let is_maximized = false;

        let tray_manager = TrayManager::new(&i18n);

        // 根据配置文件中的主题配置初始化主题
        let theme_config = match config.global.theme {
            Theme::Dark => style::ThemeConfig::new(style::Theme::Dark),
            Theme::Light => style::ThemeConfig::new(style::Theme::Light),
            Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = crate::utils::window_utils::get_system_color_mode();
                tracing::info!(
                    "[启动] [主题] 自动模式，系统主题: {}",
                    if is_system_dark { "深色" } else { "浅色" }
                );

                if is_system_dark {
                    style::ThemeConfig::new(style::Theme::Dark)
                } else {
                    style::ThemeConfig::new(style::Theme::Light)
                }
            }
        };

        Self {
            i18n,
            config: config.clone(),
            active_page: super::ActivePage::OnlineWallpapers,
            pending_window_size: None,
            debounce_timer: std::time::Instant::now(),
            tray_manager,
            theme_config,
            show_close_confirmation: false,
            show_notification: false,
            notification_message: String::new(),
            notification_type: NotificationType::Success,
            current_window_width: config.display.width,
            current_window_height: config.display.height,
            current_items_per_row: 1, // 初始值为1
            local_state: super::local::LocalState::default(),
            online_state: super::online::OnlineState::load_from_config(&config),
            settings_state: super::settings::SettingsState::load_from_config(&config),
            auto_change_state: super::auto_change::AutoChangeState::load_from_config(&config),
            download_state: super::download::DownloadStateFull::new(),
            initial_loaded: false,                                 // 标记是否已加载初始数据
            auto_change_running: Arc::new(AtomicBool::new(false)), // 初始化定时切换执行标志
            wallpaper_history: {
                // 初始化壁纸切换历史记录，获取当前壁纸路径并添加到记录中
                let mut history = Vec::new();
                if let Ok(current_wallpaper) = wallpaper::get() {
                    if !current_wallpaper.is_empty() {
                        tracing::info!("[壁纸历史] 初始化，添加当前壁纸: {}", current_wallpaper);
                        history.push(current_wallpaper);
                    }
                }
                history
            },
            is_visible: false,
            is_maximized, // 初始化窗口最大化状态
        }
    }

    pub fn title(&self) -> String {
        self.i18n.t("wallwarp")
    }
}
