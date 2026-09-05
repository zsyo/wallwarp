// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::main::MainMessage;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 处理窗口尺寸的延迟保存
    pub(in crate::ui::main) fn execute_pending_save(&mut self) -> Task<AppMessage> {
        let elapsed = self.main_state.debounce_timer.elapsed();
        if elapsed >= std::time::Duration::from_millis(300) {
            // 只有当存在 pending 数据时才保存，保存完立即 take() 掉
            if let Some((width, height)) = self.main_state.pending_window_size.take()
                && width >= crate::utils::config::MIN_WINDOW_WIDTH
                && height >= crate::utils::config::MIN_WINDOW_HEIGHT
            {
                // 同步窗口大小到配置文件
                self.config.update_window_size(width, height);
            }
        }
        Task::none()
    }

    /// 请求延迟保存配置文件（300ms 防抖）
    ///
    /// 用于高频触发的配置变更（如在线筛选的连续点击）：
    /// 内存中的 config 立即更新，磁盘写入合并为最后一次
    pub(in crate::ui) fn request_config_save(&mut self) -> Task<AppMessage> {
        self.config_save_dirty = true;
        self.config_save_debounce_timer = std::time::Instant::now();
        Task::perform(
            tokio::time::sleep(std::time::Duration::from_millis(300)),
            |_| MainMessage::ExecutePendingConfigSave.into(),
        )
    }

    /// 防抖到期：把配置写入磁盘
    ///
    /// 到期阈值(200ms)小于防抖间隔(300ms)，保证最后一次变更必定落盘
    pub(in crate::ui) fn execute_pending_config_save(&mut self) -> Task<AppMessage> {
        if self.config_save_dirty
            && self.config_save_debounce_timer.elapsed() >= std::time::Duration::from_millis(200)
        {
            self.config_save_dirty = false;
            self.config.save_to_file();
        }
        Task::none()
    }
}
