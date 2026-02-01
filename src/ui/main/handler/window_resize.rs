// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::{Task, window};

impl App {
    pub(in crate::ui::main) fn window_resized(&mut self, width: u32, height: u32) -> Task<AppMessage> {
        // 更新当前窗口宽度和高度，用于响应式布局和判断是否需要自动加载下一页
        self.main_state.current_window_width = width;
        self.main_state.current_window_height = height;
        // 如果宽度和高度都为 0，通常意味着窗口被最小化了
        self.main_state.is_visible = width > 0 && height > 0;
        // 暂存窗口大小，等待防抖处理
        self.main_state.pending_window_size = Some((width, height));

        // 窗口大小发生变化,查询当前窗口模式
        // 如果是从最大化还原成默认窗口,那么需要将自定义标题最大化/还原按钮重置状态,并启用边框调整大小
        let restore_border_resize = window::oldest().and_then(move |id| {
            window::is_maximized(id).map(move |is_maximized| {
                if !is_maximized {
                    MainMessage::RestoreBorderResize.into()
                } else {
                    AppMessage::None
                }
            })
        });

        // 在收到调整大小事件时，直接开启一个延迟任务
        // 这个 Task 会在 300ms 后发出一条"执行保存"的消息
        self.main_state.debounce_timer = std::time::Instant::now();
        let delay_task = Task::perform(tokio::time::sleep(std::time::Duration::from_millis(300)), |_| {
            MainMessage::ExecutePendingSave.into()
        });

        Task::batch(vec![restore_border_resize, delay_task])
    }

    pub(in crate::ui::main) fn restore_border_resize(&mut self) -> Task<AppMessage> {
        self.main_state.is_maximized = false;
        window::oldest().and_then(move |id| window::maximize(id, false).map(|_: ()| AppMessage::None))
    }
}
