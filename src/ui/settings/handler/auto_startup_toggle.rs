// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::startup;
use iced::Task;
use tracing::error;

impl App {
    pub(in crate::ui::settings) fn settings_auto_startup_toggled(&mut self, enabled: bool) -> Task<AppMessage> {
        if let Err(e) = startup::set_auto_startup(enabled) {
            error!("设置开机启动失败: {}", e);
        }
        Task::none()
    }
}
