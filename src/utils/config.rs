use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::error;

const CONFIG_FILE: &str = "config.toml";
const DEFAULT_WINDOW_WIDTH: u32 = 1200;
const DEFAULT_WINDOW_HEIGHT: u32 = 800;
const MIN_WINDOW_WIDTH: u32 = 1280;
const MIN_WINDOW_HEIGHT: u32 = 800;
const DEFAULT_DATA_PATH: &str = "data";
const DEFAULT_CACHE_PATH: &str = "cache";

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GlobalConfig {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub auto_startup: bool,
    #[serde(default)]
    pub close_action: CloseAction,
    #[serde(default)]
    pub proxy: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DataConfig {
    #[serde(default)]
    pub data_path: String,
    #[serde(default)]
    pub cache_path: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DisplayConfig {
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct WallhavenConfig {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub purity: String,
    #[serde(default)]
    pub sorting: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub top_range: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub resolution_mode: String,
    #[serde(default)]
    pub atleast_resolution: String,
    #[serde(default)]
    pub resolutions: String,
    #[serde(default)]
    pub ratios: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct WallpaperConfig {
    #[serde(default)]
    pub mode: WallpaperMode,
    #[serde(default)]
    pub auto_change_mode: WallpaperAutoChangeMode,
    #[serde(default)]
    pub auto_change_interval: WallpaperAutoChangeInterval,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperAutoChangeMode {
    Online,
    Local,
}

impl Default for WallpaperAutoChangeMode {
    fn default() -> Self {
        WallpaperAutoChangeMode::Local
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallpaperAutoChangeInterval {
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
        WallpaperAutoChangeInterval::from_str(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("invalid wallpaper auto change interval: {}", s)))
    }
}

impl Default for WallpaperAutoChangeInterval {
    fn default() -> Self {
        WallpaperAutoChangeInterval::Off
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
            WallpaperAutoChangeInterval::Minutes(_) | WallpaperAutoChangeInterval::Custom(_) => {
                if let Some(minutes) = self.get_minutes() {
                    format!("{}m", minutes)
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
            s if s.ends_with('m') => {
                let minutes_str = &s[..s.len() - 1];
                if let Ok(minutes) = minutes_str.parse::<u32>() {
                    if minutes >= 1 {
                        Some(WallpaperAutoChangeInterval::Custom(minutes))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
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

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WallpaperMode {
    Crop,
    Fit,
    Stretch,
    Tile,
    Center,
    Span,
}

impl Default for WallpaperMode {
    fn default() -> Self {
        WallpaperMode::Stretch
    }
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
            _ => None,
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

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    Ask,
    MinimizeToTray,
    CloseApp,
}

impl Default for CloseAction {
    fn default() -> Self {
        CloseAction::Ask
    }
}

impl Config {
    pub fn new(lang: &str) -> Self {
        let config_path = Path::new(CONFIG_FILE);
        let default_config = Self::default_with_lang(lang);

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(mut local_config) = toml::from_str::<Config>(&content) {
                local_config.fix_config();
                // 合并缺失的配置项
                if local_config.global.language.is_empty() {
                    local_config.global.language = default_config.global.language;
                }
                if local_config.global.proxy.is_empty() {
                    local_config.global.proxy = default_config.global.proxy;
                }
                if local_config.data.data_path.is_empty() {
                    local_config.data.data_path = default_config.data.data_path;
                }
                if local_config.data.cache_path.is_empty() {
                    local_config.data.cache_path = default_config.data.cache_path;
                }
                if local_config.wallhaven.category.is_empty() {
                    local_config.wallhaven.category = default_config.wallhaven.category;
                }
                if local_config.wallhaven.purity.is_empty() {
                    local_config.wallhaven.purity = default_config.wallhaven.purity;
                }
                if local_config.wallhaven.sorting.is_empty() {
                    local_config.wallhaven.sorting = default_config.wallhaven.sorting;
                }
                if local_config.wallhaven.color.is_empty() {
                    local_config.wallhaven.color = default_config.wallhaven.color;
                }
                if local_config.wallhaven.top_range.is_empty() {
                    local_config.wallhaven.top_range = default_config.wallhaven.top_range;
                }
                if local_config.wallhaven.api_key.is_empty() {
                    local_config.wallhaven.api_key = default_config.wallhaven.api_key;
                }
                // 验证壁纸模式，如果无效则使用默认值
                if WallpaperMode::from_str(local_config.wallpaper.mode.as_str()).is_none() {
                    local_config.wallpaper.mode = default_config.wallpaper.mode;
                }
                // 验证定时切换模式，如果无效则使用默认值
                if WallpaperAutoChangeMode::from_str(local_config.wallpaper.auto_change_mode.as_str()).is_none() {
                    local_config.wallpaper.auto_change_mode = default_config.wallpaper.auto_change_mode;
                }
                // 验证定时切换周期，如果无效则使用默认值
                if WallpaperAutoChangeInterval::from_str(&local_config.wallpaper.auto_change_interval.as_str()).is_none() {
                    local_config.wallpaper.auto_change_interval = default_config.wallpaper.auto_change_interval;
                }
                local_config.save_to_file();
                return local_config;
            }
        }

        default_config.save_to_file();
        default_config
    }

    fn default_with_lang(lang: &str) -> Self {
        Config {
            global: GlobalConfig {
                language: lang.to_string(),
                auto_startup: false,
                close_action: CloseAction::Ask,
                proxy: String::new(),
            },
            data: DataConfig {
                data_path: DEFAULT_DATA_PATH.to_string(),
                cache_path: DEFAULT_CACHE_PATH.to_string(),
            },
            display: DisplayConfig {
                width: DEFAULT_WINDOW_WIDTH,
                height: DEFAULT_WINDOW_HEIGHT,
            },
            wallhaven: WallhavenConfig {
                category: "general".to_string(),
                purity: "sfw".to_string(),
                sorting: "date_added".to_string(),
                color: "any".to_string(),
                top_range: "1M".to_string(),
                api_key: String::new(),
                resolution_mode: "all".to_string(),
                atleast_resolution: String::new(),
                resolutions: String::new(),
                ratios: String::new(),
            },
            wallpaper: WallpaperConfig {
                mode: WallpaperMode::Stretch,
                auto_change_mode: WallpaperAutoChangeMode::Local,
                auto_change_interval: WallpaperAutoChangeInterval::Off,
            },
        }
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
                let _ = fs::write(CONFIG_FILE, content);
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
        self.global.language = lang.clone();
        self.save_to_file();
    }

    pub fn set_auto_startup(&mut self, enable: bool) {
        self.global.auto_startup = enable;
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
