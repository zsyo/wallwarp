// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    /// 确认删除壁纸
    pub(in crate::ui::local) fn confirm_local_delete(&mut self, index: usize) -> Task<AppMessage> {
        // 确认删除壁纸
        self.local_state.delete_confirm_visible = false;
        self.local_state.delete_target_index = None;

        // 删除壁纸
        if let Some(path) = self.local_state.all_paths.get(index) {
            let full_path = helpers::get_absolute_path(path);

            // 尝试删除文件
            match std::fs::remove_file(&full_path) {
                Ok(_) => {
                    // 删除成功，从列表中移除
                    self.local_state.all_paths.remove(index);
                    self.local_state.wallpapers.remove(index);
                    self.local_state.total_count -= 1;

                    // 如果删除的是当前显示的图片，关闭模态窗口
                    if self.local_state.modal_visible && self.local_state.current_image_index == index {
                        self.local_state.modal_visible = false;
                    } else if self.local_state.modal_visible && self.local_state.current_image_index > index {
                        // 如果删除的图片在当前显示图片之前，调整索引
                        self.local_state.current_image_index -= 1;
                    }

                    // 显示成功通知
                    return self.show_notification(self.i18n.t("local-list.delete-success"), NotificationType::Success);
                }
                Err(e) => {
                    // 删除失败，显示错误通知
                    return self.show_notification(
                        format!("{}: {}", self.i18n.t("local-list.delete-failed"), e),
                        NotificationType::Error,
                    );
                }
            }
        }
        Task::none()
    }
}
