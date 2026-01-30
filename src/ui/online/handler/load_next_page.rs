// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::style::{IMAGE_HEIGHT, IMAGE_SPACING, IMAGE_WIDTH};
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

            // 计算每行可以显示多少张图
            let available_width = (self.main_state.current_window_width as f32 - IMAGE_SPACING).max(IMAGE_WIDTH);
            let unit_width = IMAGE_WIDTH + IMAGE_SPACING;
            let items_per_row = (available_width / unit_width).floor() as usize;
            let items_per_row = items_per_row.max(1);

            // 计算实际行数
            let num_wallpapers = self.online_state.wallpapers.len();
            let num_rows = (num_wallpapers + items_per_row - 1) / items_per_row;

            // 估算内容高度
            let estimated_content_height = num_rows as f32 * (IMAGE_HEIGHT + IMAGE_SPACING);

            // 如果估算的内容高度小于窗口高度，需要加载下一页
            // 这样可以确保内容足够多，能够显示滚动条
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
