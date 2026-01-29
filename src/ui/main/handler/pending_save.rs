// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 处理延迟保存
    pub(in crate::ui::main) fn execute_pending_save(&mut self) -> Task<AppMessage> {
        let elapsed = self.debounce_timer.elapsed();
        if elapsed >= std::time::Duration::from_millis(300) {
            // 只有当存在 pending 数据时才保存，保存完立即 take() 掉
            if let Some((width, height)) = self.pending_window_size.take() {
                if width > 0 && height > 0 {
                    // 同步窗口大小到配置文件
                    self.config.update_window_size(width, height);
                }
            }
        }
        Task::none()
    }
}
