// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, CloseAction};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_close_action_selected(&mut self, action: CloseAction) -> Task<AppMessage> {
        let old_action = self.config.global.close_action.clone();
        info!("[设置] [关闭动作] 修改: {} -> {}", old_action.as_str(), action.as_str());

        self.config.set_close_action(action);
        Task::none()
    }
}
