// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::AutoChangeState;
use crate::utils::config::{Config, Theme, WallpaperAutoChangeInterval};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tracing::{info, warn};

impl AutoChangeState {
    /// 从配置文件加载定时切换状态
    pub fn load_from_config(config: &Config) -> Self {
        // 根据配置文件中的定时切换周期初始化定时任务状态
        let (auto_change_enabled, next_execute_time) =
            if matches!(config.wallpaper.auto_change_interval, WallpaperAutoChangeInterval::Off) {
                // 配置为关闭状态，不启动定时任务
                info!("[定时切换] [启动] 配置为关闭状态，定时任务未启动");
                (false, None)
            } else {
                // 配置为开启状态，自动启动定时任务
                if let Some(minutes) = config.wallpaper.auto_change_interval.get_minutes() {
                    let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                    // info!(
                    //     "[定时切换] [启动] 配置为开启状态，间隔: {}分钟, 下次执行时间: {}",
                    //     minutes,
                    //     next_time.format("%Y-%m-%d %H:%M:%S")
                    // );
                    (true, Some(next_time))
                } else {
                    warn!("[定时切换] [启动] 配置为开启状态，但间隔时间解析错误, 停止任务");
                    (false, None)
                }
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
