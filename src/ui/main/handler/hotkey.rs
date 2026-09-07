// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use crate::utils::hotkey_manager::{self, HotkeyAction};
use iced::Task;
use tracing::info;

impl App {
    /// 按配置注册全局热键（启动时调用；失败仅告警，不打扰启动流程）
    pub(in crate::ui) fn init_hotkeys_from_config(&mut self) {
        if !crate::platform::supports_global_hotkeys() {
            info!("[全局热键] 当前桌面环境不支持，跳过初始化");
            return;
        }

        let hotkey_strs = [
            (
                HotkeyAction::SwitchNext,
                self.config.global.hotkey_switch_next.clone(),
            ),
            (
                HotkeyAction::SwitchPrevious,
                self.config.global.hotkey_switch_previous.clone(),
            ),
            (
                HotkeyAction::ShowWindow,
                self.config.global.hotkey_show_window.clone(),
            ),
            (
                HotkeyAction::SaveCurrent,
                self.config.global.hotkey_save_current.clone(),
            ),
        ];
        for (action, s) in hotkey_strs {
            if let Some(hotkey) = hotkey_manager::parse_hotkey_string(&s, action)
                && let Some(manager) = self.hotkey_manager.as_mut()
            {
                // 启动阶段注册失败通常是与其他程序冲突，保留其余热键
                if let Err(e) = manager.register(action, hotkey) {
                    tracing::warn!("[全局热键] [{:?}] 启动注册失败: {}", action, e);
                }
            }
        }
    }

    /// 全局热键触发处理：按 id 反查动作并复用托盘菜单同源逻辑
    pub(in crate::ui) fn hotkey_triggered(
        &mut self,
        event: crate::utils::hotkey_manager::HotkeyEvent,
    ) -> Task<AppMessage> {
        let Some(action) = self
            .hotkey_manager
            .as_ref()
            .and_then(|manager| manager.action_of_pressed(event.id, event.state))
        else {
            return Task::none();
        };

        info!("[全局热键] [{:?}] 触发", action);
        match action {
            HotkeyAction::SwitchNext => self.tray_switch_next_wallpaper(),
            HotkeyAction::SwitchPrevious => self.tray_switch_previous_wallpaper(),
            HotkeyAction::ShowWindow => self.show_window(),
            HotkeyAction::SaveCurrent => self.tray_save_current_wallpaper(),
        }
    }

    /// 注册动作的新热键并持久化到配置（设置页录制完成后调用）
    ///
    /// 注册失败时弹通知并保持旧配置不变
    pub(in crate::ui) fn apply_hotkey(
        &mut self,
        action: HotkeyAction,
        hotkey_str: String,
    ) -> Task<AppMessage> {
        let Some(manager) = self.hotkey_manager.as_mut() else {
            return Task::none();
        };

        let Some(hotkey) = hotkey_manager::parse_hotkey_string(&hotkey_str, action) else {
            return Task::none();
        };

        if let Err(e) = manager.register(action, hotkey) {
            tracing::warn!("[全局热键] [{:?}] 注册失败: {}", action, e);
            let message = self.i18n.t("settings.hotkey-register-failed").to_string();
            return self.show_notification(message, NotificationType::Error);
        }

        info!("[全局热键] [{:?}] 更新并持久化: {}", action, hotkey_str);
        let field = match action {
            HotkeyAction::SwitchNext => &mut self.config.global.hotkey_switch_next,
            HotkeyAction::SwitchPrevious => &mut self.config.global.hotkey_switch_previous,
            HotkeyAction::ShowWindow => &mut self.config.global.hotkey_show_window,
            HotkeyAction::SaveCurrent => &mut self.config.global.hotkey_save_current,
        };
        *field = hotkey_str;
        let _ = self.request_config_save();
        Task::none()
    }

    /// 清除动作的热键（注销并清空配置）
    pub(in crate::ui) fn clear_hotkey(&mut self, action: HotkeyAction) -> Task<AppMessage> {
        if let Some(manager) = self.hotkey_manager.as_mut() {
            manager.unregister_action(action);
        }

        info!("[全局热键] [{:?}] 已清除", action);
        let field = match action {
            HotkeyAction::SwitchNext => &mut self.config.global.hotkey_switch_next,
            HotkeyAction::SwitchPrevious => &mut self.config.global.hotkey_switch_previous,
            HotkeyAction::ShowWindow => &mut self.config.global.hotkey_show_window,
            HotkeyAction::SaveCurrent => &mut self.config.global.hotkey_save_current,
        };
        field.clear();
        let _ = self.request_config_save();
        Task::none()
    }
}
