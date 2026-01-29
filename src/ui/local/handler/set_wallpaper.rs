// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    /// 设置壁纸
    pub(in crate::ui::local) fn local_set_as_wallpaper(&mut self, index: usize) -> Task<AppMessage> {
        // 设置壁纸
        if let Some(path) = self.local_state.all_paths.get(index).cloned() {
            let full_path = helpers::get_absolute_path(&path);
            let wallpaper_mode = self.config.wallpaper.mode;

            // 提前获取翻译文本，避免线程安全问题
            let failed_message = self.i18n.t("local-list.set-wallpaper-failed").to_string();

            // 异步设置壁纸
            return Task::perform(
                async_task::async_set_wallpaper(full_path.clone(), wallpaper_mode),
                move |result| match result {
                    Ok(_) => MainMessage::AddToWallpaperHistory(full_path).into(),
                    Err(e) => {
                        MainMessage::ShowNotification(format!("{}: {}", failed_message, e), NotificationType::Error)
                            .into()
                    }
                },
            );
        }
        Task::none()
    }
}
