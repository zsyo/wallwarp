// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    pub(in crate::ui::settings) fn settings_show_path_clear_confirm(&mut self, path_type: String) -> Task<AppMessage> {
        // 显示路径清空确认对话框
        self.show_path_clear_confirmation = true;
        self.path_to_clear = path_type;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_confirm_path_clear(&mut self, path_type: String) -> Task<AppMessage> {
        // 隐藏确认对话框
        self.show_path_clear_confirmation = false;

        // 执行清空操作
        let path_to_clear = match path_type.as_str() {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => return Task::none(),
        };
        // 获取绝对路径
        let full_path = helpers::get_absolute_path(path_to_clear);

        // 尝试清空目录内容
        let result = if let Ok(entries) = std::fs::read_dir(&full_path) {
            let mut success_count = 0;
            let mut error_count = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let result = if path.is_file() {
                        std::fs::remove_file(&path)
                    } else if path.is_dir() {
                        std::fs::remove_dir_all(&path)
                    } else {
                        Ok(())
                    };

                    if result.is_ok() {
                        success_count += 1;
                    } else {
                        error_count += 1;
                    }
                }
            }

            if error_count == 0 {
                Ok(success_count)
            } else {
                Err(error_count)
            }
        } else {
            Err(0) // 目录不存在或无法访问
        };

        match result {
            Ok(count) => {
                // 清空成功，显示成功通知
                let message = if path_type == "data" {
                    format!("数据路径清空成功，删除了{}个项目", count)
                } else {
                    format!("缓存路径清空成功，删除了{}个项目", count)
                };
                return self.show_notification(message, NotificationType::Success);
            }
            Err(error_count) => {
                // 清空失败，显示错误通知
                let message = if path_type == "data" {
                    format!("数据路径清空失败，{}个项目未删除", error_count)
                } else {
                    format!("缓存路径清空失败，{}个项目未删除", error_count)
                };
                return self.show_notification(message, NotificationType::Error);
            }
        }
    }

    pub(in crate::ui::settings) fn settings_cancel_path_clear(&mut self) -> Task<AppMessage> {
        // 隐藏确认对话框，不执行清空操作
        self.show_path_clear_confirmation = false;
        Task::none()
    }
}
