// Copyright (C) 2026 zsyo - GNU AGPL v3.0

mod load_from_config;

use crate::services::wallhaven::{self, Sorting, TimeRange};
use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode, WallpaperMode};

/// 设置页面相关状态
#[derive(Debug, Clone)]
pub struct SettingsState {
    // 下拉框展开状态
    pub language_picker_expanded: bool,
    pub proxy_protocol_picker_expanded: bool,
    pub theme_picker_expanded: bool,
    pub log_level_picker_expanded: bool,

    // 代理设置临时状态
    pub proxy_enabled: bool,
    pub proxy_protocol: String,
    pub proxy_address: String,
    pub proxy_port: u32,

    // API 设置临时状态
    pub wallhaven_api_key: String,
    /// Wallhaven API Key 的脱敏显示串，隐藏状态下输入框展示用
    pub wallhaven_api_key_masked: String,
    /// Wallhaven API Key 输入框内容是否可见（仅内存状态，默认隐藏，重启后恢复隐藏）
    pub wallhaven_api_key_visible: bool,

    // 壁纸设置临时状态
    pub wallpaper_mode: WallpaperMode,
    pub auto_change_mode: WallpaperAutoChangeMode,
    pub auto_change_interval: WallpaperAutoChangeInterval,
    pub custom_interval_minutes: u32,
    pub auto_change_query: String,
    pub auto_change_sorting: Sorting,
    pub auto_change_time_range: TimeRange,
    pub sorting_picker_expanded: bool,
    pub time_range_picker_expanded: bool,

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
            log_level_picker_expanded: false,
            proxy_enabled: false,
            proxy_protocol: "http".to_string(),
            proxy_address: String::new(),
            proxy_port: 1080,
            wallhaven_api_key: String::new(),
            wallhaven_api_key_masked: String::new(),
            wallhaven_api_key_visible: false,
            wallpaper_mode: WallpaperMode::default(),
            auto_change_mode: WallpaperAutoChangeMode::default(),
            auto_change_interval: WallpaperAutoChangeInterval::default(),
            custom_interval_minutes: 30,
            auto_change_query: String::new(),
            auto_change_sorting: Sorting::DateAdded,
            auto_change_time_range: TimeRange::Month,
            sorting_picker_expanded: false,
            time_range_picker_expanded: false,
            show_path_clear_confirmation: false,
            path_to_clear: String::new(),
        }
    }
}

impl SettingsState {
    /// 刷新 Wallhaven API Key 的脱敏显示串：空 Key 显示为空，其余保留前4位与后4位，中间以4个星号代替
    pub fn refresh_wallhaven_api_key_masked(&mut self) {
        self.wallhaven_api_key_masked = if self.wallhaven_api_key.is_empty() {
            String::new()
        } else {
            wallhaven::mask_api_key(&self.wallhaven_api_key)
        };
    }
}
