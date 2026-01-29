// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::auto_change::AutoChangeMessage;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;

impl App {
    /// 处理获取支持的图片文件列表成功
    pub(in crate::ui::auto_change) fn get_supported_images_success(&mut self, paths: Vec<String>) -> Task<AppMessage> {
        if !paths.is_empty() {
            // 记录找到的壁纸数量
            tracing::info!("[定时切换] [获取] 找到 {} 张壁纸", paths.len());

            // 获取成功，立即设置一张随机壁纸
            let wallpaper_mode = self.config.wallpaper.mode;

            Task::perform(
                async_task::async_set_random_wallpaper(paths, wallpaper_mode),
                |result| match result {
                    Ok(path) => AutoChangeMessage::SetRandomWallpaperSuccess(path).into(),
                    Err(e) => AutoChangeMessage::SetRandomWallpaperFailed(e.to_string()).into(),
                },
            )
        } else {
            // 没有找到支持的壁纸
            tracing::warn!("[定时切换] [获取] 没有找到支持的壁纸文件");
            let error_message = self.i18n.t("local-list.no-valid-wallpapers").to_string();
            self.show_notification(error_message, NotificationType::Error)
        }
    }

    /// 处理获取支持的图片文件列表失败
    pub(in crate::ui::auto_change) fn get_supported_images_failed(&mut self, error: String) -> Task<AppMessage> {
        tracing::error!("[定时切换] [失败] 获取壁纸列表失败: {}", error);
        self.auto_change_state.auto_change_enabled = false;
        let error_message = format!("获取壁纸列表失败: {}", error);
        self.show_notification(error_message, NotificationType::Error)
    }

    /// 处理随机设置壁纸成功
    pub(in crate::ui::auto_change) fn set_random_wallpaper_success(&mut self, path: String) -> Task<AppMessage> {
        tracing::info!("[定时切换] [成功] 已设置壁纸: {}", path);

        // 将壁纸路径添加到历史记录
        Task::done(MainMessage::AddToWallpaperHistory(path).into())
    }

    /// 处理随机设置壁纸失败
    pub(in crate::ui::auto_change) fn set_random_wallpaper_failed(&mut self, error: String) -> Task<AppMessage> {
        tracing::error!("[定时切换] [失败] 设置壁纸失败: {}", error);
        let error_message = format!("设置壁纸失败: {}", error);
        self.show_notification(error_message, NotificationType::Error)
    }
}
