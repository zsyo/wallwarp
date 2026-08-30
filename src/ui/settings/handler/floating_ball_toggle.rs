// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use tracing::info;

impl App {
    pub(in crate::ui::settings) fn settings_floating_ball_toggled(
        &mut self,
        enabled: bool,
    ) -> Task<AppMessage> {
        let old_value = self.config.global.show_floating_ball;
        info!("[设置] [显示悬浮球] 修改: {} -> {}", old_value, enabled);
        self.config.global.show_floating_ball = enabled;
        self.config.save_to_file();

        if enabled {
            self.open_floating_ball_window()
        } else {
            self.close_floating_ball_window()
        }
    }
}
