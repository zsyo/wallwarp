// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Task;
use tracing::error;

impl App {
    pub(in crate::ui::settings) fn settings_open_path(&mut self, path_type: String) -> Task<AppMessage> {
        let path_to_open = match path_type.as_str() {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => return Task::none(),
        };
        let full_path = helpers::get_absolute_path(path_to_open);

        if let Err(e) = open::that(&full_path) {
            error!("打开目录失败 {}: {}", full_path, e);
        }

        Task::none()
    }

    pub(in crate::ui::settings) fn settings_open_logs_path(&mut self) -> Task<AppMessage> {
        let logs_path = "logs";
        let full_path = helpers::get_absolute_path(logs_path);

        if let Err(e) = open::that(&full_path) {
            error!("打开日志目录失败 {}: {}", full_path, e);
        }

        Task::none()
    }
}
