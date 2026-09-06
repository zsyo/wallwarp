// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::Path;

impl App {
    pub(in crate::ui::download) fn view_downloaded_file(&mut self, id: usize) -> Task<AppMessage> {
        if let Some(task) = self.download_state.tasks.iter().find(|t| t.task.id == id) {
            let full_path = helpers::get_absolute_path(&task.task.save_path);

            // 检查文件是否存在
            if !Path::new(&full_path).exists() {
                let message = self
                    .i18n
                    .t_with_args("notification.file-deleted", &[("path", full_path.clone())]);
                return self.show_notification(message, NotificationType::Error);
            }

            helpers::open_file_in_explorer(&full_path);
        }
        Task::none()
    }
}
