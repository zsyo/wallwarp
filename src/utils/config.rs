use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = "config.toml"; // 建议改为 .toml 后缀

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub global: GlobalConfig,
    pub data: DataConfig,
    pub api: ApiConfig,
    pub display: DisplayConfig,
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

#[derive(Clone, Serialize, Deserialize, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")] // 自动处理枚举与字符串的转换，如 "minimize_to_tray"
pub enum CloseAction {
    Ask,            // 每次询问
    MinimizeToTray, // 最小化到托盘
    CloseApp,       // 关闭程序
}

impl Config {
    /// 初始化配置：尝试读取，失败则创建默认
    pub fn new(lang: &str) -> Self {
        let config_path = Path::new(CONFIG_FILE);

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(mut config) = toml::from_str::<Config>(&content) {
                config.fix_config();
                return config;
            }
        }

        // 如果文件不存在或解析失败，返回默认值并保存
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
                data_path: "data".to_string(),
                cache_path: "cache".to_string(),
            },
            api: ApiConfig {
                wallhaven_api_key: String::new(),
            },
            display: DisplayConfig {
                width: 1200,
                height: 800,
            },
        };

        // 尝试创建默认的数据和缓存目录
        Self::create_default_directories(&default_config);

        default_config
    }

    fn fix_config(&mut self) {
        if self.display.width < 1280 || self.display.height < 800 {
            self.display.width = 1280;
            self.display.height = 800;
        }
        self.save_to_file();
    }

    /// 核心保存逻辑：一行代码搞定序列化
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

    /// 创建默认的数据和缓存目录
    fn create_default_directories(config: &Config) {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));

        // 创建数据目录
        let data_path = current_dir.join(&config.data.data_path);
        if let Err(e) = std::fs::create_dir_all(&data_path) {
            eprintln!("Failed to create data directory {:?}: {}", data_path, e);
        }

        // 创建缓存目录
        let cache_path = current_dir.join(&config.data.cache_path);
        if let Err(e) = std::fs::create_dir_all(&cache_path) {
            eprintln!("Failed to create cache directory {:?}: {}", cache_path, e);
        }
    }
}
