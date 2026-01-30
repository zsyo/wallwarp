// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Task;
use tracing::error;

impl App {
    pub(in crate::ui::settings) fn settings_open_path(&mut self, path_type: String) -> Task<AppMessage> {
        let (path_to_open, dir_name) = match path_type.as_str() {
            "data" => (&self.config.data.data_path, "数据目录"),
            "cache" => (&self.config.data.cache_path, "缓存目录"),
            _ => return Task::none(),
        };

        let full_path = helpers::get_absolute_path(path_to_open);

        // 检查并创建目录
        helpers::ensure_directory_exists(&full_path, dir_name);

        // 打开目录
        if let Err(e) = open::that(&full_path) {
            error!("[设置] [{}] 打开目录失败 {}: {}", dir_name, full_path, e);
        }

        Task::none()
    }

    pub(in crate::ui::settings) fn settings_open_logs_path(&mut self) -> Task<AppMessage> {
        let logs_path = "logs";
        let full_path = helpers::get_absolute_path(logs_path);

        // 检查并创建日志目录
        helpers::ensure_directory_exists(&full_path, "日志目录");

        // 打开日志目录
        if let Err(e) = open::that(&full_path) {
            error!("[设置] [日志目录] 打开目录失败 {}: {}", full_path, e);
        }

        Task::none()
    }
}
