// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::config::WallpaperAutoChangeMode;
use iced::Task;
use tracing::{info, warn};

impl App {
    pub(in crate::ui::main) fn tray_switch_previous_wallpaper(&mut self) -> Task<AppMessage> {
        // 检查历史记录是否为空
        if self.wallpaper_history.is_empty() {
            warn!("[托盘菜单] 壁纸历史记录为空，无法切换上一张");
            return Task::none();
        }

        // 查找上一张壁纸（历史记录中的倒数第二条）
        if self.wallpaper_history.len() < 2 {
            warn!("[托盘菜单] 壁纸历史记录不足2条，无法切换上一张");
            return Task::none();
        }

        let previous_wallpaper = self.wallpaper_history[self.wallpaper_history.len() - 2].clone();

        // 设置壁纸
        let wallpaper_mode = self.config.wallpaper.mode;

        info!("[托盘菜单] 切换上一张壁纸: {}", previous_wallpaper);

        // 提前获取翻译文本，避免线程安全问题
        let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

        iced::Task::perform(
            async_task::async_set_wallpaper(previous_wallpaper.clone(), wallpaper_mode),
            move |result| match result {
                Ok(_) => {
                    // 切换成功，将当前壁纸从历史记录末尾移除
                    MainMessage::RemoveLastFromWallpaperHistory.into()
                }
                Err(e) => {
                    MainMessage::ShowNotification(format!("{}: {}", failed_message, e), NotificationType::Error).into()
                }
            },
        )
    }

    pub(in crate::ui::main) fn tray_switch_next_wallpaper(&mut self) -> Task<AppMessage> {
        // 提前获取翻译文本，避免线程安全问题
        let no_valid_wallpapers_message = self.i18n.t("local-list.no-valid-wallpapers").to_string();

        // 根据定时切换模式执行不同的逻辑
        match self.config.wallpaper.auto_change_mode {
            WallpaperAutoChangeMode::Local => {
                // 本地模式：获取支持的图片文件列表
                let data_path = self.config.data.data_path.clone();
                Task::perform(
                    async_task::async_get_supported_images(data_path),
                    |result| match result {
                        Ok(paths) => {
                            // 获取到图片列表后，立即尝试设置随机壁纸
                            if paths.is_empty() {
                                MainMessage::ShowNotification(no_valid_wallpapers_message, NotificationType::Error)
                                    .into()
                            } else {
                                // 发送一个消息来触发设置随机壁纸
                                AutoChangeMessage::GetSupportedImagesSuccess(paths).into()
                            }
                        }
                        Err(e) => {
                            let error_message = format!("获取壁纸列表失败: {}", e);
                            MainMessage::ShowNotification(error_message, NotificationType::Error).into()
                        }
                    },
                )
            }
            WallpaperAutoChangeMode::Online => {
                // 在线模式：从Wallhaven获取随机壁纸
                let config = self.config.clone();
                let auto_change_running = self.auto_change_state.auto_change_running.clone();
                Task::perform(
                    async_task::async_set_random_online_wallpaper(config, auto_change_running),
                    |result| match result {
                        Ok(path) => AutoChangeMessage::SetRandomWallpaperSuccess(path).into(),
                        Err(e) => {
                            let error_message = format!("设置壁纸失败: {}", e);
                            MainMessage::ShowNotification(error_message, NotificationType::Error).into()
                        }
                    },
                )
            }
        }
    }
}
