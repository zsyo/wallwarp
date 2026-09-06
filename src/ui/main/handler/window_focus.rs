// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn window_focused(&mut self, id: iced::window::Id) -> Task<AppMessage> {
        // 仅主窗口聚焦才视为可见（悬浮球窗口聚焦不算）
        if id == self.main_window_id {
            // 更新窗口状态为已聚焦
            self.main_state.is_visible = true;
        }
        Task::none()
    }
}
