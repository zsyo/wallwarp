// Copyright (C) 2026 zsyo - GNU AGPL v3.0

use crate::services::download::DownloadService;
use crate::ui::download::DownloadStatus;
use crate::ui::{App, AppMessage};
use iced::Task;

impl App {
    pub(in crate::ui::download) fn cancel_download_task(&mut self, id: usize) -> Task<AppMessage> {
        // 先保存任务信息，因为取消后可能无法访问
        let task_info = self
            .download_state
            .tasks
            .iter()
            .find(|t| t.task.id == id)
            .map(|t| (t.task.url.clone(), t.task.status.clone()));

        // 取消任务
        self.download_state.cancel_task(id);
        // 将任务状态设置为已取消
        self.download_state.update_status(id, DownloadStatus::Cancelled);

        // 清除未完成的下载文件（仅删除 .download 缓存文件）
        if let Some((url, status)) = task_info {
            // 只有在下载中、等待中或暂停时才清除缓存文件
            if status == DownloadStatus::Downloading
                || status == DownloadStatus::Waiting
                || status == DownloadStatus::Paused
            {
                let cache_path = self.config.data.cache_path.clone();

                // 删除缓存文件（cache_path/online中的 .download 文件）
                if let Ok(cache_file_path) = DownloadService::get_online_image_cache_path(&cache_path, &url, 0) {
                    if let Ok(_metadata) = std::fs::metadata(&cache_file_path) {
                        let _ = std::fs::remove_file(&cache_file_path);
                        tracing::info!("[下载任务] [ID:{}] 已删除未完成的缓存文件: {}", id, cache_file_path);
                    }
                }
            }
        }
        Task::none()
    }
}
