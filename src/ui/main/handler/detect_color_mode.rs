// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use crate::utils::config::Theme;
use crate::utils::window_utils;
use iced::Task;

impl App {
    pub(in crate::ui::main) fn detect_color_mode(&mut self) -> Task<AppMessage> {
        let system_is_dark = window_utils::get_system_color_mode();
        if system_is_dark != self.theme_config.is_dark() {
            if system_is_dark {
                return self.toggle_theme(Theme::Dark);
            } else {
                return self.toggle_theme(Theme::Light);
            }
        }
        Task::none()
    }
}
