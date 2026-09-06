// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::grid_estimated_height;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_check_and_load_next_page(&mut self) -> Task<AppMessage> {
        // 检查是否需要自动加载下一页
        if !self.online_state.last_page && !self.online_state.loading_page {
            // 如果没有数据，不执行检查（等待空数据自动加载逻辑处理）
            if self.online_state.wallpapers.is_empty() {
                return Task::none();
            }

            // 如果估算的内容高度小于窗口高度，需要加载下一页
            // 这样可以确保内容足够多，能够显示滚动条
            let estimated_content_height = grid_estimated_height(
                self.online_state.wallpapers.len(),
                self.main_state.current_window_width,
            );
            if estimated_content_height < self.main_state.current_window_height as f32 {
                self.load_online_page()
            } else {
                Task::none()
            }
        } else {
            Task::none()
        }
    }
}
