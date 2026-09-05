// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use tracing::debug;

impl App {
    pub(in crate::ui::online) fn modal_image_downloaded(
        &mut self,
        handle: Handle,
    ) -> Task<AppMessage> {
        // 检查模态窗口是否仍然可见
        if self.online_state.modal_visible {
            // 模态窗口图片下载完成，保存图片数据
            self.online_state.modal_image_handle = Some(handle);
        } else {
            // 模态窗口已关闭，显式释放图片数据
            let _old_handle = handle;
        }
        // 不重置进度字段：大图解码上屏前底层占位符仍可见，
        // 保留最后一次的进度环可避免闪现"图片加载中"文字；
        // 进度字段会在下次打开模态/切换图片时统一清零
        self.online_state.modal_download_cancel_token = None;
        Task::none()
    }

    pub(in crate::ui::online) fn modal_image_download_failed(
        &mut self,
        error: String,
    ) -> Task<AppMessage> {
        // 失败细节由 streaming 层 error 记录，UI 层降为 debug 避免双记
        debug!("[模态窗口图片下载] 下载失败: {}", error);
        // 切换图片场景：这是旧任务被取消的残余消息，新任务已在途，
        // 不重置其进度显示、不清掉新任务的取消令牌
        if error == crate::services::download::DOWNLOAD_CANCELLED
            && self.online_state.modal_download_cancel_token.is_some()
        {
            return Task::none();
        }
        // 重置下载状态
        self.online_state.modal_download_progress = 0.0;
        self.online_state.modal_downloaded_bytes = 0;
        self.online_state.modal_total_bytes = 0;
        self.online_state.modal_download_cancel_token = None;
        Task::none()
    }
}
