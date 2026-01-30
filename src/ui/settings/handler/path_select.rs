// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::settings::SettingsMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_data_path_selected(&mut self, path: String) -> Task<AppMessage> {
        if !path.is_empty() && path != "SELECT_DATA_PATH" {
            // 这是异步任务返回的实际路径
            let old_path = self.config.data.data_path.clone();
            info!("[设置] [数据路径] 修改: {} -> {}", old_path, path);
            self.config.set_data_path(path);
        } else if path == "SELECT_DATA_PATH" {
            // 这是用户点击按钮时的原始消息，触发异步任务
            return Task::perform(async_task::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    SettingsMessage::DataPathSelected(selected_path).into()
                } else {
                    SettingsMessage::DataPathSelected("".to_string()).into() // 用户取消选择
                }
            });
        }

        Task::none()
    }

    pub(in crate::ui::settings) fn settings_cache_path_selected(&mut self, path: String) -> Task<AppMessage> {
        if !path.is_empty() && path != "SELECT_CACHE_PATH" {
            // 这是异步任务返回的实际路径
            let old_path = self.config.data.cache_path.clone();
            info!("[设置] [缓存路径] 修改: {} -> {}", old_path, path);
            self.config.set_cache_path(path);
        } else if path == "SELECT_CACHE_PATH" {
            // 这是用户点击按钮时的原始消息，触发异步任务
            return Task::perform(async_task::select_folder_async(), |selected_path| {
                if !selected_path.is_empty() {
                    SettingsMessage::CachePathSelected(selected_path).into()
                } else {
                    SettingsMessage::CachePathSelected("".to_string()).into() // 用户取消选择
                }
            });
        }

        Task::none()
    }
}
