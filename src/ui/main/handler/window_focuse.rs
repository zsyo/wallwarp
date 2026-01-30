// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn window_focused(&mut self) -> Task<AppMessage> {
        // 更新窗口状态为已聚焦
        self.main_state.is_visible = true;
        Task::none()
    }
}
