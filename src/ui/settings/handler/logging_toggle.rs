// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage, NotificationType};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_logging_toggled(&mut self, enabled: bool) -> Task<AppMessage> {
        let old_value = self.config.global.enable_logging;
        info!("[设置] [运行日志] 修改: {} -> {}", old_value, enabled);
        self.config.global.enable_logging = enabled;
        self.config.save_to_file();

        // 发送通知
        let message = if enabled {
            self.i18n.t("settings.logging-notice-enabled")
        } else {
            self.i18n.t("settings.logging-notice-disabled")
        };
        self.show_notification(message, NotificationType::Info)
    }
}
