// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::helpers;
use iced::Task;

impl App {
    pub(in crate::ui::settings) fn settings_show_path_clear_confirm(
        &mut self,
        path_type: String,
    ) -> Task<AppMessage> {
        // 显示路径清空确认对话框
        self.settings_state.show_path_clear_confirmation = true;
        self.settings_state.path_to_clear = path_type;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_confirm_path_clear(
        &mut self,
        path_type: String,
    ) -> Task<AppMessage> {
        // 隐藏确认对话框
        self.settings_state.show_path_clear_confirmation = false;

        // 执行清空操作
        let path_to_clear = match path_type.as_str() {
            "data" => &self.config.data.data_path,
            "cache" => &self.config.data.cache_path,
            _ => return Task::none(),
        };
        // 获取绝对路径
        let full_path = helpers::get_absolute_path(path_to_clear);

        // 异步清空目录内容（目录可能很大，放阻塞线程避免卡 UI）
        Task::perform(
            async_task::async_clear_directory(full_path),
            move |result| match result {
                Ok(count) => {
                    // 清空成功，显示成功通知
                    let message = if path_type == "data" {
                        format!("数据路径清空成功，删除了{}个项目", count)
                    } else {
                        format!("缓存路径清空成功，删除了{}个项目", count)
                    };
                    MainMessage::ShowNotification(message, NotificationType::Success).into()
                }
                Err(error_count) => {
                    // 清空失败，显示错误通知
                    let message = if path_type == "data" {
                        format!("数据路径清空失败，{}个项目未删除", error_count)
                    } else {
                        format!("缓存路径清空失败，{}个项目未删除", error_count)
                    };
                    MainMessage::ShowNotification(message, NotificationType::Error).into()
                }
            },
        )
    }

    pub(in crate::ui::settings) fn settings_cancel_path_clear(&mut self) -> Task<AppMessage> {
        // 隐藏确认对话框，不执行清空操作
        self.settings_state.show_path_clear_confirmation = false;
        Task::none()
    }
}
