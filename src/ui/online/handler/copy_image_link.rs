// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 复制在线壁纸原图链接到剪贴板
    pub(in crate::ui::online) fn copy_online_image_link(
        &mut self,
        index: usize,
    ) -> Task<AppMessage> {
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let success_message = self.i18n.t("download-tasks.copy-link-success").to_string();
            let failed_message = self.i18n.t("download-tasks.copy-link-failed").to_string();
            return self.copy_text_to_clipboard(url, success_message, failed_message);
        }
        Task::none()
    }
}

