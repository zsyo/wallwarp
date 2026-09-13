// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 配置文件的读写与修复（Config 的 IO 相关方法）

use super::{Config, WallpaperAutoChangeInterval, MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use crate::utils::helpers;
use std::fs;
use tracing::error;
// info 仅 Linux 迁移旧布局配置的日志使用
#[cfg(target_os = "linux")]
use tracing::info;

impl Config {
    /// 修复越界的配置项与旧版本字段迁移
    ///
    /// 返回是否实际修改过内容
    pub(super) fn fix_config(&mut self) -> bool {
        let mut modified = false;
        if self.display.width < MIN_WINDOW_WIDTH || self.display.height < MIN_WINDOW_HEIGHT {
            self.display.width = MIN_WINDOW_WIDTH;
            self.display.height = MIN_WINDOW_HEIGHT;
            modified = true;
        }
        // x、y 同时为负是"最小化到托盘"被误持久化的坐标（Windows 移到屏幕外），
        // 按此坐标启动窗口会完全不可见且无法唤出，还原为未设置（居中打开）。
        // 正常位置记忆中 x 允许小幅为负（标题栏保留 120px 在屏内）、y 恒 >= 0，
        // 不会落入本分支
        if self.display.x < 0 && self.display.y < 0 {
            self.display.x = i32::MIN;
            self.display.y = i32::MIN;
            modified = true;
        }
        // 旧版本用"周期 = off"表达停用；迁移到独立开关字段后，
        // "启用 + 无有效周期"只可能是旧配置的缺字段默认值，修正为停用。
        // 新 UI 下启用的前提是周期有效，不会出现该组合
        if matches!(
            self.wallpaper.auto_change_interval,
            WallpaperAutoChangeInterval::Off
        ) && self.wallpaper.auto_change_enabled
        {
            self.wallpaper.auto_change_enabled = false;
            modified = true;
        }
        modified
    }

    /// Linux：迁移旧布局配置到当前配置路径
    ///
    /// 数据根下的 config.toml 为旧版本布局产物；仅在新路径无配置时复制，
    /// 解析与修复交由后续统一流程处理
    #[cfg(target_os = "linux")]
    pub(super) fn migrate_legacy_config(config_path: &std::path::Path) {
        let legacy = std::path::PathBuf::from("config.toml"); // 工作目录 = 数据根
        if !legacy.is_file() || config_path.exists() {
            return;
        }
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::copy(&legacy, config_path).is_ok() {
            info!(
                "[Config] 已迁移旧布局配置: {} -> {}",
                legacy.display(),
                config_path.display()
            );
        }
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

                // 3. 先写临时文件再替换，避免写盘中断留下损坏的配置文件。
                //    父目录可能不存在（Linux 配置按 XDG 另置 ~/.config/wallwarp/，
                //    启动只创建数据根），创建失败由后续写入统一报错
                let config_path = helpers::config_file_path();
                if let Some(parent) = config_path.parent()
                    && !parent.as_os_str().is_empty()
                {
                    let _ = fs::create_dir_all(parent);
                }
                let tmp_file = format!("{}.tmp", config_path.display());
                let write_result = fs::write(&tmp_file, full_content).and_then(|_| {
                    // Windows 上 rename 不允许覆盖已存在的目标，需先移除旧文件；
                    // 移除前先留 .bak，防止 remove 成功而 rename 失败导致配置丢失
                    if fs::metadata(&config_path).is_ok() {
                        let backup = format!("{}.bak", config_path.display());
                        let _ = fs::copy(&config_path, &backup);
                        fs::remove_file(&config_path)?;
                    }
                    fs::rename(&tmp_file, &config_path)
                });
                if let Err(e) = write_result {
                    error!(
                        "[Config] 配置文件写入失败: {}，原因: {}",
                        config_path.display(),
                        e
                    );
                }
            }
            Err(e) => error!("[Config] TOML 序化失败: {}", e),
        }
    }
}
