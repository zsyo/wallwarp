// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::Path;

impl App {
    /// 在文件夹中查看壁纸
    pub(in crate::ui::local) fn view_local_file(&mut self, index: usize) -> Task<AppMessage> {
        // 查看文件夹并选中文件
        if let Some(path) = self.local_state.all_paths.get(index) {
            let full_path = helpers::get_absolute_path(path);

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
