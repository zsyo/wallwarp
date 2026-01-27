// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::App;
use crate::ui::AppMessage;
use crate::ui::NotificationType;
use crate::ui::async_tasks;
use iced::Task;
use std::path::Path;

impl App {
    pub(in crate::ui::download) fn set_as_wallpaper(&mut self, id: usize) -> Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let path = task.task.save_path.clone();
            let full_path = crate::ui::common::get_absolute_path(&path);
            let wallpaper_mode = self.config.wallpaper.mode;

            // 检查文件是否存在
            if Path::new(&full_path).exists() {
                // 提前获取翻译文本，避免线程安全问题
                let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

                // 异步设置壁纸
                return Task::perform(
                    async_tasks::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                    move |result| match result {
                        Ok(_) => AppMessage::AddToWallpaperHistory(full_path),
                        Err(e) => {
                            AppMessage::ShowNotification(format!("{}: {}", failed_message, e), NotificationType::Error)
                        }
                    },
                );
            } else {
                let error_message = self.i18n.t("download-tasks.set-wallpaper-file-not-found").to_string();
                return Task::done(AppMessage::ShowNotification(error_message, NotificationType::Error));
            }
        }
        Task::none()
    }
}
