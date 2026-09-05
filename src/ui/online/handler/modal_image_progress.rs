// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    /// 模态窗口图片下载进度更新（服务层每 5% 一条，经广播订阅回传）
    pub(in crate::ui::online) fn modal_image_progress(
        &mut self,
        downloaded: u64,
        total: u64,
    ) -> Task<AppMessage> {
        // 模态窗口已关闭时忽略残余进度消息
        if !self.online_state.modal_visible {
            return Task::none();
        }

        self.online_state.modal_downloaded_bytes = downloaded;
        self.online_state.modal_total_bytes = total;
        if total > 0 {
            self.online_state.modal_download_progress = (downloaded as f32 / total as f32).min(1.0);
        }
        Task::none()
    }
}
