// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 应用配置（config.toml）
//!
//! 结构按职责拆分：[`sections`] 各分节结构体、[`enums`] 配置枚举、
//! [`io`] 配置文件读写与修复；本文件为顶层 `Config` 聚合与访问方法

mod enums;
mod io;
mod sections;

pub use enums::*;
pub use sections::*;

use serde::{Deserialize, Serialize};
use std::fs;
use tracing::error;

use crate::utils::helpers;

const DEFAULT_DATA_PATH: &str = "data";
pub const MIN_WINDOW_WIDTH: u32 = 1280;
pub const MIN_WINDOW_HEIGHT: u32 = 800;

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
    pub pixabay: PixabayConfig,
    #[serde(default)]
    pub unsplash: UnsplashConfig,
    #[serde(default)]
    pub wallpaper: WallpaperConfig,
}

impl Config {
    /// 从配置文件加载配置（损坏备份后回退默认；语言不在可用列表时重置）
    pub fn new(lang: &str, available_langs: &[String]) -> Self {
        let config_path = helpers::config_file_path();

        // Linux：旧版本配置存于数据根（工作目录下 config.toml），目录规则
        // 调整后另置 ~/.config/wallwarp/。新路径无配置而旧路径存在时复制
        // 迁移，避免升级丢失用户配置（旧文件保留不删）
        #[cfg(target_os = "linux")]
        Self::migrate_legacy_config(&config_path);

        if let Ok(content) = fs::read_to_string(&config_path) {
            match toml::from_str::<Config>(&content) {
                Ok(mut local_config) => {
                    // 仅在修复逻辑实际修改过内容时才回写，避免抹掉用户手动添加的注释
                    let mut modified = local_config.fix_config();
                    if !available_langs.contains(&local_config.global.language) {
                        local_config.global.language = lang.to_string();
                        modified = true;
                    }
                    if modified {
                        local_config.save_to_file();
                    }
                    return local_config;
                }
                Err(e) => {
                    // 配置文件损坏：备份留证后以默认配置启动，不再直接终止程序
                    error!(
                        "[Config] 配置文件解析失败: {}，将备份为 {}.bak 并使用默认配置",
                        e,
                        config_path.display()
                    );
                    if let Err(rename_err) =
                        fs::rename(&config_path, format!("{}.bak", config_path.display()))
                    {
                        error!(
                            "[Config] 备份损坏的配置文件失败: {}，原因: {}",
                            config_path.display(),
                            rename_err
                        );
                    }
                    let default_config = Config::with_language(lang);
                    default_config.save_to_file();
                    return default_config;
                }
            }
        }

        // 配置文件不存在，创建默认配置
        let default_config = Config::with_language(lang);
        default_config.save_to_file();
        default_config
    }

    /// 以指定语言构造默认配置
    fn with_language(lang: &str) -> Self {
        Config {
            global: GlobalConfig {
                language: lang.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// 当前生效的代理地址（未启用代理或地址为空时返回 None）
    pub fn resolved_proxy(&self) -> Option<String> {
        if self.global.proxy_enabled && !self.global.proxy.is_empty() {
            Some(self.global.proxy.clone())
        } else {
            None
        }
    }

    // 以下各 setter 只更新内存，磁盘写入统一由 App 层的
    // request_config_save()（300ms 防抖）合并执行，调用方负责触发

    pub fn update_window_size(&mut self, width: u32, height: u32) {
        self.display.width = width;
        self.display.height = height;
    }

    pub fn set_language(&mut self, lang: String) {
        self.global.language = lang;
    }

    pub fn set_close_action(&mut self, action: CloseAction) {
        self.global.close_action = action;
    }

    pub fn set_data_path(&mut self, path: String) {
        self.data.data_path = path;
    }

    pub fn set_cache_path(&mut self, path: String) {
        self.data.cache_path = path;
    }

    pub fn set_wallhaven_api_key(&mut self, key: String) {
        self.wallhaven.api_key = key;
    }

    pub fn set_pixabay_api_key(&mut self, key: String) {
        self.pixabay.api_key = key;
    }

    /// 设置日志展示窗口等级（只影响展示窗口，不动文件/控制台日志配置）
    pub fn set_log_window_level(&mut self, level: LogLevel) {
        self.global.log_window_level = level;
    }

    /// 设置日志展示窗口最大保留行数（钳制到 50-10000，UI 提交前另有区间校验）
    pub fn set_log_window_max_lines(&mut self, max_lines: u32) {
        self.global.log_window_max_lines =
            max_lines.clamp(LOG_WINDOW_MAX_LINES_MIN, LOG_WINDOW_MAX_LINES_MAX);
    }

    pub fn set_unsplash_api_key(&mut self, key: String) {
        self.unsplash.api_key = key;
    }

    pub fn set_proxy(&mut self, proxy: String) {
        self.global.proxy = proxy;
    }

    pub fn set_wallpaper_mode(&mut self, mode: WallpaperMode) {
        self.wallpaper.mode = mode;
    }
}
