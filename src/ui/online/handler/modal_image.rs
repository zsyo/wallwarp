// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::online::OnlineMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use iced::widget::image::Handle;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

impl App {
    pub(in crate::ui::online) fn show_online_modal(&mut self, index: usize) -> Task<AppMessage> {
        // 显示模态窗口
        self.online_state.current_image_index = index;
        self.online_state.modal_visible = true;

        // 显式释放旧的图片数据: 先将 Handle 移出,然后让新值覆盖
        let _old_handle = std::mem::replace(&mut self.online_state.modal_image_handle, None);

        // 重置下载状态
        self.online_state.modal_download_progress = 0.0;
        self.online_state.modal_downloaded_bytes = 0;
        self.online_state.modal_total_bytes = 0;

        // 异步加载图片数据（流式下载）
        if let Some(wallpaper) = self.online_state.wallpapers_data.get(index) {
            let url = wallpaper.path.clone();
            let file_size = wallpaper.file_size;
            let cache_path = self.config.data.cache_path.clone();
            let proxy = if self.config.global.proxy_enabled && !self.config.global.proxy.is_empty() {
                Some(self.config.global.proxy.clone())
            } else {
                None
            };

            // 创建取消令牌
            let cancel_token = Arc::new(AtomicBool::new(false));
            self.online_state.modal_download_cancel_token = Some(cancel_token.clone());

            // 启动下载任务
            return Task::perform(
                async_task::async_load_online_wallpaper_image_with_streaming(
                    url,
                    file_size,
                    cache_path,
                    proxy,
                    cancel_token.clone(),
                ),
                |result| match result {
                    Ok(handle) => OnlineMessage::ModalImageDownloaded(handle).into(),
                    Err(e) => OnlineMessage::ModalImageDownloadFailed(e.to_string()).into(),
                },
            );
        }

        Task::none()
    }

    pub(in crate::ui::online) fn online_modal_image_loaded(&mut self, handle: Handle) -> Task<AppMessage> {
        // 模态窗口图片加载完成，保存图片数据
        self.online_state.modal_image_handle = Some(handle);
        Task::none()
    }

    pub(in crate::ui::online) fn close_online_modal(&mut self) -> Task<AppMessage> {
        // 关闭模态窗口
        self.online_state.modal_visible = false;

        // 显式释放图片数据: 先将 Handle 移出,然后让新值覆盖
        let _old_handle = std::mem::replace(&mut self.online_state.modal_image_handle, None);

        // 取消当前下载
        self.online_state.cancel_modal_download();
        Task::none()
    }
}
