// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::logger;
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_logging_toggled(
        &mut self,
        enabled: bool,
    ) -> Task<AppMessage> {
        let old_value = self.config.global.enable_logging;
        info!("[设置] [运行日志] 修改: {} -> {}", old_value, enabled);
        self.config.global.enable_logging = enabled;
        self.config.save_to_file();

        // 实时生效：挂载/卸载文件输出层
        logger::update_log_config(enabled, self.config.global.log_level);

        Task::none()
    }
}
