// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};

/// 设置页面相关状态
#[derive(Debug, Clone)]
pub struct SettingsState {
    // 下拉框展开状态
    pub language_picker_expanded: bool,
    pub proxy_protocol_picker_expanded: bool,
    pub theme_picker_expanded: bool,

    // 代理设置临时状态
    pub proxy_protocol: String,
    pub proxy_address: String,
    pub proxy_port: u32,

    // API 设置临时状态
    pub wallhaven_api_key: String,

    // 壁纸设置临时状态
    pub wallpaper_mode: WallpaperMode,
    pub auto_change_mode: WallpaperAutoChangeMode,
    pub auto_change_interval: WallpaperAutoChangeInterval,
    pub custom_interval_minutes: u32,
    pub auto_change_query: String,

    // 对话框状态
    pub show_path_clear_confirmation: bool,
    pub path_to_clear: String,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            language_picker_expanded: false,
            proxy_protocol_picker_expanded: false,
            theme_picker_expanded: false,
            proxy_protocol: "http".to_string(),
            proxy_address: String::new(),
            proxy_port: 1080,
            wallhaven_api_key: String::new(),
            wallpaper_mode: WallpaperMode::default(),
            auto_change_mode: WallpaperAutoChangeMode::default(),
            auto_change_interval: WallpaperAutoChangeInterval::default(),
            custom_interval_minutes: 30,
            auto_change_query: String::new(),
            show_path_clear_confirmation: false,
            path_to_clear: String::new(),
        }
    }
}
