// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use crate::utils::config::CloseAction;
use iced::Task;

impl App {
    pub(in crate::ui::main) fn window_close_requested(&mut self) -> Task<AppMessage> {
        // 根据配置处理关闭请求
        match self.config.global.close_action {
            CloseAction::MinimizeToTray => Task::done(MainMessage::MinimizeToTray.into()),
            CloseAction::CloseApp => iced::exit(),
            CloseAction::Ask => Task::done(MainMessage::ShowCloseConfirmation.into()),
        }
    }
}
