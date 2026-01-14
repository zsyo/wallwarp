use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    pub sorting: String,
    #[serde(default)]
    pub purity: String,
    #[serde(default)]
    pub api_key: String,
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
                if local_config.wallhaven.sorting.is_empty() {
                    local_config.wallhaven.sorting = default_config.wallhaven.sorting;
                }
                if local_config.wallhaven.purity.is_empty() {
                    local_config.wallhaven.purity = default_config.wallhaven.purity;
                }
                if local_config.wallhaven.api_key.is_empty() {
                    local_config.wallhaven.api_key = default_config.wallhaven.api_key;
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
                sorting: "date_added".to_string(),
                purity: "sfw".to_string(),
                api_key: String::new(),
            },
        }
    }

    fn fix_config(&mut self) {
        if self.display.width < MIN_WINDOW_WIDTH || self.display.height < MIN_WINDOW_HEIGHT {
            self.display.width = MIN_WINDOW_WIDTH;
            self.display.height = MIN_WINDOW_HEIGHT;
        }
        self.save_to_file();
    }

    pub fn save_to_file(&self) {
        match toml::to_string_pretty(self) {
            Ok(content) => {
                let _ = fs::write(CONFIG_FILE, content);
            }
            Err(e) => eprintln!("TOML 序化失败: {}", e),
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
}
