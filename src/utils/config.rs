// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::error;

const CONFIG_FILE: &str = "config.toml";
const MIN_WINDOW_WIDTH: u32 = 1280;
const MIN_WINDOW_HEIGHT: u32 = 800;
const DEFAULT_DATA_PATH: &str = "data";
const DEFAULT_CACHE_PATH: &str = "cache";

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub data: DataConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub wallhaven: WallhavenConfig,
    #[serde(default)]
    pub wallpaper: WallpaperConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub close_action: CloseAction,
    #[serde(default)]
    pub proxy: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            close_action: CloseAction::default(),
            proxy: String::new(),
        }
    }
}

fn default_language() -> String {
    "zh-cn".to_string()
}

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
    DEFAULT_CACHE_PATH.to_string()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DisplayConfig {
    #[serde(default = "default_window_width")]
    pub width: u32,
    #[serde(default = "default_window_height")]
    pub height: u32,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            width: default_window_width(),
            height: default_window_height(),
        }
    }
}

fn default_window_width() -> u32 {
    MIN_WINDOW_WIDTH
}

fn default_window_height() -> u32 {
    MIN_WINDOW_HEIGHT
}

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

fn default_top_range() -> String {
    "1M".to_string()
}

fn default_resolution_mode() -> String {
    "all".to_string()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WallpaperConfig {
    #[serde(default)]
    pub mode: WallpaperMode,
    #[serde(default)]
    pub auto_change_mode: WallpaperAutoChangeMode,
    #[serde(default)]
    pub auto_change_interval: WallpaperAutoChangeInterval,
    #[serde(default)]
    pub auto_change_query: String,
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            mode: WallpaperMode::default(),
            auto_change_mode: WallpaperAutoChangeMode::default(),
            auto_change_interval: WallpaperAutoChangeInterval::default(),
            auto_change_query: String::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperAutoChangeMode {
    #[default]
    Local,
    Online,
}

impl WallpaperAutoChangeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperAutoChangeMode::Online => "online",
            WallpaperAutoChangeMode::Local => "local",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "online" => Some(WallpaperAutoChangeMode::Online),
            "local" => Some(WallpaperAutoChangeMode::Local),
            _ => None,
        }
    }
}

impl std::fmt::Display for WallpaperAutoChangeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperAutoChangeMode::Online => write!(f, "Online"),
            WallpaperAutoChangeMode::Local => write!(f, "Local"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WallpaperAutoChangeInterval {
    #[default]
    Off,
    Minutes(u32),
    Custom(u32),
}

impl Serialize for WallpaperAutoChangeInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for WallpaperAutoChangeInterval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(WallpaperAutoChangeInterval::from_str(&s).unwrap_or_default())
    }
}

impl WallpaperAutoChangeInterval {
    pub fn as_str(&self) -> String {
        match self {
            WallpaperAutoChangeInterval::Off => "off".to_string(),
            WallpaperAutoChangeInterval::Minutes(10) => "10m".to_string(),
            WallpaperAutoChangeInterval::Minutes(30) => "30m".to_string(),
            WallpaperAutoChangeInterval::Minutes(60) => "1h".to_string(),
            WallpaperAutoChangeInterval::Minutes(120) => "2h".to_string(),
            WallpaperAutoChangeInterval::Custom(minutes) => format!("custom:{}m", minutes),
            WallpaperAutoChangeInterval::Minutes(_) => {
                // 其他未预定义的 Minutes 值，序列化为 custom: 格式
                if let Some(minutes) = self.get_minutes() {
                    format!("custom:{}m", minutes)
                } else {
                    "off".to_string()
                }
            }
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "off" => Some(WallpaperAutoChangeInterval::Off),
            "10m" => Some(WallpaperAutoChangeInterval::Minutes(10)),
            "30m" => Some(WallpaperAutoChangeInterval::Minutes(30)),
            "1h" => Some(WallpaperAutoChangeInterval::Minutes(60)),
            "2h" => Some(WallpaperAutoChangeInterval::Minutes(120)),
            s if s.starts_with("custom:") && s.ends_with('m') => {
                // 解析 custom:XXm 格式
                let inner = &s[7..s.len() - 1]; // 去掉 "custom:" 和 "m"
                if let Ok(minutes) = inner.parse::<u32>() {
                    if minutes >= 1 && minutes <= 1440 {
                        Some(WallpaperAutoChangeInterval::Custom(minutes))
                    } else {
                        Some(WallpaperAutoChangeInterval::Off)
                    }
                } else {
                    Some(WallpaperAutoChangeInterval::Off)
                }
            }
            _ => Some(WallpaperAutoChangeInterval::Off),
        }
    }

    pub fn get_minutes(&self) -> Option<u32> {
        match self {
            WallpaperAutoChangeInterval::Off => None,
            WallpaperAutoChangeInterval::Minutes(m) => Some(*m),
            WallpaperAutoChangeInterval::Custom(m) => Some(*m),
        }
    }
}

impl std::fmt::Display for WallpaperAutoChangeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperAutoChangeInterval::Off => write!(f, "Off"),
            WallpaperAutoChangeInterval::Minutes(10) => write!(f, "10 min"),
            WallpaperAutoChangeInterval::Minutes(30) => write!(f, "30 min"),
            WallpaperAutoChangeInterval::Minutes(60) => write!(f, "1 hour"),
            WallpaperAutoChangeInterval::Minutes(120) => write!(f, "2 hours"),
            WallpaperAutoChangeInterval::Minutes(m) => write!(f, "{} min", m),
            WallpaperAutoChangeInterval::Custom(m) => write!(f, "{} min", m),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperMode {
    #[default]
    Crop,
    Fit,
    Stretch,
    Tile,
    Center,
    Span,
}

impl WallpaperMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperMode::Crop => "crop",
            WallpaperMode::Fit => "fit",
            WallpaperMode::Stretch => "stretch",
            WallpaperMode::Tile => "tile",
            WallpaperMode::Center => "center",
            WallpaperMode::Span => "span",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "crop" => Some(WallpaperMode::Crop),
            "fit" => Some(WallpaperMode::Fit),
            "stretch" => Some(WallpaperMode::Stretch),
            "tile" => Some(WallpaperMode::Tile),
            "center" => Some(WallpaperMode::Center),
            "span" => Some(WallpaperMode::Span),
            _ => Some(WallpaperMode::Crop),
        }
    }
}

impl std::fmt::Display for WallpaperMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperMode::Crop => write!(f, "Fill"),
            WallpaperMode::Fit => write!(f, "Fit"),
            WallpaperMode::Stretch => write!(f, "Stretch"),
            WallpaperMode::Tile => write!(f, "Tile"),
            WallpaperMode::Center => write!(f, "Center"),
            WallpaperMode::Span => write!(f, "Span"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    #[default]
    Ask,
    MinimizeToTray,
    CloseApp,
}

impl CloseAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            CloseAction::Ask => "ask",
            CloseAction::MinimizeToTray => "minimize_to_tray",
            CloseAction::CloseApp => "close_app",
        }
    }
}

impl std::fmt::Display for CloseAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloseAction::Ask => write!(f, "Ask"),
            CloseAction::MinimizeToTray => write!(f, "MinimizeToTray"),
            CloseAction::CloseApp => write!(f, "CloseApp"),
        }
    }
}

impl Config {
    pub fn new(lang: &str) -> Self {
        let config_path = Path::new(CONFIG_FILE);

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(mut local_config) = toml::from_str::<Config>(&content) {
                local_config.fix_config();
                // 设置语言（优先使用传入的语言）
                local_config.global.language = lang.to_string();
                local_config.save_to_file();
                return local_config;
            } else {
                // 配置文件出错, 终止程序
                panic!("配置文件出错");
            }
        }

        // 配置文件不存在，创建默认配置
        let default_config = Config {
            global: GlobalConfig {
                language: lang.to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        default_config.save_to_file();
        default_config
    }

    fn fix_config(&mut self) {
        if self.display.width < MIN_WINDOW_WIDTH || self.display.height < MIN_WINDOW_HEIGHT {
            self.display.width = MIN_WINDOW_WIDTH;
            self.display.height = MIN_WINDOW_HEIGHT;
        };
    }

    pub fn save_to_file(&self) {
        match toml::to_string_pretty(self) {
            Ok(content) => {
                // 1. 定义警告信息（使用 # 开头）
                let header = concat!(
                    "# ====================================================\n",
                    "# 警告：手动修改此配置文件时请务必谨慎！\n",
                    "# 如果格式填写错误，该项可能会被重置为默认值，甚至导致程序无法启动。\n",
                    "# 建议在修改前备份此文件。\n",
                    "# ====================================================\n\n"
                );

                // 2. 将 header 和 content 拼接在一起
                let full_content = format!("{}{}", header, content);
                let _ = fs::write(CONFIG_FILE, full_content);
            }
            Err(e) => error!("TOML 序化失败: {}", e),
        }
    }

    pub fn update_window_size(&mut self, width: u32, height: u32) {
        self.display.width = width;
        self.display.height = height;
        self.save_to_file();
    }

    pub fn set_language(&mut self, lang: String) {
        self.global.language = lang;
        self.save_to_file();
    }

    pub fn set_close_action(&mut self, action: CloseAction) {
        self.global.close_action = action;
        self.save_to_file();
    }

    pub fn set_data_path(&mut self, path: String) {
        self.data.data_path = path;
        self.save_to_file();
    }

    pub fn set_cache_path(&mut self, path: String) {
        self.data.cache_path = path;
        self.save_to_file();
    }

    pub fn set_wallhaven_api_key(&mut self, key: String) {
        self.wallhaven.api_key = key;
        self.save_to_file();
    }

    pub fn set_proxy(&mut self, proxy: String) {
        self.global.proxy = proxy;
        self.save_to_file();
    }

    pub fn set_wallpaper_mode(&mut self, mode: WallpaperMode) {
        self.wallpaper.mode = mode;
        self.save_to_file();
    }
}
