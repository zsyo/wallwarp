// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 显示删除确认对话框
    pub(in crate::ui::local) fn show_local_delete_confirm(&mut self, index: usize) -> Task<AppMessage> {
        // 显示删除确认对话框
        self.local_state.delete_confirm_visible = true;
        self.local_state.delete_target_index = Some(index);
        Task::none()
    }

    /// 关闭删除确认对话框
    pub(in crate::ui::local) fn close_local_delete_confirm(&mut self) -> Task<AppMessage> {
        // 关闭删除确认对话框
        self.local_state.delete_confirm_visible = false;
        self.local_state.delete_target_index = None;
        Task::none()
    }
}
