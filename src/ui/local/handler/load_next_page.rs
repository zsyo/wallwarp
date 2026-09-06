// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::common::grid_estimated_height;
use crate::ui::local::LocalMessage;
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
            // 如果估算的内容高度小于窗口高度，说明没有滚动条，需要加载下一页
            let estimated_content_height = grid_estimated_height(
                self.local_state.wallpapers.len(),
                self.main_state.current_window_width,
            );
            if estimated_content_height < self.main_state.current_window_height as f32 {
                return Task::done(LocalMessage::LoadPage.into());
            }
        }
        Task::none()
    }
}
