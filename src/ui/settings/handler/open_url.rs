// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::error;

impl App {
    pub(in crate::ui::settings) fn settings_open_url(&mut self, url: String) -> Task<AppMessage> {
        if let Err(e) = open::that(&url) {
            error!("打开链接失败 {}: {}", url, e);
        }
        Task::none()
    }
}
