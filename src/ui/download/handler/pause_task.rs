// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn pause_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 记录暂停时的下载进度
        if let Some(index) = self.download_state.find_task_index(id) {
            if let Some(task) = self.download_state.get_task_by_index(index) {
                tracing::info!(
                    "[下载任务] [ID:{}] 暂停：total_size = {} bytes, downloaded_size = {} bytes",
                    id,
                    task.task.total_size,
                    task.task.downloaded_size
                );

                // 注意：不读取缓存文件大小，因为异步写入可能还没刷新到磁盘
                // 直接使用任务中记录的 downloaded_size 即可
            }
        }

        // 然后设置状态为暂停
        self.download_state.update_status(id, DownloadStatus::Paused);

        // 最后设置取消标志，终止下载
        self.download_state.cancel_task(id);

        // 注意：不删除已下载的缓存文件，保留以便断点续传
        Task::none()
    }
}
