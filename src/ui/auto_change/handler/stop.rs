// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 停止定时切换壁纸
    pub(in crate::ui::auto_change) fn stop_auto_change(&mut self) -> Task<AppMessage> {
        self.auto_change_state.auto_change_enabled = false;
        self.auto_change_state.auto_change_timer = None;
        self.auto_change_state.auto_change_last_time = None;
        Task::none()
    }
}
