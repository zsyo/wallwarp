// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::local::LocalMessage;
use crate::ui::style::{IMAGE_HEIGHT, IMAGE_SPACING, IMAGE_WIDTH};
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 检查是否需要自动加载下一页
    pub(in crate::ui::local) fn load_local_next_page(&mut self) -> Task<AppMessage> {
        // 检查是否需要自动加载下一页
        // 条件：还有更多壁纸，且当前没有正在加载
        if self.local_state.current_page * self.local_state.page_size < self.local_state.total_count
            && !self.local_state.loading_page
        {
            // 计算每行可以显示多少张图
            let available_width = (self.current_window_width as f32 - IMAGE_SPACING).max(IMAGE_WIDTH);
            let unit_width = IMAGE_WIDTH + IMAGE_SPACING;
            let items_per_row = (available_width / unit_width).floor() as usize;
            let items_per_row = items_per_row.max(1);

            // 计算实际行数
            let num_wallpapers = self.local_state.wallpapers.len();
            let num_rows = (num_wallpapers + items_per_row - 1) / items_per_row; // 向上取整

            // 估算内容高度：行数 * (每张图高度 + 间距)
            let estimated_content_height = num_rows as f32 * (IMAGE_HEIGHT + IMAGE_SPACING);

            // 如果估算的内容高度小于窗口高度，说明没有滚动条，需要加载下一页
            if estimated_content_height < self.current_window_height as f32 {
                return Task::done(LocalMessage::LoadPage.into());
            }
        }
        Task::none()
    }
}
