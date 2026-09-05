// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use std::path::Path;

impl App {
    pub(in crate::ui::download) fn set_downloaded_as_wallpaper(
        &mut self,
        id: usize,
    ) -> Task<AppMessage> {
        let full_path = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| crate::utils::helpers::get_absolute_path(&t.task.save_path));

        if let Some(full_path) = full_path {
            // 检查文件是否存在
            if Path::new(&full_path).exists() {
                return self.apply_wallpaper(full_path);
            }

            let error_message = self
                .i18n
                .t("download-tasks.set-wallpaper-file-not-found")
                .to_string();
            return self.show_notification(error_message, NotificationType::Error);
        }
        Task::none()
    }
}
