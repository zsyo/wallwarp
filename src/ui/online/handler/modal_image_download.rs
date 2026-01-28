// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use tracing::error;

impl App {
    pub(in crate::ui::online) fn modal_image_downloaded(&mut self, handle: Handle) -> Task<AppMessage> {
        // 检查模态窗口是否仍然可见
        if self.online_state.modal_visible {
            // 模态窗口图片下载完成，保存图片数据
            self.online_state.modal_image_handle = Some(handle);
        } else {
            // 模态窗口已关闭，显式释放图片数据
            let _old_handle = handle;
        }
        // 重置下载状态
        self.online_state.modal_download_progress = 0.0;
        self.online_state.modal_downloaded_bytes = 0;
        self.online_state.modal_total_bytes = 0;
        self.online_state.modal_download_cancel_token = None;
        Task::none()
    }

    pub(in crate::ui::online) fn modal_image_download_failed(&mut self, error: String) -> Task<AppMessage> {
        // 模态窗口图片下载失败
        error!("[模态窗口图片下载] 下载失败: {}", error);
        // 重置下载状态
        self.online_state.modal_download_progress = 0.0;
        self.online_state.modal_downloaded_bytes = 0;
        self.online_state.modal_total_bytes = 0;
        self.online_state.modal_download_cancel_token = None;
        Task::none()
    }
}
