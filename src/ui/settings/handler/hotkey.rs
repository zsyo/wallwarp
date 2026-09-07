// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::hotkey_manager::{self, HotkeyAction};
use iced::keyboard::{Key, key};
use iced::Task;
use tracing::info;

impl App {
    /// 开始录制全局热键（点击设置行的录制按钮）
    pub(in crate::ui::settings) fn settings_hotkey_record_started(
        &mut self,
        action: HotkeyAction,
    ) -> Task<AppMessage> {
        info!("[设置] [全局热键] [{:?}] 开始录制", action);
        self.settings_state.hotkey_recording = Some(action);
        Task::none()
    }

    /// 取消热键录制
    pub(in crate::ui::settings) fn settings_hotkey_record_cancelled(&mut self) -> Task<AppMessage> {
        info!("[设置] [全局热键] 取消录制");
        self.settings_state.hotkey_recording = None;
        Task::none()
    }

    /// 清除动作的全局热键
    pub(in crate::ui::settings) fn settings_hotkey_cleared(
        &mut self,
        action: HotkeyAction,
    ) -> Task<AppMessage> {
        self.settings_state.hotkey_recording = None;
        self.clear_hotkey(action)
    }

    /// 热键录制中的按键捕获（由 KeyEvent 分发进入）
    ///
    /// - Esc：取消录制
    /// - 纯修饰键：继续等待主键
    /// - 其余按键：校验（须含修饰键，F1-F12 除外）后注册并持久化
    pub(in crate::ui) fn hotkey_record_capture(
        &mut self,
        key: Key,
        modifiers: iced::keyboard::Modifiers,
    ) -> Task<AppMessage> {
        let Some(action) = self.settings_state.hotkey_recording else {
            return Task::none();
        };

        // Esc 取消录制（无论是否带修饰键）
        if matches!(key, Key::Named(key::Named::Escape)) {
            return self.settings_hotkey_record_cancelled();
        }

        // 纯修饰键按下：等待主键
        if hotkey_manager::is_modifier_key(&key) {
            return Task::none();
        }

        // 校验组合有效性：必须含至少一个修饰键（F1-F12 可单独使用）
        if !hotkey_manager::hotkey_modifiers_allowed(modifiers, &key) {
            let message = self.i18n.t("settings.hotkey-need-modifier").to_string();
            self.settings_state.hotkey_recording = None;
            return self.show_notification(message, crate::ui::NotificationType::Error);
        }

        let Some(hotkey_str) = hotkey_manager::iced_key_to_hotkey_string(modifiers, &key) else {
            let message = self.i18n.t("settings.hotkey-unsupported-key").to_string();
            self.settings_state.hotkey_recording = None;
            return self.show_notification(message, crate::ui::NotificationType::Error);
        };

        info!("[设置] [全局热键] [{:?}] 捕获: {}", action, hotkey_str);
        self.settings_state.hotkey_recording = None;
        self.apply_hotkey(action, hotkey_str)
    }
}
