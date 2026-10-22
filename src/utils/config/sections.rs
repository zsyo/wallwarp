// Copyright (C) 2026 zsyo - GNU GPL v3.0

//! 配置的各分节结构体（global/data/display/wallhaven/wallpaper）

use super::{DEFAULT_DATA_PATH, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use crate::utils::config::enums::{
    CloseAction, LogLevel, Theme, WallpaperAutoChangeInterval, WallpaperAutoChangeMode,
    WallpaperMode, WallpaperReadability,
};
use serde::{Deserialize, Serialize};

/// 应用全局配置（语言/主题/代理/热键等）
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub enable_logging: bool,
    /// 日志等级（运行日志开启时生效，可在设置页实时调整）
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(default)]
    pub close_action: CloseAction,
    #[serde(default)]
    pub proxy: String,
    #[serde(default = "default_proxy_enabled")]
    pub proxy_enabled: bool,
    /// 是否显示桌面悬浮球
    #[serde(default)]
    pub show_floating_ball: bool,
    /// 悬浮球位置 x（逻辑坐标，i32::MIN 表示未设置，使用默认位置）
    #[serde(default = "default_floating_ball_pos")]
    pub floating_ball_x: i32,
    /// 悬浮球位置 y（逻辑坐标，i32::MIN 表示未设置，使用默认位置）
    #[serde(default = "default_floating_ball_pos")]
    pub floating_ball_y: i32,
    /// 全局热键"切换下一张壁纸"（HotKey 字符串，空 = 未设置）
    #[serde(default)]
    pub hotkey_switch_next: String,
    /// 全局热键"切换上一张壁纸"（HotKey 字符串，空 = 未设置）
    #[serde(default)]
    pub hotkey_switch_previous: String,
    /// 全局热键"显示窗口"（HotKey 字符串，空 = 未设置）
    #[serde(default)]
    pub hotkey_show_window: String,
    /// 全局热键"保存当前壁纸到库"（HotKey 字符串，空 = 未设置）
    #[serde(default)]
    pub hotkey_save_current: String,
    /// 当前在线图源（services::source::SourceKind 的 id，默认 wallhaven）
    #[serde(default = "default_online_source")]
    pub online_source: String,
    /// 日志展示窗口的等级（仅控制该窗口显示，不影响文件与控制台日志）
    #[serde(default)]
    pub log_window_level: LogLevel,
    /// 日志展示窗口最大保留行数（区间 50-10000，setter 与 UI 校验共用）
    #[serde(default = "default_log_window_max_lines")]
    pub log_window_max_lines: u32,
}

/// 日志展示窗口最大保留行数下限
pub const LOG_WINDOW_MAX_LINES_MIN: u32 = 50;
/// 日志展示窗口最大保留行数上限
pub const LOG_WINDOW_MAX_LINES_MAX: u32 = 10000;

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: Theme::default(),
            enable_logging: false,
            log_level: LogLevel::default(),
            close_action: CloseAction::default(),
            proxy: String::new(),
            proxy_enabled: default_proxy_enabled(),
            show_floating_ball: false,
            floating_ball_x: default_floating_ball_pos(),
            floating_ball_y: default_floating_ball_pos(),
            hotkey_switch_next: String::new(),
            hotkey_switch_previous: String::new(),
            hotkey_show_window: String::new(),
            hotkey_save_current: String::new(),
            online_source: default_online_source(),
            log_window_level: LogLevel::default(),
            log_window_max_lines: default_log_window_max_lines(),
        }
    }
}

fn default_language() -> String {
    "zh-cn".to_string()
}

fn default_proxy_enabled() -> bool {
    true
}

fn default_floating_ball_pos() -> i32 {
    i32::MIN
}

fn default_online_source() -> String {
    "wallhaven".to_string()
}

fn default_log_window_max_lines() -> u32 {
    1000
}

/// 数据与缓存目录配置
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataConfig {
    #[serde(default = "default_data_path")]
    pub data_path: String,
    #[serde(default = "default_cache_path")]
    pub cache_path: String,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            data_path: default_data_path(),
            cache_path: default_cache_path(),
        }
    }
}

fn default_data_path() -> String {
    DEFAULT_DATA_PATH.to_string()
}

fn default_cache_path() -> String {
    crate::utils::helpers::default_cache_path()
}

/// 主窗口显示配置
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DisplayConfig {
    #[serde(default = "default_window_width")]
    pub width: u32,
    #[serde(default = "default_window_height")]
    pub height: u32,
    /// 主窗口位置 x（逻辑坐标，i32::MIN 表示未设置，使用系统默认位置）
    #[serde(default = "default_window_pos")]
    pub x: i32,
    /// 主窗口位置 y（逻辑坐标，i32::MIN 表示未设置，使用系统默认位置）
    #[serde(default = "default_window_pos")]
    pub y: i32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            width: default_window_width(),
            height: default_window_height(),
            x: i32::MIN,
            y: i32::MIN,
        }
    }
}

fn default_window_pos() -> i32 {
    i32::MIN
}

fn default_window_width() -> u32 {
    MIN_WINDOW_WIDTH
}

fn default_window_height() -> u32 {
    MIN_WINDOW_HEIGHT
}

/// Wallhaven 图源筛选配置
///
/// 同时承载各图源共用的在线筛选字段（source_color/site_category 由
/// wallpaperscraft/upx8 使用），图源无关字段按需由各图源 service 层映射，
/// 无能力的图源忽略
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WallhavenConfig {
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default = "default_purity")]
    pub purity: String,
    #[serde(default = "default_sorting")]
    pub sorting: String,
    #[serde(default = "default_color")]
    pub color: String,
    /// Basic 档图源共用色板（SourceColor 取值，默认 any 不限）
    #[serde(default = "default_source_color")]
    pub source_color: String,
    /// 站点内容分类（SiteCategory 取值，默认空 = 不限）
    #[serde(default)]
    pub site_category: String,
    #[serde(default = "default_top_range")]
    pub top_range: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_resolution_mode")]
    pub resolution_mode: String,
    #[serde(default)]
    pub atleast_resolution: String,
    #[serde(default)]
    pub resolutions: String,
    #[serde(default)]
    pub ratios: String,
}

impl Default for WallhavenConfig {
    fn default() -> Self {
        Self {
            category: default_category(),
            purity: default_purity(),
            sorting: default_sorting(),
            color: default_color(),
            source_color: default_source_color(),
            site_category: String::new(),
            top_range: default_top_range(),
            api_key: String::new(),
            resolution_mode: default_resolution_mode(),
            atleast_resolution: String::new(),
            resolutions: String::new(),
            ratios: String::new(),
        }
    }
}

fn default_category() -> String {
    "111".to_string()
}

fn default_purity() -> String {
    "sfw".to_string()
}

fn default_sorting() -> String {
    "date_added".to_string()
}

fn default_color() -> String {
    "any".to_string()
}

fn default_source_color() -> String {
    "any".to_string()
}

fn default_top_range() -> String {
    "1M".to_string()
}

fn default_resolution_mode() -> String {
    "all".to_string()
}

fn default_auto_change_sorting() -> String {
    "random".to_string()
}

fn default_auto_change_top_range() -> String {
    "1M".to_string()
}

/// 定时切换启用的持久化默认值
///
/// 旧版本配置中没有该字段，旧语义是"周期非 Off 即启用"，
/// 因此缺字段时按 true 迁移；旧配置周期为 Off 的由 fix_config 修正为 false
fn default_auto_change_enabled() -> bool {
    true
}

/// 壁纸相关配置（铺满模式与定时切换）
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WallpaperConfig {
    #[serde(default)]
    pub mode: WallpaperMode,
    #[serde(default)]
    pub auto_change_mode: WallpaperAutoChangeMode,
    #[serde(default)]
    pub auto_change_interval: WallpaperAutoChangeInterval,
    #[serde(default = "default_auto_change_enabled")]
    pub auto_change_enabled: bool,
    #[serde(default)]
    pub auto_change_query: String,
    #[serde(default = "default_auto_change_sorting")]
    pub auto_change_sorting: String,
    #[serde(default = "default_auto_change_top_range")]
    pub auto_change_top_range: String,
    /// 定时切换本地选图按系统深浅色分组（深浅色各用独立目录）
    #[serde(default)]
    pub auto_change_theme_aware: bool,
    /// 浅色主题本地选图目录（空 = 沿用 data_path）
    #[serde(default)]
    pub auto_change_light_dir: String,
    /// 深色主题本地选图目录（空 = 沿用 data_path）
    #[serde(default)]
    pub auto_change_dark_dir: String,
    /// 壁纸可读性处理（设壁纸时生成处理副本，系统壁纸指向副本）
    #[serde(default)]
    pub readability: WallpaperReadability,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            mode: WallpaperMode::default(),
            auto_change_mode: WallpaperAutoChangeMode::default(),
            auto_change_interval: WallpaperAutoChangeInterval::default(),
            auto_change_enabled: false,
            auto_change_query: String::new(),
            auto_change_sorting: default_auto_change_sorting(),
            auto_change_top_range: default_auto_change_top_range(),
            auto_change_theme_aware: false,
            auto_change_light_dir: String::new(),
            auto_change_dark_dir: String::new(),
            readability: WallpaperReadability::default(),
        }
    }
}
