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
    pub global: GlobalConfig,
    pub data: DataConfig,
    pub api: ApiConfig,
    pub display: DisplayConfig,
    pub wallhaven: WallhavenConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub language: String,
    pub auto_startup: bool,
    pub close_action: CloseAction,
    pub proxy: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataConfig {
    pub data_path: String,
    pub cache_path: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub wallhaven_api_key: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WallhavenConfig {
    pub category: String,
    pub sorting: String,
    pub purity: String,
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CloseAction {
    Ask,
    MinimizeToTray,
    CloseApp,
}

impl Config {
    pub fn new(lang: &str) -> Self {
        let config_path = Path::new(CONFIG_FILE);

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(mut config) = toml::from_str::<Config>(&content) {
                config.fix_config();
                return config;
            }
        }

        let default_config = Self::default_with_lang(lang);
        default_config.save_to_file();
        default_config
    }

    fn default_with_lang(lang: &str) -> Self {
        let default_config = Config {
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
            api: ApiConfig {
                wallhaven_api_key: String::new(),
            },
            display: DisplayConfig {
                width: DEFAULT_WINDOW_WIDTH,
                height: DEFAULT_WINDOW_HEIGHT,
            },
            wallhaven: WallhavenConfig {
                category: "general".to_string(),
                sorting: "date_added".to_string(),
                purity: "sfw".to_string(),
            },
        };

        Self::create_default_directories(&default_config);

        default_config
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
        self.api.wallhaven_api_key = key;
        self.save_to_file();
    }

    pub fn set_proxy(&mut self, proxy: String) {
        self.global.proxy = proxy;
        self.save_to_file();
    }

    fn create_default_directories(config: &Config) {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

        let data_path = current_dir.join(&config.data.data_path);
        if let Err(e) = std::fs::create_dir_all(&data_path) {
            eprintln!("Failed to create data directory {:?}: {}", data_path, e);
        }

        let cache_path = current_dir.join(&config.data.cache_path);
        if let Err(e) = std::fs::create_dir_all(&cache_path) {
            eprintln!("Failed to create cache directory {:?}: {}", cache_path, e);
        }
    }
}
