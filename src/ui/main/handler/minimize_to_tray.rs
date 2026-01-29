// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    pub(in crate::ui::main) fn minimize_to_tray(&mut self) -> Task<AppMessage> {
        self.is_visible = false;
        // 获取 ID 后设置模式为隐藏
        window::oldest().and_then(|id| window::set_mode(id, window::Mode::Hidden))
    }
}
