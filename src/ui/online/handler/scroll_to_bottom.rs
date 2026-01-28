// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_scroll_to_bottom(&mut self) -> Task<AppMessage> {
        // 滚动到底部，加载下一页
        if !self.online_state.last_page && !self.online_state.loading_page {
            self.load_online_page()
        } else {
            Task::none()
        }
    }
}
