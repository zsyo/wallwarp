// Copyright (C) 2026 zsyo - GNU AGPL v3.0

//! 历史记录条目辅助操作（定位文件/复制路径）

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;
use std::path::Path;

impl App {
    /// 在文件夹中查看该壁纸
    pub(in crate::ui::history) fn view_history_file(&mut self, index: usize) -> Task<AppMessage> {
        if let Some(entry) = self.history_state.entries.get(index) {
            let full_path = helpers::get_absolute_path(&entry.path);

            if !Path::new(&full_path).exists() {
                return self.show_notification(
                    format!("{}: {}", self.i18n.t("history.file-missing"), full_path),
                    NotificationType::Error,
                );
            }

            helpers::open_file_in_explorer(&full_path);
        }
        Task::none()
    }

    /// 复制文件路径到剪贴板
    pub(in crate::ui::history) fn copy_history_path(&mut self, index: usize) -> Task<AppMessage> {
        if let Some(entry) = self.history_state.entries.get(index) {
            let full_path = helpers::get_absolute_path(&entry.path);
            return self.copy_text_to_clipboard(
                full_path,
                self.i18n.t("history.copy-success"),
                self.i18n.t("history.copy-failed"),
            );
        }
        Task::none()
    }
}
