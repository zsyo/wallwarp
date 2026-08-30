// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::window;

impl App {
    /// 显示主窗口并置前（三平台通用 iced API）
    ///
    /// 悬浮球/托盘唤醒入口：窗口隐藏时重新可见（保留最大化状态）、
    /// 最小化时恢复、最后请求键盘焦点
    pub(in crate::ui::main) fn show_window(&mut self) -> Task<AppMessage> {
        tracing::info!("[显示窗口] 恢复并置前主窗口");
        let main_id = self.main_window_id;
        Task::batch(vec![
            // Hidden → 可见（set_mode 不改变最大化状态）
            window::set_mode(main_id, window::Mode::Windowed),
            // 最小化 → 恢复（保留最大化状态）
            window::minimize(main_id, false),
            // 置前并获得焦点
            window::gain_focus(main_id),
        ])
    }
}
