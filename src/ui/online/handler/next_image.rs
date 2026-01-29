// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::async_task;
use crate::ui::online::OnlineMessage;
use crate::ui::{App, AppMessage};
use iced::Task;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

impl App {
    pub(in crate::ui::online) fn next_online_image(&mut self) -> Task<AppMessage> {
        // 显示下一张图片
        if self.online_state.current_image_index < self.online_state.wallpapers.len().saturating_sub(1) {
            let next_index = self.online_state.current_image_index + 1;
            self.online_state.current_image_index = next_index;

            // 显式释放旧的图片数据: 先将 Handle 移出,然后让新值覆盖
            let _old_handle = std::mem::replace(&mut self.online_state.modal_image_handle, None);

            // 取消当前下载
            self.online_state.cancel_modal_download();

            if let Some(wallpaper) = self.online_state.wallpapers_data.get(next_index) {
                let url = wallpaper.path.clone();
                let file_size = wallpaper.file_size;
                let cache_path = self.config.data.cache_path.clone();
                let proxy = if self.config.global.proxy.is_empty() {
                    None
                } else {
                    Some(self.config.global.proxy.clone())
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
        }

        Task::none()
    }
}
