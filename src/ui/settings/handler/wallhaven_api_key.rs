// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_wallhaven_api_key_changed(&mut self, api_key: String) -> Task<AppMessage> {
        self.settings_state.wallhaven_api_key = api_key;
        Task::none()
    }

    pub(in crate::ui::settings) fn settings_save_wallhaven_api_key(&mut self) -> Task<AppMessage> {
        // 保存API KEY到配置文件
        let old_api_key = self.config.wallhaven.api_key.clone();
        let new_api_key = self.settings_state.wallhaven_api_key.clone();

        // 对 API key 进行脱敏处理
        let mask_key = |key: &str| -> String {
            if key.is_empty() {
                "(空)".to_string()
            } else if key.len() >= 8 {
                format!("{}****{}", &key[..4], &key[key.len() - 4..])
            } else {
                "****".to_string()
            }
        };

        info!(
            "[设置] [Wallhaven API Key] 保存: {} -> {}",
            mask_key(&old_api_key),
            mask_key(&new_api_key)
        );
        self.config.set_wallhaven_api_key(new_api_key);

        // 如果 API Key 被清空，移除 NSFW 选项
        if self.settings_state.wallhaven_api_key.is_empty() {
            // 移除 NSFW 位（第0位）
            self.online_state.purities &= !wallhaven::Purity::NSFW.bit_value();
            // 保存到配置文件
            self.online_state.save_to_config(&mut self.config);
        }

        // 显示成功通知
        self.show_notification("WallHeven API KEY 保存成功".to_string(), NotificationType::Success)
    }
}
