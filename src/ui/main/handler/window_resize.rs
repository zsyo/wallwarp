// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::main) fn window_resized(&mut self, width: u32, height: u32) -> Task<AppMessage> {
        // 更新当前窗口宽度和高度，用于响应式布局和判断是否需要自动加载下一页
        self.current_window_width = width;
        self.current_window_height = height;
        // 如果宽度和高度都为 0，通常意味着窗口被最小化了
        self.is_visible = width > 0 && height > 0;
        // 暂存窗口大小，等待防抖处理
        self.pending_window_size = Some((width, height));
        // 在收到调整大小事件时，直接开启一个延迟任务
        self.debounce_timer = std::time::Instant::now();
        // 这个 Task 会在 300ms 后发出一条“执行保存”的消息
        return Task::perform(tokio::time::sleep(std::time::Duration::from_millis(300)), |_| {
            MainMessage::ExecutePendingSave.into()
        });
    }
}
