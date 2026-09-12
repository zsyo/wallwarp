// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::settings::SettingsMessage;
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

        // 异步清空目录内容（目录可能很大，放阻塞线程避免卡 UI）；
        // 结果回到 handler 格式化通知文案（闭包内无法访问 i18n）
        Task::perform(
            async_task::async_clear_directory(full_path),
            move |result| SettingsMessage::PathClearFinished(path_type, result).into(),
        )
    }

    pub(in crate::ui::settings) fn settings_path_clear_finished(
        &mut self,
        path_type: String,
        result: Result<usize, usize>,
    ) -> Task<AppMessage> {
        // 清空成功后失效相关页面状态，进入页面时重新加载/重建缩略图
        if result.is_ok() {
            if path_type == "cache" {
                self.mark_all_thumbs_stale();
            } else if path_type == "data" {
                // 壁纸库源文件被清空：本地页强制重扫，历史/收藏重载后过滤失效条目
                self.local_state.loaded_data_path = None;
                self.history_state.invalidate();
                self.favorites_state.invalidate();
            }
        }

        let (message, notification_type) = match result {
            Ok(count) => {
                let key = if path_type == "data" {
                    "notification.data-path-clear-success"
                } else {
                    "notification.cache-path-clear-success"
                };
                (
                    self.i18n.t_with_args(key, &[("count", count.to_string())]),
                    NotificationType::Success,
                )
            }
            Err(count) => {
                let key = if path_type == "data" {
                    "notification.data-path-clear-failed"
                } else {
                    "notification.cache-path-clear-failed"
                };
                (
                    self.i18n.t_with_args(key, &[("count", count.to_string())]),
                    NotificationType::Error,
                )
            }
        };
        use crate::ui::main::MainMessage;
        Task::done(MainMessage::ShowNotification(message, notification_type).into())
    }

    pub(in crate::ui::settings) fn settings_cancel_path_clear(&mut self) -> Task<AppMessage> {
        // 隐藏确认对话框，不执行清空操作
        self.settings_state.show_path_clear_confirmation = false;
        Task::none()
    }
}
