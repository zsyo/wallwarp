// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use crate::ui::async_tasks;
use crate::ui::auto_change::AutoChangeMessage;
use crate::utils::config::{WallpaperAutoChangeInterval, WallpaperAutoChangeMode};
use iced::Task;
use std::time::Instant;

impl App {
    /// 启动定时切换壁纸
    pub(in crate::ui::auto_change) fn start(&mut self) -> Task<AppMessage> {
        // 检查定时切换间隔是否为关闭状态
        if matches!(
            self.config.wallpaper.auto_change_interval,
            WallpaperAutoChangeInterval::Off
        ) {
            return Task::none();
        }

        // 启动定时切换
        self.auto_change_state.auto_change_enabled = true;
        self.auto_change_state.auto_change_timer = Some(Instant::now());
        self.auto_change_state.auto_change_last_time = Some(Instant::now());

        // 根据切换模式启动不同的逻辑
        match self.config.wallpaper.auto_change_mode {
            WallpaperAutoChangeMode::Local => {
                // 本地模式：获取支持的图片文件列表
                let data_path = self.config.data.data_path.clone();
                Task::perform(
                    async_tasks::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesSuccess(paths)),
                        Err(e) => AppMessage::AutoChange(AutoChangeMessage::GetSupportedImagesFailed(e.to_string())),
                    },
                )
            }
            WallpaperAutoChangeMode::Online => {
                // 在线模式：启动在线壁纸切换
                Task::none()
            }
        }
    }
}
