// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage, CloseAction, CloseConfirmationAction};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn show_close_confirm(&mut self) -> Task<AppMessage> {
        self.show_close_confirmation = true;
        Task::none()
    }

    pub(in crate::ui::main) fn close_confirm_response(
        &mut self,
        action: CloseConfirmationAction,
        remember_setting: bool,
    ) -> Task<AppMessage> {
        // 隐藏对话框
        self.show_close_confirmation = false;

        // 如果勾选了记住设置，则更新配置
        if remember_setting {
            let new_close_action = match action {
                CloseConfirmationAction::MinimizeToTray => CloseAction::MinimizeToTray,
                CloseConfirmationAction::CloseApp => CloseAction::CloseApp,
            };
            self.config.set_close_action(new_close_action);
        }

        // 根据选择执行相应操作
        match action {
            CloseConfirmationAction::MinimizeToTray => Task::done(MainMessage::MinimizeToTray.into()),
            CloseConfirmationAction::CloseApp => iced::exit(),
        }
    }

    pub(in crate::ui::main) fn close_confirm_cancelled(&mut self) -> Task<AppMessage> {
        // 隐藏对话框，不执行任何操作
        self.show_close_confirmation = false;
        Task::none()
    }

    pub(in crate::ui::main) fn toggle_remember_setting(&mut self, checked: bool) -> Task<AppMessage> {
        self.settings_state.remember_close_setting = checked;
        Task::none()
    }
}
