// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 隐藏通知（无条件隐藏，保留用于其他场景）
    pub(in crate::ui::main) fn hide_notification(&mut self) -> Task<AppMessage> {
        self.main_state.show_notification = false;
        Task::none()
    }

    /// 隐藏通知（带版本号检查，只有当传入的版本号与当前通知版本号匹配时才隐藏）
    pub(in crate::ui::main) fn hide_notification_with_version(&mut self, version: u64) -> Task<AppMessage> {
        // 只有当传入的版本号与当前通知版本号匹配时才隐藏通知
        // 这样可以防止旧版本的隐藏任务关闭新显示的通知
        if self.main_state.notification_version == version {
            self.main_state.show_notification = false;
        }
        Task::none()
    }
}
