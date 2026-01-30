// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use super::AutoChangeState;
use crate::utils::config::{Config, Theme, WallpaperAutoChangeInterval};
use std::time::Instant;

impl AutoChangeState {
    /// 从配置文件加载定时切换状态
    pub fn load_from_config(config: &Config) -> Self {
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;

        // 根据配置文件中的定时切换周期初始化定时任务状态
        let (auto_change_enabled, auto_change_timer, auto_change_last_time) =
            if matches!(config.wallpaper.auto_change_interval, WallpaperAutoChangeInterval::Off) {
                // 配置为关闭状态，不启动定时任务
                tracing::info!("[定时切换] [启动] 配置为关闭状态，定时任务未启动");
                (false, None, None)
            } else {
                // 配置为开启状态，自动启动定时任务
                let now = Instant::now();
                if let Some(minutes) = config.wallpaper.auto_change_interval.get_minutes() {
                    let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
                    tracing::info!(
                        "[定时切换] [启动] 配置为开启状态，间隔: {}分钟, 下次执行时间: {}",
                        minutes,
                        next_time.format("%Y-%m-%d %H:%M:%S")
                    );
                }
                (true, Some(now), Some(now))
            };

        Self {
            auto_change_enabled,
            auto_change_timer,
            auto_change_last_time,
            auto_detect_color_mode: config.global.theme == Theme::Auto,
            auto_change_running: Arc::new(AtomicBool::new(false)),
        }
    }
}
