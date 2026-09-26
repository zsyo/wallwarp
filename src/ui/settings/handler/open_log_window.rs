// Copyright (C) 2026 zsyo - GNU GPL v3.0

use crate::ui::log_window::LogWindowMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 打开日志展示窗口（已打开则聚焦）
    pub(in crate::ui::settings) fn settings_open_log_window(&mut self) -> Task<AppMessage> {
        self.handle_log_window_message(LogWindowMessage::Open)
    }
}
