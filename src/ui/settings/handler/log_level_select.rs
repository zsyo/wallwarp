// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::config::LogLevel;
use crate::utils::logger;
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_log_level_selected(
        &mut self,
        level: LogLevel,
    ) -> Task<AppMessage> {
        let old_level = self.config.global.log_level;
        info!(
            "[设置] [日志等级] 修改: {} -> {}",
            old_level.as_str(),
            level.as_str()
        );
        self.config.global.log_level = level;
        self.config.save_to_file();

        // 实时生效：同步刷新控制台层与文件层
        logger::update_log_config(self.config.global.enable_logging, level);

        // 自动收起选择器
        self.settings_state.log_level_picker_expanded = false;

        Task::none()
    }
}
