// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::wallhaven;
use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_wallhaven_api_key_changed(
        &mut self,
        api_key: String,
    ) -> Task<AppMessage> {
        self.settings_state.wallhaven_api_key = api_key;
        // 输入过程中保持显示状态，避免输入内容被脱敏展示替换
        self.settings_state.wallhaven_api_key_visible = true;
        Task::none()
    }

    /// 切换 API Key 输入框内容的显示/隐藏状态（仅内存状态，重启后恢复隐藏）
    pub(in crate::ui::settings) fn settings_toggle_wallhaven_api_key_visible(
        &mut self,
    ) -> Task<AppMessage> {
        self.settings_state.wallhaven_api_key_visible =
            !self.settings_state.wallhaven_api_key_visible;
        // 切回隐藏时同步刷新脱敏显示串
        self.settings_state.refresh_wallhaven_api_key_masked();
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
            } else {
                wallhaven::mask_api_key(key)
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

        // 保存后内容非空时自动切换为隐藏状态；内容为空则维持显示以便后续输入
        if !self.settings_state.wallhaven_api_key.is_empty() {
            self.settings_state.wallhaven_api_key_visible = false;
            self.settings_state.refresh_wallhaven_api_key_masked();
        }

        // 显示成功通知
        self.show_notification(
            "WallHeven API KEY 保存成功".to_string(),
            NotificationType::Success,
        )
    }
}
