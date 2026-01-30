// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::{App, AppMessage};
use crate::utils::config::WallpaperAutoChangeMode;
use iced::Task;
use std::time::Instant;

impl App {
    /// 处理定时切换壁纸的定时器事件
    pub(in crate::ui::auto_change) fn auto_change_tick(&mut self) -> Task<AppMessage> {
        if !self.auto_change_state.auto_change_enabled {
            return Task::none();
        }

        // 2. 更新最后一次执行时间（用于 UI 显示或其他逻辑参考）
        self.auto_change_state.auto_change_last_time = Some(Instant::now());

        // 3. 记录日志 (现在只有在真正执行时才会打印)
        let next_interval = self.config.wallpaper.auto_change_interval.get_minutes().unwrap_or(30);
        let next_time_label = chrono::Local::now() + chrono::Duration::minutes(next_interval as i64);
        tracing::info!(
            "[定时切换] 执行壁纸切换。模式: {:?}, 下次预计时间: {}",
            self.config.wallpaper.auto_change_mode,
            next_time_label.format("%H:%M:%S")
        );

        // 4. 根据模式直接执行切换任务
        match self.config.wallpaper.auto_change_mode {
            WallpaperAutoChangeMode::Local => {
                let data_path = self.config.data.data_path.clone();
                Task::perform(
                    async_task::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            if paths.is_empty() {
                                AutoChangeMessage::GetSupportedImagesFailed("没有找到支持的壁纸文件".to_string()).into()
                            } else {
                                AutoChangeMessage::GetSupportedImagesSuccess(paths).into()
                            }
                        }
                        Err(e) => AutoChangeMessage::GetSupportedImagesFailed(e.to_string()).into(),
                    },
                )
            }
            WallpaperAutoChangeMode::Online => {
                let config = self.config.clone();
                let auto_change_running = self.main_state.auto_change_running.clone();
                Task::perform(
                    async_task::async_set_random_online_wallpaper(config, auto_change_running),
                    |result| match result {
                        Ok(path) => AutoChangeMessage::SetRandomWallpaperSuccess(path).into(),
                        Err(e) => AutoChangeMessage::SetRandomWallpaperFailed(e.to_string()).into(),
                    },
                )
            }
        }
    }
}
