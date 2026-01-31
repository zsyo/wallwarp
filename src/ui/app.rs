// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::ActivePage;
use crate::i18n::I18n;
use crate::ui::main::TrayManager;
use crate::ui::style;
use crate::utils::config::{Config, Theme};
use crate::utils::window_utils;
use tracing::info;

pub struct App {
    pub i18n: I18n,
    pub config: Config,
    pub active_page: ActivePage,
    pub tray_manager: TrayManager,
    // 主题配置
    pub theme_config: crate::ui::style::ThemeConfig,
    // 主窗口状态
    pub main_state: crate::ui::main::MainState,
    // 本地壁纸页面状态
    pub local_state: crate::ui::local::LocalState,
    // 在线壁纸页面状态
    pub online_state: crate::ui::online::OnlineState,
    // 设置页面状态
    pub settings_state: crate::ui::settings::SettingsState,
    // 下载管理页面状态
    pub download_state: crate::ui::download::DownloadStateFull,
    // 定时切换壁纸状态
    pub auto_change_state: crate::ui::auto_change::AutoChangeState,
    // 壁纸切换历史记录（最多50条）
    pub wallpaper_history: Vec<String>,
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
        i18n.set_language(config.global.language.clone());

        let tray_manager = TrayManager::new(&i18n);

        // 根据配置文件中的主题配置初始化主题
        let theme_config = match config.global.theme {
            Theme::Dark => style::ThemeConfig::new(style::Theme::Dark),
            Theme::Light => style::ThemeConfig::new(style::Theme::Light),
            Theme::Auto => {
                // 自动模式：根据系统主题判断
                let is_system_dark = window_utils::get_system_color_mode();
                info!(
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

        // 初始化壁纸切换历史记录，获取当前壁纸路径并添加到记录中
        let mut wallpaper_history = Vec::new();
        if let Ok(current_wallpaper) = wallpaper::get() {
            if !current_wallpaper.is_empty() {
                tracing::info!(
                    "[壁纸历史] 初始化，添加当前壁纸: {}",
                    crate::utils::helpers::normalize_path(&current_wallpaper)
                );
                wallpaper_history.push(current_wallpaper);
            }
        }

        let mut app = Self {
            i18n,
            config: config.clone(),
            active_page: ActivePage::OnlineWallpapers,
            tray_manager,
            theme_config,
            main_state: super::main::MainState::load_from_config(&config),
            local_state: super::local::LocalState::default(),
            online_state: super::online::OnlineState::load_from_config(&config),
            settings_state: super::settings::SettingsState::load_from_config(&config),
            auto_change_state: super::auto_change::AutoChangeState::load_from_config(&config),
            download_state: super::download::DownloadStateFull::new(),
            wallpaper_history,
        };

        // 初始化托盘菜单项的状态
        app.update_tray_menu_items();

        app
    }

    pub fn title(&self) -> String {
        self.i18n.t("app-title")
    }

    /// 更新托盘菜单项的状态
    fn update_tray_menu_items(&mut self) {
        self.tray_manager
            .update_switch_previous_item(self.wallpaper_history.len());
        self.tray_manager
            .update_save_current_item(self.can_save_current_wallpaper());
    }
}
