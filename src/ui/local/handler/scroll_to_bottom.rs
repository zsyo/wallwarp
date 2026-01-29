// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::local::LocalMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 滚动到底部并加载下一页
    pub(in crate::ui::local) fn local_scroll_to_bottom(&mut self) -> Task<AppMessage> {
        // 滚动到底部，如果还有更多壁纸则加载下一页
        if self.local_state.current_page * self.local_state.page_size < self.local_state.total_count
            && !self.local_state.loading_page
        {
            return Task::done(LocalMessage::LoadPage.into());
        }
        Task::none()
    }
}
