// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::AutoChangeState;
use crate::utils::config::{Config, Theme};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tracing::{info, warn};

impl AutoChangeState {
    /// 从配置文件加载定时切换状态
    ///
    /// 启用状态由配置的持久化开关字段决定（旧配置经 Config::fix_config 迁移）
    pub fn load_from_config(config: &Config) -> Self {
        let mut auto_change_enabled = config.wallpaper.auto_change_enabled;
        let next_execute_time = if auto_change_enabled {
            match config.wallpaper.auto_change_interval.get_minutes() {
                Some(minutes) => {
                    let next_time =
                        chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                    Some(next_time)
                }
                None => {
                    warn!("[定时切换] [启动] 配置为启用状态，但间隔时间解析错误, 停止任务");
                    auto_change_enabled = false;
                    None
                }
            }
        } else {
            info!("[定时切换] [启动] 配置为停用状态，定时任务未启动");
            None
        };

        Self {
            auto_change_enabled,
            next_execute_time,
            last_executed_time: None,
            auto_detect_color_mode: config.global.theme == Theme::Auto,
            auto_change_running: Arc::new(AtomicBool::new(false)),
        }
    }
}
