// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::{operation, scrollable};

impl App {
    pub(in crate::ui::main) fn scroll_to_top(&mut self, scrollable_id: String) -> Task<AppMessage> {
        // 滚动到指定滚动组件的顶部
        operation::scroll_by(scrollable_id, scrollable::AbsoluteOffset { x: 0.0, y: 0.0 })
    }
}
