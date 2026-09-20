// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::helpers;
use iced::Task;
use std::path::Path;
use tracing::error;

impl App {
    pub(in crate::ui::settings) fn settings_open_path(
        &mut self,
        path_type: String,
    ) -> Task<AppMessage> {
        let (path_to_open, dir_name) = match path_type.as_str() {
            "data" => (&self.config.data.data_path, "数据目录"),
            "cache" => (&self.config.data.cache_path, "缓存目录"),
            // 主题分组选图目录：未配置时查看当前生效的壁纸库目录
            "auto-change-light" | "auto-change-dark" => {
                let theme_dir = if path_type == "auto-change-light" {
                    &self.config.wallpaper.auto_change_light_dir
                } else {
                    &self.config.wallpaper.auto_change_dark_dir
                };
                if theme_dir.is_empty() {
                    (&self.config.data.data_path, "壁纸库目录")
                } else {
                    (theme_dir, "主题选图目录")
                }
            }
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

    pub(in crate::ui::settings) fn settings_open_log_file(&mut self) -> Task<AppMessage> {
        let full_path = helpers::get_absolute_path("logs");

        // 检查并创建日志目录
        helpers::ensure_directory_exists(&full_path, "日志目录");

        // latest.log 存在时在文件管理器中选中它，否则打开日志目录
        let latest_log = Path::new(&full_path).join("latest.log");
        if latest_log.exists() {
            helpers::open_file_in_explorer(&latest_log.to_string_lossy());
        } else if let Err(e) = open::that(&full_path) {
            error!("[设置] [日志目录] 打开目录失败 {}: {}", full_path, e);
        }

        Task::none()
    }
}
