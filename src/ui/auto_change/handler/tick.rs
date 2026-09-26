// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::services::async_task;
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::{App, AppMessage};
use crate::utils::config::WallpaperAutoChangeMode;
use iced::Task;
use tracing::{info, warn};

impl App {
    /// 解析当前本地模式选图目录（定时切换与托盘"下一张"共用）
    ///
    /// 开启"按主题分组"后按系统深浅色取对应目录；目录未配置时回退壁纸库目录
    pub(in crate::ui) fn theme_aware_local_wallpaper_dir(&self) -> String {
        let wallpaper = &self.config.wallpaper;
        if wallpaper.auto_change_theme_aware {
            let dir = if crate::platform::system_color_mode() {
                &wallpaper.auto_change_dark_dir
            } else {
                &wallpaper.auto_change_light_dir
            };
            if !dir.is_empty() {
                return dir.clone();
            }
        }
        self.config.data.data_path.clone()
    }

    /// 处理定时切换壁纸的定时器事件
    pub(in crate::ui::auto_change) fn auto_change_tick(&mut self) -> Task<AppMessage> {
        if !self.auto_change_state.auto_change_enabled {
            return Task::none();
        }

        // 2. 更新最后一次执行时间（用于 UI 显示或其他逻辑参考）
        self.auto_change_state.last_executed_time = Some(chrono::Local::now());

        // 3. 记录日志
        if let Some(minutes) = self.config.wallpaper.auto_change_interval.get_minutes() {
            let next_time = chrono::Local::now() + chrono::Duration::minutes(minutes as i64);
            info!(
                "[定时切换] 执行壁纸切换。模式: {:?}，间隔: {}分钟, 下次执行时间: {}",
                self.config.wallpaper.auto_change_mode,
                minutes,
                next_time.format("%Y-%m-%d %H:%M:%S")
            );
            self.auto_change_state.next_execute_time = Some(next_time);
        } else {
            warn!("[定时切换] [启动] 配置为开启状态，但间隔时间解析错误, 停止后续任务");
            self.auto_change_state.auto_change_enabled = false;
            self.auto_change_state.next_execute_time = None;
        }

        // 4. 根据模式直接执行切换任务
        match self.config.wallpaper.auto_change_mode {
            WallpaperAutoChangeMode::Local => {
                let data_path = self.theme_aware_local_wallpaper_dir();
                Task::perform(
                    async_task::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            if paths.is_empty() {
                                AutoChangeMessage::GetSupportedImagesFailed(
                                    "没有找到支持的壁纸文件".to_string(),
                                )
                                .into()
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
                let auto_change_running = self.auto_change_state.auto_change_running.clone();
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
