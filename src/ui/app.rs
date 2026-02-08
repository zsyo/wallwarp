// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::ActivePage;
use crate::i18n::I18n;
use crate::ui::main::TrayManager;
use crate::ui::style;
use crate::utils::assets;
use crate::utils::config::{Config, Theme};
use crate::utils::window_utils;
use iced::widget::image::Handle;
use std::path::PathBuf;
use tracing::{error, info};

pub struct App {
    pub i18n: I18n,
    pub config: Config,
    pub active_page: ActivePage,
    pub tray_manager: TrayManager,
    /// 主题配置
    pub theme_config: crate::ui::style::ThemeConfig,
    /// 主题颜色缓存（仅在主题切换时更新）
    pub theme_colors: crate::ui::style::ThemeColors,
    /// 主窗口状态
    pub main_state: crate::ui::main::MainState,
    /// 本地壁纸页面状态
    pub local_state: crate::ui::local::LocalState,
    /// 在线壁纸页面状态
    pub online_state: crate::ui::online::OnlineState,
    /// 设置页面状态
    pub settings_state: crate::ui::settings::SettingsState,
    /// 下载管理页面状态
    pub download_state: crate::ui::download::DownloadStateFull,
    /// 定时切换壁纸状态
    pub auto_change_state: crate::ui::auto_change::AutoChangeState,
    /// 壁纸切换历史记录（最多50条）
    pub wallpaper_history: Vec<String>,
    /// 图标资源
    pub logo_handle: Handle,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let i18n = I18n::new();
        let config = Config::new(&i18n.current_lang, &i18n.available_langs);
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

        // 初始化主题颜色缓存
        let theme_colors = style::ThemeColors::from_theme(theme_config.get_theme());

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

        let (img, width, height) = assets::get_logo(style::LOGO_SIZE);

        let mut app = Self {
            i18n,
            config: config.clone(),
            active_page: ActivePage::OnlineWallpapers,
            tray_manager,
            theme_config,
            theme_colors,
            main_state: super::main::MainState::load_from_config(&config),
            local_state: super::local::LocalState::default(),
            online_state: super::online::OnlineState::load_from_config(&config),
            settings_state: super::settings::SettingsState::load_from_config(&config),
            auto_change_state: super::auto_change::AutoChangeState::load_from_config(&config),
            download_state: super::download::DownloadStateFull::new(),
            wallpaper_history,
            logo_handle: Handle::from_rgba(width, height, img),
        };

        // 初始化下载任务数据库
        app.init_download_database();

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

    /// 初始化下载任务数据库
    fn init_download_database(&mut self) {
        // 数据库文件存储在程序目录下的 db 子目录中
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let db_dir = current_dir.join("db");
        let db_path = db_dir.join("data.db");

        info!("[启动] [下载任务数据库] 初始化数据库: {}", db_path.display());

        match self.download_state.init_database(&db_path.to_string_lossy()) {
            Ok(_) => {
                info!("[启动] [下载任务数据库] 数据库初始化成功");

                // 从数据库加载任务
                match self.download_state.load_from_database() {
                    Ok(count) => {
                        info!("[启动] [下载任务数据库] 加载了 {} 个任务", count);
                    }
                    Err(e) => {
                        error!("[启动] [下载任务数据库] 加载任务失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("[启动] [下载任务数据库] 数据库初始化失败: {}", e);
            }
        }
    }
}
