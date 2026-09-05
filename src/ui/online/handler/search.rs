// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::online) fn online_search(&mut self) -> Task<AppMessage> {
        // 搜索：重置到第一页并重新加载
        self.online_state.current_page = 1;

        // 取消所有等待中的下载任务并清理半成品文件
        self.cancel_pending_tasks_and_cleanup();

        // 滚动到顶部，避免触发自动加载下一页
        let scroll_to_top_task =
            Task::done(MainMessage::ScrollToTop("online_wallpapers".to_string()).into());

        // 执行搜索和滚动到顶部
        Task::batch([self.load_online_wallpapers(), scroll_to_top_task])
    }
}
